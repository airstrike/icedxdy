//! Togglers (switches) let users make binary choices by toggling a switch.
//!
//! This is a sweetened version of `iced`'s [`toggler`], vendored from
//! sweeten with two additions on top of sweeten's animated thumb slide:
//!
//! - **Focus ring painter** in `draw()` — when `Style::ring` is `Some`, a
//!   colored halo is rendered behind the track bounds. Stock iced's
//!   `toggler::Style` has no ring field; we add it here so our nova-correct
//!   focus-visible halo (`focus-visible:ring-3 ring-ring/50`) renders.
//! - **Local `Style` / `Catalog` / `StyleFn`** with the `ring` field,
//!   plus a lossless `From<iced_widget::toggler::Style>` so the Catalog
//!   impl on `iced_core::Theme` (sweeten's stock fallback) keeps working.
//!
//! [`toggler`]: https://docs.iced.rs/iced/widget/toggler/
use crate::core::alignment;
use crate::core::animation::Easing;
use crate::core::border;
use crate::core::font;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text;
use crate::core::time::Instant;
use crate::core::touch;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Animation, Background, Border, Color, Element, Em, Event, Layout, Length, Pixels, Rectangle,
    Shell, Size, Theme, Widget,
};

// Iced's widget crate re-exports `core::widget::text::*` at `crate::text`. We
// can't reuse that path (our `crate::text` is the icedxdy typography helpers,
// and `use iced_widget::text` collides with `crate::core::text` already in
// scope above). A private re-exporting module gives the vendored draw + layout
// calls shallow `text_helpers::*` paths that satisfy the pre-commit deep-path
// / aliased-import checks. Mirrors `widget/src/checkbox/widget.rs`.
mod text_helpers {
    pub use iced_widget::text::{Format, State, Style, draw, layout};
}

/// A toggler (switch) widget.
pub struct Toggler<
    'a,
    Message,
    Theme = iced_widget::Theme,
    Renderer = iced_widget::Renderer,
> where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    is_toggled: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    label: Option<text::Fragment<'a>>,
    width: Length,
    size: f32,
    text_size: Option<Pixels>,
    line_height: text::LineHeight,
    alignment: text::Alignment,
    text_shaping: text::Shaping,
    wrapping: text::Wrapping,
    letter_spacing: Em,
    font_features: Vec<font::Feature>,
    spacing: f32,
    font: Option<Renderer::Font>,
    class: Theme::Class<'a>,
    interaction: Option<mouse::Interaction>,
    last_status: Option<Status>,
}

impl<'a, Message, Theme, Renderer> Toggler<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    /// The default size of a [`Toggler`].
    ///
    /// `DEFAULT_SIZE` represents the **track height** in pixels. Nova default:
    /// `data-[size=default]:h-[18.4px]`. The track **width** is derived as
    /// `32.0 / 18.4 * size` so the 32 × 18.4 aspect ratio is preserved at
    /// any caller-supplied size. The thumb diameter is
    /// `size - 2 * (padding_ratio * size) ≈ 16 px` at the default size,
    /// matching `group-data-[size=default]/switch:size-4` (16 × 16 px).
    ///
    /// Sweeten uses 16; we use 18.4 to match the nova spec exactly.
    pub const DEFAULT_SIZE: f32 = 18.4;

    /// Creates a new [`Toggler`].
    pub fn new(is_toggled: bool) -> Self {
        Toggler {
            is_toggled,
            on_toggle: None,
            label: None,
            width: Length::Shrink,
            size: Self::DEFAULT_SIZE,
            text_size: None,
            line_height: text::LineHeight::default(),
            alignment: text::Alignment::Default,
            text_shaping: text::Shaping::default(),
            wrapping: text::Wrapping::default(),
            letter_spacing: Em::default(),
            font_features: Vec::new(),
            // Nova's demo uses `space-x-2` = 8 px between the switch and its
            // inline label. Fix at 8 px regardless of size so it doesn't
            // drift when callers pass a custom size.
            spacing: 8.0,
            font: None,
            class: Theme::default(),
            interaction: None,
            last_status: None,
        }
    }

    /// Sets the label of the [`Toggler`].
    pub fn label(mut self, label: impl text::IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Sets the message that should be produced when a user toggles
    /// the [`Toggler`].
    ///
    /// If this method is not called, the [`Toggler`] will be disabled.
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// Sets the message that should be produced when a user toggles
    /// the [`Toggler`], if `Some`.
    ///
    /// If `None`, the [`Toggler`] will be disabled.
    pub fn on_toggle_maybe(
        mut self,
        on_toggle: Option<impl Fn(bool) -> Message + 'a>,
    ) -> Self {
        self.on_toggle = on_toggle.map(|on_toggle| Box::new(on_toggle) as _);
        self
    }

    /// Sets the size of the [`Toggler`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// Sets the width of the [`Toggler`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the text size of the [`Toggler`].
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// Sets the text [`text::LineHeight`] of the [`Toggler`].
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// Sets the horizontal alignment of the text of the [`Toggler`]
    pub fn alignment(mut self, alignment: impl Into<text::Alignment>) -> Self {
        self.alignment = alignment.into();
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`Toggler`].
    pub fn shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the [`text::Wrapping`] strategy of the [`Toggler`].
    pub fn wrapping(mut self, wrapping: text::Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Sets the letter spacing of the [`Toggler`].
    pub fn letter_spacing(mut self, letter_spacing: impl Into<Em>) -> Self {
        self.letter_spacing = letter_spacing.into();
        self
    }

    /// Adds a single font feature to the [`Toggler`].
    pub fn font_feature(mut self, feature: impl Into<font::Feature>) -> Self {
        self.font_features.push(feature.into());
        self
    }

    /// Sets the font features of the [`Toggler`].
    pub fn font_features(mut self, features: Vec<font::Feature>) -> Self {
        self.font_features = features;
        self
    }

    /// Sets the spacing between the [`Toggler`] and the text.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the [`Renderer::Font`] of the text of the [`Toggler`].
    ///
    /// [`Renderer::Font`]: text::Renderer
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the [`mouse::Interaction`] to use when the cursor is over the
    /// [`Toggler`].
    #[must_use]
    pub fn interaction(mut self, interaction: mouse::Interaction) -> Self {
        self.interaction = Some(interaction);
        self
    }

    /// Sets the style of the [`Toggler`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Toggler`].
    /// **Deviation from upstream sweeten**: stock sweeten gates `class` behind
    /// the `advanced` feature flag. We always expose it because our
    /// `.style(...)` bridge in `widget/src/switch.rs` needs to land a
    /// pre-baked `widget::StyleFn` (the closure that runs `from_core` on the
    /// core StyleFn's output). Without `class`, that bridge has nowhere to
    /// land. Mirrors the same deviation in `widget/src/checkbox/widget.rs`.
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

/// Internal state for the animated [`Toggler`].
struct State<P: text::Paragraph> {
    paragraph: text_helpers::State<P>,
    animation: Animation<bool>,
    now: Option<Instant>,
    last_is_toggled: bool,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Toggler<'_, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Renderer::Paragraph> {
            paragraph: text_helpers::State::default(),
            animation: Animation::new(self.is_toggled)
                .very_quick()
                .easing(Easing::EaseOut),
            now: None,
            last_is_toggled: self.is_toggled,
        })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width);

        layout::next_to_each_other(
            &limits,
            if self.label.is_some() {
                self.spacing
            } else {
                0.0
            },
            |_| {
                let size = if renderer::CRISP {
                    let scale_factor = renderer.scale_factor().unwrap_or(1.0);

                    (self.size * scale_factor).round() / scale_factor
                } else {
                    self.size
                };

                // Nova default: `data-[size=default]:w-[32px] h-[18.4px]`.
                // Width = (32 / 18.4) * size preserves the exact aspect ratio
                // at any caller-supplied size. At DEFAULT_SIZE = 18.4 this
                // yields exactly 32 × 18.4. (Sweeten used 2.0 × size = 32 × 16.)
                layout::Node::new(Size::new(32.0 / 18.4 * size, size))
            },
            |limits| {
                if let Some(label) = self.label.as_deref() {
                    let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

                    text_helpers::layout(
                        &mut state.paragraph,
                        renderer,
                        limits,
                        label,
                        text_helpers::Format {
                            width: self.width,
                            height: Length::Shrink,
                            line_height: self.line_height,
                            size: self.text_size,
                            font: self.font,
                            align_x: self.alignment,
                            align_y: alignment::Vertical::Top,
                            shaping: self.text_shaping,
                            wrapping: self.wrapping,
                            ellipsis: text::Ellipsis::None,
                            letter_spacing: self.letter_spacing,
                            font_features: self.font_features.clone(),
                            font_variations: Vec::new(),
                            weight: None,
                        },
                    )
                } else {
                    layout::Node::new(Size::ZERO)
                }
            },
        )
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        // Animation bookkeeping — runs regardless of whether the widget is
        // enabled, so that in-flight animations complete even if the toggler
        // becomes disabled.
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = Some(*now);

            if state.animation.is_animating(*now) {
                shell.request_redraw();
            }
        }

        if self.is_toggled != state.last_is_toggled {
            state.last_is_toggled = self.is_toggled;

            if let Some(now) = state.now {
                state.animation.go_mut(self.is_toggled, now);
                shell.request_redraw();
            }
        }

        if let Some(on_toggle) = &self.on_toggle {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    let mouse_over = cursor.is_over(layout.bounds());

                    if mouse_over {
                        shell.publish(on_toggle(!self.is_toggled));
                        shell.capture_event();
                    }
                }
                _ => {}
            }
        }

        let current_status = if self.on_toggle.is_none() {
            Status::Disabled {
                is_toggled: self.is_toggled,
            }
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered {
                is_toggled: self.is_toggled,
            }
        } else {
            Status::Active {
                is_toggled: self.is_toggled,
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(current_status);
        } else if self
            .last_status
            .is_some_and(|status| status != current_status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            if self.on_toggle.is_none() {
                mouse::Interaction::NotAllowed
            } else {
                self.interaction.unwrap_or_default()
            }
        } else {
            mouse::Interaction::default()
        }
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
        let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();

        let mut children = layout.children();
        let toggler_layout = children.next().unwrap();

        let current_status = self.last_status.unwrap_or(Status::Disabled {
            is_toggled: self.is_toggled,
        });

        // While the toggle is animating, interpolate the fill colors between
        // the off- and on-state styles so the background/foreground fade in
        // sync with the handle slide, not snap instantly when `is_toggled`
        // flips.
        let style = match state.now {
            Some(now) if state.animation.is_animating(now) => {
                let off = theme.style(&self.class, current_status.with_toggled(false));
                let on = theme.style(&self.class, current_status.with_toggled(true));

                // Only `Background::Color` is interpolable here; fall back to
                // transparent for gradients so the draw code stays simple.
                let bg_color = |bg: Background| match bg {
                    Background::Color(c) => c,
                    Background::Gradient(_) => Color::TRANSPARENT,
                };

                let background = Background::Color(state.animation.interpolate(
                    bg_color(off.background),
                    bg_color(on.background),
                    now,
                ));
                let foreground = Background::Color(state.animation.interpolate(
                    bg_color(off.foreground),
                    bg_color(on.foreground),
                    now,
                ));

                // Snap non-color fields to the animation's current target —
                // interpolating border widths/radius/padding shimmers.
                let target = if state.animation.value() { on } else { off };
                Style {
                    background,
                    foreground,
                    ..target
                }
            }
            _ => theme.style(&self.class, current_status),
        };

        if self.label.is_some() {
            let label_layout = children.next().unwrap();

            text_helpers::draw(
                renderer,
                defaults,
                label_layout.bounds(),
                state.paragraph.raw(),
                text_helpers::Style {
                    color: style.text_color,
                },
                viewport,
            );
        }

        let scale_factor = renderer.scale_factor().unwrap_or(1.0);
        let bounds = toggler_layout.bounds();

        let border_radius = style
            .border_radius
            .unwrap_or_else(|| border::Radius::new(bounds.height / 2.0));

        // Ring: paint a halo *outside* the track bounds, behind the main
        // fill, when `style.ring` is `Some`. Mirrors the text_input + checkbox
        // painters (`widget/src/text_input/widget.rs` lines 503-516,
        // `widget/src/checkbox/widget.rs` ring block in `draw`).
        // `border.width = ring.width` paints inward from the expanded
        // bounds, so the visible halo wraps around the track.
        if let Some(ring) = style.ring {
            let r = ring.width;
            let mut radius = border_radius;
            radius.top_left += r;
            radius.top_right += r;
            radius.bottom_right += r;
            radius.bottom_left += r;
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x - r,
                        y: bounds.y - r,
                        width: bounds.width + r * 2.0,
                        height: bounds.height + r * 2.0,
                    },
                    border: Border {
                        radius,
                        width: r,
                        color: ring.color,
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: border_radius,
                    width: style.background_border_width,
                    color: style.background_border_color,
                },
                ..renderer::Quad::default()
            },
            style.background,
        );

        let toggle_bounds = {
            // Try to align toggle to the pixel grid
            let bounds = if renderer::CRISP {
                (bounds * scale_factor).round()
            } else {
                bounds
            };

            let padding = (style.padding_ratio * bounds.height).round();

            let left = padding;
            let right = bounds.width - bounds.height + padding;

            let x = match state.now {
                Some(now) => state.animation.interpolate(left, right, now),
                None => {
                    if self.is_toggled {
                        right
                    } else {
                        left
                    }
                }
            };

            Rectangle {
                x: bounds.x + x,
                y: bounds.y + padding,
                width: bounds.height - (2.0 * padding),
                height: bounds.height - (2.0 * padding),
            } * (1.0 / scale_factor)
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggle_bounds,
                border: Border {
                    radius: border_radius,
                    width: style.foreground_border_width,
                    color: style.foreground_border_color,
                },
                ..renderer::Quad::default()
            },
            style.foreground,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Toggler<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(
        toggler: Toggler<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(toggler)
    }
}

/// The possible status of a [`Toggler`].
///
/// **Deviation from upstream iced**: `Status` is re-exported from
/// `icedxdy_core` so the whole crate (theme + widget + wrapper) speaks one
/// canonical type. Variants match upstream 1:1 so the rest of this file's
/// pattern matches keep working.
pub use icedxdy_core::theme::switch::Status;

trait StatusExt {
    fn with_toggled(self, is_toggled: bool) -> Self;
}

impl StatusExt for Status {
    fn with_toggled(self, is_toggled: bool) -> Self {
        match self {
            Status::Active { .. } => Status::Active { is_toggled },
            Status::Hovered { .. } => Status::Hovered { is_toggled },
            Status::Disabled { .. } => Status::Disabled { is_toggled },
        }
    }
}

/// A focus ring rendered around the switch track — a solid colored halo of
/// `width` pixels painted behind the main fill. Equivalent to CSS
/// `box-shadow: 0 0 0 {width}px {color}` (Tailwind `ring-*`).
///
/// **Deviation from upstream sweeten**: this widget renders a focus ring in
/// `draw()` when `Style::ring` is `Some`. Sweeten's `toggler::Style` (and
/// iced's stock one) have no ring field; we add it here so our nova-correct
/// focus halo (`focus-visible:ring-3 ring-ring/50`) renders.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    /// Halo color (already alpha-resolved by the catalog).
    pub color: Color,
    /// Halo thickness in pixels.
    pub width: f32,
}

/// The appearance of a toggler.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The background [`Color`] of the toggler.
    pub background: Background,
    /// The width of the background border of the toggler.
    pub background_border_width: f32,
    /// The [`Color`] of the background border of the toggler.
    pub background_border_color: Color,
    /// The foreground [`Color`] of the toggler.
    pub foreground: Background,
    /// The width of the foreground border of the toggler.
    pub foreground_border_width: f32,
    /// The [`Color`] of the foreground border of the toggler.
    pub foreground_border_color: Color,
    /// The text [`Color`] of the toggler.
    pub text_color: Option<Color>,
    /// The border radius of the toggler.
    ///
    /// If `None`, the toggler will be perfectly round.
    pub border_radius: Option<border::Radius>,
    /// The ratio of separation between the background and the toggle in
    /// relative height.
    pub padding_ratio: f32,
    /// Optional focus ring (Tailwind `ring-*`). `None` = no halo rendered.
    pub ring: Option<Ring>,
}

/// The theme catalog of a [`Toggler`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Toggler`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

/// Convert from iced's stock `toggler::Style` into the vendored widget's
/// `Style`. Lossless in this direction (iced has no `ring` field, so we set
/// `ring: None`). Used by `Catalog for iced_widget::Theme` to satisfy callers
/// that pick stock iced::Theme instead of `icedxdy_core::Theme`.
impl From<iced_widget::toggler::Style> for Style {
    fn from(s: iced_widget::toggler::Style) -> Self {
        Style {
            background: s.background.into(),
            background_border_width: s.background_border_width,
            background_border_color: s.background_border_color,
            foreground: s.foreground.into(),
            foreground_border_width: s.foreground_border_width,
            foreground_border_color: s.foreground_border_color,
            text_color: s.text_color,
            border_radius: s.border_radius,
            padding_ratio: s.padding_ratio,
            ring: None,
        }
    }
}

/// Stock-iced fallback Catalog impl — for callers who pick
/// `iced_core::Theme` instead of `icedxdy_core::Theme`. Routes through
/// iced's stock toggler default and lifts the result through
/// `From<iced_widget::toggler::Style> for Style`. Ring stays `None`.
impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a stock-iced themed toggler. Lifts through iced's
/// own default toggler resolver and converts to our `Style`.
pub fn default(theme: &Theme, status: Status) -> Style {
    let iced: iced_widget::toggler::Style =
        <Theme as iced_widget::toggler::Catalog>::style(
            theme,
            &<Theme as iced_widget::toggler::Catalog>::default(),
            status,
        );
    iced.into()
}
