//! Theme-aware rich-text widget.
//!
//! Build inline runs of differently-styled text without resorting to a
//! `row![text, text]` hack. Each [`Span`] carries an optional theme-aware
//! color [`Token`] that is resolved against the active palette at draw time
//! — so the same widget renders correctly in light and dark modes without
//! the user ever touching a concrete `Color`.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::rich_text;
//! use icedxdy::widget::{rich_text::span, text};
//! use icedxdy::theme::tweak::Token;
//!
//! rich_text![
//!     span("Required Field ").size(text::size::SM).medium(),
//!     span("*").size(text::size::SM).color(Token::Destructive),
//! ]
//! ```
//!
//! # Implementation
//!
//! Iced 0.15's stock `iced_widget::text::Rich` takes per-span concrete
//! `Color` at construction time. Per-span theme-resolved colors require
//! reaching `&Theme` after the spans were built, which the stock widget
//! doesn't expose. We vendor the minimal `Rich::layout` / `Rich::draw`
//! algorithm here and resolve `Token` → `Color` at draw time, rebuilding
//! the cached paragraph when the palette changes.

use std::ops::Add;

use crate::core::Renderer as _;
use crate::core::alignment;
use crate::core::font;
use crate::core::font::{Feature, Variation};
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text;
use crate::core::text::Renderer as _;
use crate::core::text::paragraph::Paragraph as _;
use crate::core::widget::text::{Alignment, Catalog, Shaping, Wrapping, draw};
use crate::core::widget::tree::{self, Tree};
use crate::core::{
    Color, Element, Em, Event, Layout, Length, Pixels, Rectangle, Shell, Size, Text, Widget,
};
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme::tweak::Token;

use crate::component::{Component, IntoElement};

/// Local alias for iced's text fragment so callers don't have to type
/// the iced-core path inline.
type Fragment<'a> = text::Fragment<'a>;

/// One inline run of text inside a [`RichText`].
///
/// Mirrors the subset of iced's text `Span` we need, plus a
/// [`Token`]-typed color that resolves against the active theme palette
/// at draw time. Underline, strikethrough and link semantics are punted
/// for now — no current call site needs them.
#[derive(Debug, Clone)]
pub struct Span<'a> {
    fragment: Fragment<'a>,
    size: Option<Pixels>,
    line_height: Option<text::LineHeight>,
    weight: Option<font::Weight>,
    italic: bool,
    color: Option<Token>,
    letter_spacing: Option<Em>,
}

/// Create a [`Span`] with the given text fragment.
pub fn span<'a>(fragment: impl text::IntoFragment<'a>) -> Span<'a> {
    Span {
        fragment: fragment.into_fragment(),
        size: None,
        line_height: None,
        weight: None,
        italic: false,
        color: None,
        letter_spacing: None,
    }
}

impl<'a> Span<'a> {
    /// Set the font size of this span.
    pub fn size(mut self, size: impl Into<crate::Size>) -> Self {
        self.size = Some(Pixels(size.into().text_size()));
        self
    }

    /// Set the font weight of this span.
    pub fn weight(mut self, weight: impl Into<crate::Weight>) -> Self {
        self.weight = Some(weight.into().into());
        self
    }

    /// Shorthand for [`Self::weight`] with [`Weight::Medium`].
    pub fn medium(self) -> Self {
        self.weight(crate::Weight::Medium)
    }

    /// Shorthand for [`Self::weight`] with [`Weight::Semibold`].
    pub fn semibold(self) -> Self {
        self.weight(crate::Weight::Semibold)
    }

    /// Shorthand for [`Self::weight`] with [`Weight::Bold`].
    pub fn bold(self) -> Self {
        self.weight(crate::Weight::Bold)
    }

    /// Render this span in italic.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Override the line height of this span.
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// Override the letter spacing of this span.
    pub fn letter_spacing(mut self, letter_spacing: impl Into<Em>) -> Self {
        self.letter_spacing = Some(letter_spacing.into());
        self
    }

    /// Apply a theme-aware color to this span.
    ///
    /// Resolves against the active palette at draw time, so the same
    /// span renders correctly in both light and dark modes.
    pub fn color(mut self, color: impl Into<Token>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the size iff this span has none.
    fn fallback_size(mut self, size: f32) -> Self {
        if self.size.is_none() {
            self.size = Some(Pixels(size));
        }
        self
    }

    /// Set the weight iff this span has none.
    fn fallback_weight(mut self, weight: font::Weight) -> Self {
        if self.weight.is_none() {
            self.weight = Some(weight);
        }
        self
    }

    /// Build the iced span used for *layout* (color stripped — color is a
    /// rendering concern and including it would needlessly invalidate the
    /// paragraph cache when the palette changes between draws).
    fn to_layout_span(&self) -> text::Span<'static, (), iced_core::Font> {
        let mut font = match self.italic {
            true => Some(iced_core::Font::DEFAULT.style(font::Style::Italic)),
            false => None,
        };
        if let Some(weight) = self.weight {
            let base = font.unwrap_or(iced_core::Font::DEFAULT);
            font = Some(base.weight(weight));
        }
        let mut span = text::Span::new(self.fragment.clone().into_owned());
        if let Some(size) = self.size {
            span = span.size(size);
        }
        if let Some(lh) = self.line_height {
            span = span.line_height(lh);
        }
        if let Some(font) = font {
            span = span.font(font);
        }
        if let Some(letter_spacing) = self.letter_spacing {
            span = span.letter_spacing(letter_spacing);
        }
        span
    }

    /// Build the iced span used for *drawing*, with `color` resolved
    /// against `palette`.
    fn to_draw_span(
        &self,
        palette: &icedxdy_core::Palette,
    ) -> text::Span<'static, (), iced_core::Font> {
        let mut span = self.to_layout_span();
        if let Some(token) = self.color {
            let color: Color = token.resolve(palette).into();
            span = span.color(color);
        }
        span
    }
}

// `&str` / `String` ergonomics: `rich_text![span("..."), "literal"]` works.
impl<'a> From<&'a str> for Span<'a> {
    fn from(s: &'a str) -> Self {
        span(s)
    }
}

impl From<String> for Span<'_> {
    fn from(s: String) -> Self {
        span(s)
    }
}

// Compose with a `Token` via `+` — `span("hi") + Token::Destructive`.
impl<'a> Add<Token> for Span<'a> {
    type Output = Span<'a>;
    fn add(self, rhs: Token) -> Span<'a> {
        self.color(rhs)
    }
}

/// A bunch of inline [`Span`]s rendered as a single text widget.
///
/// Theme-aware: each span's [`Token`] color is resolved against the active
/// palette at draw time, so swapping themes re-colors spans correctly.
pub struct RichText<'a> {
    spans: Vec<Span<'a>>,
    size: Option<Pixels>,
    line_height: text::LineHeight,
    width: Length,
    height: Length,
    align_x: Alignment,
    align_y: alignment::Vertical,
    wrapping: Wrapping,
    letter_spacing: Em,
    /// Default color applied to spans that lack their own `color`. Driven
    /// by the catalog (so call sites can `style(theme::text::Foreground)`
    /// to set a baseline) — but most sites don't need it.
    class: <Theme as Catalog>::Class<'a>,
}

/// Create a [`RichText`] from an iterator of spans.
pub fn rich_text<'a>(spans: impl IntoIterator<Item = Span<'a>>) -> RichText<'a> {
    RichText {
        spans: spans.into_iter().collect(),
        size: None,
        line_height: text::LineHeight::default(),
        width: Length::Shrink,
        height: Length::Shrink,
        align_x: Alignment::Default,
        align_y: alignment::Vertical::Top,
        wrapping: Wrapping::default(),
        letter_spacing: Em::default(),
        class: <Theme as Catalog>::default(),
    }
}

impl<'a> RichText<'a> {
    /// Set the default font size of the rich text. Spans may override
    /// per-span.
    pub fn size(mut self, size: impl Into<crate::Size>) -> Self {
        self.size = Some(Pixels(size.into().text_size()));
        self
    }

    /// Set the line height.
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// Set the width of the layout box.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height of the layout box.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set the horizontal alignment.
    pub fn align_x(mut self, alignment: impl Into<Alignment>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Set the vertical alignment.
    pub fn align_y(mut self, alignment: impl Into<alignment::Vertical>) -> Self {
        self.align_y = alignment.into();
        self
    }

    /// Set the wrapping strategy.
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Set the letter spacing.
    pub fn letter_spacing(mut self, letter_spacing: impl Into<Em>) -> Self {
        self.letter_spacing = letter_spacing.into();
        self
    }

    /// Set the catalog class for the default span color (analogous to
    /// iced's `Rich::style`).
    pub fn style(mut self, style: impl Into<iced_widget::text::StyleFn<'a, Theme>>) -> Self {
        self.class = style.into();
        self
    }
}

// Tree state: cached paragraph + the spans + palette signature used to
// build them. Color isn't part of paragraph shape, but iced's
// `Paragraph::compare` only diffs the *outer* `Text` (size, font, bounds,
// etc.), not the per-span list — so `with_spans` is the only path that
// reshapes when spans change. We therefore rebuild the paragraph whenever
// our drawn-span list (which IS palette-dependent) differs.
struct State<P: text::Paragraph> {
    spans: Vec<text::Span<'static, (), P::Font>>,
    paragraph: P,
}

impl<'a, Message: 'a> Widget<Message, Theme, Renderer> for RichText<'a> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<<Renderer as text::Renderer>::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        let paragraph = <<Renderer as text::Renderer>::Paragraph>::default();
        tree::State::new(State {
            spans: Vec::<text::Span<'static, (), iced_core::Font>>::new(),
            paragraph,
        })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // We don't have the theme here, so layout uses each span's
        // *layout* shape (no color). Color is rendering-only and gets
        // applied in `draw` by reshaping the paragraph against the live
        // palette. The cached paragraph's dimensions are color-independent.
        let state = tree
            .state
            .downcast_mut::<State<<Renderer as text::Renderer>::Paragraph>>();
        let layout_spans: Vec<_> = self.spans.iter().map(Span::to_layout_span).collect();

        layout::sized(limits, self.width, self.height, |limits| {
            let bounds = limits.max();
            let size = self.size.unwrap_or_else(|| renderer.default_size());
            let font = renderer.default_font();

            let text = Text {
                content: layout_spans.as_slice(),
                bounds,
                size,
                line_height: self.line_height,
                font,
                align_x: self.align_x,
                align_y: self.align_y,
                shaping: Shaping::Advanced,
                wrapping: self.wrapping,
                ellipsis: text::Ellipsis::default(),
                letter_spacing: self.letter_spacing,
                font_features: Vec::<Feature>::new(),
                font_variations: Vec::<Variation>::new(),
                weight: None,
                hint_factor: renderer.scale_factor(),
            };

            // Reshape the paragraph if our (color-stripped) layout spans
            // differ. This catches changes in size, weight, font, italic,
            // letter spacing — anything that affects shape.
            if state.spans != layout_spans {
                state.paragraph = <<Renderer as text::Renderer>::Paragraph>::with_spans(text);
                state.spans = layout_spans
                    .iter()
                    .cloned()
                    .map(text::Span::to_static)
                    .collect();
            } else {
                let probe = Text {
                    content: (),
                    bounds: text.bounds,
                    size: text.size,
                    line_height: text.line_height,
                    font: text.font,
                    align_x: text.align_x,
                    align_y: text.align_y,
                    shaping: text.shaping,
                    wrapping: text.wrapping,
                    ellipsis: text.ellipsis,
                    letter_spacing: text.letter_spacing,
                    font_features: text.font_features.clone(),
                    font_variations: text.font_variations.clone(),
                    weight: text.weight,
                    hint_factor: text.hint_factor,
                };
                match state.paragraph.compare(probe) {
                    text::Difference::None => {}
                    text::Difference::Bounds => state.paragraph.resize(bounds),
                    text::Difference::Shape => {
                        state.paragraph =
                            <<Renderer as text::Renderer>::Paragraph>::with_spans(text);
                    }
                }
            }

            state.paragraph.min_bounds()
        })
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if !layout.bounds().intersects(viewport) {
            return;
        }
        let state = tree
            .state
            .downcast_ref::<State<<Renderer as text::Renderer>::Paragraph>>();
        let palette = theme.palette();

        // Resolve each span's Token against the live palette. Build a
        // throwaway paragraph for drawing — we don't want to mutate the
        // cached layout paragraph from `&self`. The drawn paragraph has
        // identical *shape* (size/font/weight/italic/letter-spacing match
        // the layout span); only `color` differs.
        //
        // PERF: We reshape the paragraph every draw. For the small
        // (1–10 span) labels this widget is meant for, the cost is in the
        // microseconds and not worth optimizing yet. If a future call site
        // hits perf issues, the right fix is interior mutability + a cached
        // palette signature on `State`, or moving the resolution into
        // `update` and triggering a relayout on theme change.
        let draw_spans: Vec<_> = self
            .spans
            .iter()
            .map(|s| s.to_draw_span(&palette))
            .collect();

        let bounds = layout.bounds().size();
        let size = self.size.unwrap_or_else(|| renderer.default_size());
        let font = renderer.default_font();

        let draw_paragraph = <<Renderer as text::Renderer>::Paragraph>::with_spans(Text {
            content: draw_spans.as_slice(),
            bounds,
            size,
            line_height: self.line_height,
            font,
            align_x: self.align_x,
            align_y: self.align_y,
            shaping: Shaping::Advanced,
            wrapping: self.wrapping,
            ellipsis: text::Ellipsis::default(),
            letter_spacing: self.letter_spacing,
            font_features: Vec::<Feature>::new(),
            font_variations: Vec::<Variation>::new(),
            weight: None,
            hint_factor: renderer.scale_factor(),
        });

        // Match iced's `Rich::draw` underline / strikethrough
        // / highlight handling — none of our `Span` builder methods enable
        // those today, but use the cached paragraph's `span_bounds` if a
        // future caller turns them on. (Skipped for now; revisit when we
        // surface `.underline()` / `.strikethrough()` in `Span`.)
        let _ = state; // cached layout paragraph; layout passes already used it

        let style = <Theme as Catalog>::style(theme, &self.class);
        draw(
            renderer,
            defaults,
            layout.bounds(),
            &draw_paragraph,
            style,
            viewport,
        );
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        _event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::None
    }
}

impl<'a, Message: 'a> From<RichText<'a>> for Element<'a, Message, Theme, Renderer> {
    fn from(rich: RichText<'a>) -> Self {
        Element::new(rich)
    }
}

impl<'a, Message: 'a> From<RichText<'a>> for Component<'a, Message> {
    fn from(rich: RichText<'a>) -> Self {
        Component::new(rich)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for RichText<'a> {
    fn size(&mut self, size: crate::Size) {
        if self.size.is_none() {
            self.size = Some(Pixels(size.text_size()));
        }
        // Push the size hint into spans that haven't set one — matches
        // `Row` / `Column` propagation semantics so e.g. `.size(Sm)` on
        // an enclosing button cascades into rich-text spans.
        let pixels = size.text_size();
        for s in self.spans.iter_mut() {
            *s = std::mem::replace(s, span("")).fallback_size(pixels);
        }
    }

    fn weight(&mut self, weight: crate::Weight) {
        let w: font::Weight = weight.into();
        for s in self.spans.iter_mut() {
            *s = std::mem::replace(s, span("")).fallback_weight(w);
        }
    }

    fn into_element(self: Box<Self>) -> Element<'a, Message, Theme, Renderer> {
        (*self).into()
    }
}

/// Build a [`RichText`] from a comma-separated list of [`Span`]s. Mirrors
/// the shape of `row!` / `column!`.
///
/// ```ignore
/// rich_text![
///     span("Required Field ").medium(),
///     span("*").color(Token::Destructive),
/// ]
/// ```
#[macro_export]
macro_rules! rich_text {
    () => {
        $crate::rich_text::rich_text(::std::iter::empty::<$crate::rich_text::Span<'_>>())
    };
    ($($span:expr),+ $(,)?) => {
        $crate::rich_text::rich_text([
            $(::core::convert::Into::<$crate::rich_text::Span<'_>>::into($span)),+
        ])
    };
}
