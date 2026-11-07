//! Typography components
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::text;
//!
//! text::h1("Page Title")
//! text::muted("Secondary text")
//! text::p("Bold text").bold()
//! ```

use iced_widget::Widget;
use std::marker::PhantomData;

use crate::core::Length;
use crate::core::Length::Fill;
use crate::core::Padding;
use crate::core::font;
use crate::core::font::Weight;
use crate::core::text::{Fragment, IntoFragment, LineHeight};
use iced_widget::Renderer;
use iced_widget::text::StyleFn;

use crate::Size;
use crate::component::{Component, IntoElement};
use crate::rule;
use icedxdy_core::Theme;
use icedxdy_core::theme;

/// Font sizes based on Tailwind/shadcn scale
pub mod size {
    pub const XS: f32 = 12.0;
    pub const SM: f32 = 14.0;
    pub const BASE: f32 = 16.0;
    pub const LG: f32 = 18.0;
    pub const XL: f32 = 20.0;
    pub const XL2: f32 = 24.0;
    pub const XL3: f32 = 30.0;
    pub const XL4: f32 = 36.0;
}

/// Text widget wrapper with semantic styling
pub struct Text<'a> {
    fragment: Fragment<'a>,
    font: Option<iced_core::Font>,
    size: Option<f32>,
    weight: Option<Weight>,
    italic: bool,
    letter_spacing: Option<iced_core::Em>,
    line_height: Option<LineHeight>,
    style: Option<StyleFn<'a, Theme>>,
    width: Option<Length>,
}

impl<'a> Text<'a> {
    pub fn new(content: impl IntoFragment<'a>) -> Self {
        Self {
            fragment: content.into_fragment(),
            font: None,
            size: None,
            weight: None,
            italic: false,
            letter_spacing: None,
            line_height: None,
            style: None,
            width: None,
        }
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into().text_size());
        self
    }

    pub fn weight(mut self, weight: impl Into<crate::Weight>) -> Self {
        self.weight = Some(weight.into().into());
        self
    }

    pub fn medium(self) -> Self {
        self.weight(crate::Weight::Medium)
    }

    pub fn semibold(self) -> Self {
        self.weight(crate::Weight::Semibold)
    }

    pub fn bold(self) -> Self {
        self.weight(crate::Weight::Bold)
    }

    /// Apply italic font style. Used by `blockquote` to match nova's
    /// `border-l-2 pl-6 italic` cascade.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn letter_spacing(mut self, spacing: impl Into<iced_core::Em>) -> Self {
        self.letter_spacing = Some(spacing.into());
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// Override the text widget's layout width. Defaults to `Shrink`,
    /// matching `Text::new()`. Add `.width(Fill)` at the call site for
    /// paragraph copy that should occupy the parent's full width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Set the style.
    ///
    /// Accepts any text-style variant struct (`theme::text::Foreground`,
    /// `theme::text::Muted`, etc.), a `Spec` (`Foreground + Alpha(0.5)`),
    /// or an explicit `Box::new(closure)`.
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Build the iced text widget
    fn build(self) -> iced_widget::Text<'a, Theme> {
        let mut widget = iced_widget::text(self.fragment);

        if let Some(size) = self.size {
            widget = widget.size(size);
        }

        // Italic is folded into the font (no Format-level override exists),
        // so we layer it onto whichever font is set. Defaulting to
        // `Font::DEFAULT` here would clobber `font::set_defaults`, but we
        // only do it when the caller actually wants italic — the same
        // tradeoff `bold()` would have if we modeled it that way.
        let font = match (self.font, self.italic) {
            (Some(f), true) => Some(f.style(font::Style::Italic)),
            (Some(f), false) => Some(f),
            (None, true) => Some(iced_core::Font::DEFAULT.style(font::Style::Italic)),
            (None, false) => None,
        };

        if let Some(font) = font {
            widget = widget.font(font);
        }

        if let Some(weight) = self.weight {
            widget = widget.weight(weight);
        }

        if let Some(letter_spacing) = self.letter_spacing {
            widget = widget.letter_spacing(letter_spacing);
        }

        if let Some(line_height) = self.line_height {
            widget = widget.line_height(line_height);
        }

        if let Some(style) = self.style {
            widget = widget.style(style);
        }

        if let Some(width) = self.width {
            widget = widget.width(width);
        }

        widget
    }
}

impl<'a, Message: 'a> From<Text<'a>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(t: Text<'a>) -> Self {
        t.build().boxed()
    }
}

impl<'a, Message: 'a> From<Text<'a>> for crate::Component<'a, Message> {
    fn from(t: Text<'a>) -> Self {
        crate::Component::new(t)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Text<'a> {
    fn size(&mut self, size: Size) {
        if self.size.is_none() {
            self.size = Some(size.text_size());
        }
    }

    fn weight(&mut self, weight: crate::Weight) {
        if self.weight.is_none() {
            self.weight = Some(weight.into());
        }
    }

    fn line_height(&mut self, line_height: f32) {
        // Treat the pushed value as absolute logical pixels (matches nova's
        // `leading-4` = 16px semantic). `From<f32> for LineHeight` would
        // interpret it as a relative factor, so build `Absolute` explicitly.
        if self.line_height.is_none() {
            self.line_height = Some(LineHeight::Absolute(iced_core::Pixels(line_height)));
        }
    }

    fn foreground_color(&mut self, color: iced_core::Color) {
        if self.style.is_none() {
            self.style = Some(Box::new(move |_| iced_widget::text::Style {
                color: Some(color),
                selection: None,
            }));
        }
    }

    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        self.build().boxed()
    }
}

pub fn h1<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL3)
        .semibold()
        .letter_spacing(-0.025)
        .style(theme::text::Foreground)
}

pub fn h2<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL)
        .medium()
        .letter_spacing(-0.025)
        .style(theme::text::Foreground)
}

pub fn h3<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::LG)
        .medium()
        .letter_spacing(-0.025)
        .style(theme::text::Foreground)
}

pub fn h4<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::BASE)
        .medium()
        .letter_spacing(-0.025)
        .style(theme::text::Foreground)
}

pub fn p<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::BASE)
        .style(theme::text::Foreground)
}

pub fn lead<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::XL).style(theme::text::Muted)
}

pub fn large<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::LG)
        .semibold()
        .style(theme::text::Foreground)
}

pub fn small<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .style(theme::text::Foreground)
}

/// Field-style label typography (Tailwind `text-sm font-medium leading-none`).
/// `leading-none` (`line_height(1.0)`) strips the invisible line-height
/// padding that otherwise throws off vertical alignment math in horizontal
/// Field layouts — mirroring shadcn's `<Label>` cascade exactly.
pub fn label<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .medium()
        .style(theme::text::Foreground)
        .line_height(LineHeight::Relative(1.0))
}

pub fn muted<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content).size(size::SM).style(theme::text::Muted)
}

/// Field-style error message typography
/// (Tailwind `text-sm text-destructive leading-none`). `leading-none`
/// matches shadcn's `<FieldError>` typography and keeps the vertical
/// rhythm consistent with the label above it.
pub fn error<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .style(theme::text::Destructive)
        .line_height(LineHeight::Relative(1.0))
}

pub fn xs<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XS)
        .style(theme::text::Foreground)
}

/// A blockquote: 2px left rule + 24px gap + italic body, all in one
/// widget. Mirrors shadcn nova's `border-l-2 pl-6 italic` cascade.
///
/// Vertical spacing (`mt-6` in shadcn) is the caller's responsibility —
/// drive it from the enclosing column's `.spacing(...)`.
pub struct Blockquote<'a> {
    body: Text<'a>,
}

impl<'a> Blockquote<'a> {
    /// Override the body text size. Defaults to `size::BASE` (16px).
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.body = self.body.size(size);
        self
    }

    /// Override the body text style. Defaults to `theme::text::Foreground`.
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self {
        self.body = self.body.style(style);
        self
    }
}

/// Create a [`Blockquote`] for the given quote text. Italic, foreground
/// color, fill width — plus the 2px left rule + 24px left padding that
/// the bare `Text` helper alone can't express.
pub fn blockquote<'a>(content: impl IntoFragment<'a>) -> Blockquote<'a> {
    Blockquote {
        body: Text::new(content)
            .size(size::BASE)
            .italic()
            .style(theme::text::Foreground)
            .width(Fill),
    }
}

impl<'a, Message: 'a> From<Blockquote<'a>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(b: Blockquote<'a>) -> Self {
        Box::new(b).into_element()
    }
}

impl<'a, Message: 'a> From<Blockquote<'a>> for Component<'a, Message> {
    fn from(b: Blockquote<'a>) -> Self {
        Component::new(b)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Blockquote<'a> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Blockquote { body } = *self;

        iced_widget::Row::new()
            .spacing(24)
            .height(Length::Shrink)
            .push(rule::vertical(2).boxed())
            .push(body.build().boxed())
            .boxed()
    }
}

/// An inline `code`-styled fragment. Shadcn classlist:
/// `relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono
/// text-sm font-semibold`.
///
/// Font + size + weight live on the returned `Text`; `bg-muted` +
/// padding + radius are container-level — the call site wraps this
/// in a small container.
pub fn inline_code<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .semibold()
        .style(theme::text::Foreground)
}

/// A bulleted list. Mirrors shadcn nova's
/// `<ul class="ml-6 list-disc [&>li]:mt-2">` cascade — 24px left
/// indent, 8px gap between items, body-size foreground text with a
/// hard `\u{2022}` prefix per item (the closest iced approximation
/// of native browser bullets).
///
/// Vertical spacing around the list (`my-6` in shadcn) is the
/// caller's responsibility — drive it from the enclosing column's
/// `.spacing(...)`.
pub struct List<'a, Message> {
    items: Vec<Text<'a>>,
    _phantom: PhantomData<Message>,
}

/// Create a [`List`] from an iterable of fragments. Empty input
/// produces an empty list (no panic).
pub fn list<'a, Message, I, F>(items: I) -> List<'a, Message>
where
    I: IntoIterator<Item = F>,
    F: IntoFragment<'a>,
{
    let items = items
        .into_iter()
        .map(|f| {
            Text::new(format!("\u{2022} {}", f.into_fragment()))
                .size(size::BASE)
                .style(theme::text::Foreground)
                .width(Fill)
        })
        .collect();

    List {
        items,
        _phantom: PhantomData,
    }
}

impl<'a, Message: 'a> From<List<'a, Message>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(list: List<'a, Message>) -> Self {
        Box::new(list).into_element()
    }
}

impl<'a, Message: 'a> From<List<'a, Message>> for Component<'a, Message> {
    fn from(list: List<'a, Message>) -> Self {
        Component::new(list)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for List<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let List { items, .. } = *self;

        let column = items.into_iter().fold(
            iced_widget::Column::new().spacing(8).width(Fill),
            |col, item| col.push(item.build()),
        );

        iced_widget::container(column)
            .padding(Padding {
                left: 24.0,
                ..Padding::ZERO
            })
            .width(Fill)
            .height(Length::Shrink)
            .boxed()
    }
}
