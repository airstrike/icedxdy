//! Checkboxes can be used to let users make binary (or tri-state) choices.
//!
//! This is a sweetened version of `iced`'s [`checkbox`] with two additions:
//!
//! - **Indeterminate state** via the [`State`] domain enum. Stock iced's
//!   `is_checked: bool` lacks the `data-state="indeterminate"` flavor that
//!   shadcn surfaces; the [`State::Indeterminate`] variant renders a
//!   horizontal stroke instead of the check glyph.
//! - **Focus ring painter** in `draw()` — when `Style::ring` is `Some`, a
//!   colored halo is rendered behind the box bounds. Stock iced's
//!   `checkbox::Style` has no ring field; we add it here so our nova-correct
//!   focus-visible halo renders.
//!
//! [`checkbox`]: https://docs.iced.rs/iced/widget/checkbox/
use crate::core::alignment;
use crate::core::font;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text;
use crate::core::theme::palette;
use crate::core::touch;
use crate::core::widget;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Background, Border, Color, Element, Em, Event, Layout, Length, Pixels, Rectangle, Shell, Size,
    Theme, Widget,
};

// Iced's widget crate re-exports `core::widget::text::*` at `crate::text`. We
// can't reuse that path (our `crate::text` is the icedxdy typography helpers,
// and `use iced_widget::text` collides with `crate::core::text` already in
// scope above). A private re-exporting module gives the vendored draw + layout
// calls shallow `text_helpers::*` paths that satisfy the pre-commit deep-path
// / aliased-import checks.
mod text_helpers {
    pub use iced_widget::text::{Format, State, Style, draw, layout};
}

/// A box that can be checked, unchecked, or indeterminate.
///
/// **Deviation from upstream iced**: `state` is a [`State`] domain enum
/// (`Checked` / `Unchecked` / `Indeterminate`) instead of a `bool`. The
/// `From<bool> for State` impl preserves the bool-shaped call site:
/// `Checkbox::new(self.is_checked)` keeps working.
pub struct Checkbox<'a, Message, Theme = iced_widget::Theme, Renderer = iced_widget::Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    state: State,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    label: Option<text::Fragment<'a>>,
    width: Length,
    size: f32,
    spacing: f32,
    text_size: Option<Pixels>,
    line_height: text::LineHeight,
    shaping: text::Shaping,
    wrapping: text::Wrapping,
    letter_spacing: Em,
    font_features: Vec<font::Feature>,
    font: Option<Renderer::Font>,
    icon: Icon<Renderer::Font>,
    class: Theme::Class<'a>,
    last_status: Option<Status>,
}

impl<'a, Message, Theme, Renderer> Checkbox<'a, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    /// The default size of a [`Checkbox`].
    const DEFAULT_SIZE: f32 = 16.0;

    /// Creates a new [`Checkbox`].
    ///
    /// Accepts a [`State`] (the domain enum with `Checked`, `Unchecked`, and
    /// `Indeterminate` variants) — or a `bool`, via the
    /// `From<bool> for State` impl (`true` → `Checked`, `false` → `Unchecked`).
    pub fn new(state: impl Into<State>) -> Self {
        Checkbox {
            state: state.into(),
            on_toggle: None,
            label: None,
            width: Length::Shrink,
            size: Self::DEFAULT_SIZE,
            spacing: Self::DEFAULT_SIZE / 2.0,
            text_size: None,
            line_height: text::LineHeight::default(),
            shaping: text::Shaping::default(),
            wrapping: text::Wrapping::default(),
            letter_spacing: Em::default(),
            font_features: Vec::new(),
            font: None,
            icon: Icon {
                font: Renderer::ICON_FONT,
                code_point: Renderer::CHECKMARK_ICON,
                size: None,
                line_height: text::LineHeight::default(),
                shaping: text::Shaping::Basic,
            },
            class: Theme::default(),
            last_status: None,
        }
    }

    /// Sets the label of the [`Checkbox`].
    pub fn label(mut self, label: impl text::IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Sets the function that will be called when the [`Checkbox`] is toggled.
    /// It will receive the new state of the [`Checkbox`] and must produce a
    /// `Message`.
    ///
    /// Unless `on_toggle` is called, the [`Checkbox`] will be disabled.
    pub fn on_toggle<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(bool) -> Message,
    {
        self.on_toggle = Some(Box::new(f));
        self
    }

    /// Sets the function that will be called when the [`Checkbox`] is toggled,
    /// if `Some`.
    ///
    /// If `None`, the checkbox will be disabled.
    pub fn on_toggle_maybe<F>(mut self, f: Option<F>) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_toggle = f.map(|f| Box::new(f) as _);
        self
    }

    /// Sets the size of the [`Checkbox`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// Sets the width of the [`Checkbox`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the spacing between the [`Checkbox`] and the text.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the text size of the [`Checkbox`].
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// Sets the text [`text::LineHeight`] of the [`Checkbox`].
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`Checkbox`].
    pub fn shaping(mut self, shaping: text::Shaping) -> Self {
        self.shaping = shaping;
        self
    }

    /// Sets the [`text::Wrapping`] strategy of the [`Checkbox`].
    pub fn wrapping(mut self, wrapping: text::Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Sets the letter spacing of the [`Checkbox`].
    pub fn letter_spacing(mut self, letter_spacing: impl Into<Em>) -> Self {
        self.letter_spacing = letter_spacing.into();
        self
    }

    /// Adds a single font [`Feature`](font::Feature) to the [`Checkbox`].
    pub fn font_feature(mut self, feature: impl Into<font::Feature>) -> Self {
        self.font_features.push(feature.into());
        self
    }

    /// Sets the font features of the [`Checkbox`].
    pub fn font_features(mut self, features: Vec<font::Feature>) -> Self {
        self.font_features = features;
        self
    }

    /// Sets the [`Renderer::Font`] of the text of the [`Checkbox`].
    ///
    /// [`Renderer::Font`]: text::Renderer
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the [`Icon`] of the [`Checkbox`].
    pub fn icon(mut self, icon: Icon<Renderer::Font>) -> Self {
        self.icon = icon;
        self
    }

    /// Sets the style of the [`Checkbox`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Checkbox`].
    /// **Deviation from upstream iced**: stock iced gates `class` behind the
    /// `advanced` feature flag. We always expose it because our `.style(...)`
    /// bridge in `widget/src/checkbox.rs` needs to land a pre-baked
    /// `widget::StyleFn` (the closure that runs `from_core` on the core
    /// StyleFn's output). Without `class`, that bridge has nowhere to land.
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Checkbox<'_, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<text_helpers::State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(text_helpers::State::<Renderer::Paragraph>::default())
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
        layout::next_to_each_other(
            &limits.width(self.width),
            if self.label.is_some() {
                self.spacing
            } else {
                0.0
            },
            |_| layout::Node::new(Size::new(self.size, self.size)),
            |limits| {
                if let Some(label) = self.label.as_deref() {
                    let state = tree
                        .state
                        .downcast_mut::<text_helpers::State<Renderer::Paragraph>>();

                    text_helpers::layout(
                        state,
                        renderer,
                        limits,
                        label,
                        text_helpers::Format {
                            width: self.width,
                            height: Length::Shrink,
                            line_height: self.line_height,
                            size: self.text_size,
                            font: self.font,
                            align_x: text::Alignment::Default,
                            align_y: alignment::Vertical::Top,
                            shaping: self.shaping,
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
        _tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let mouse_over = cursor.is_over(layout.bounds());

                if mouse_over && let Some(on_toggle) = &self.on_toggle {
                    // Indeterminate clicks resolve to "checked" (matches the
                    // shadcn / radix convention: clicking a tri-state checkbox
                    // commits to the on state). True/False simply flip.
                    let next = match self.state {
                        State::Checked => false,
                        State::Unchecked | State::Indeterminate => true,
                    };
                    shell.publish((on_toggle)(next));
                    shell.capture_event();
                }
            }
            _ => {}
        }

        let current_status = {
            let is_mouse_over = cursor.is_over(layout.bounds());
            let is_disabled = self.on_toggle.is_none();
            // For status purposes, Indeterminate paints with the "checked"
            // surface (the icon shows; nova styles `data-state=indeterminate`
            // and `data-state=checked` identically).
            let is_checked = self.state.is_marked();

            if is_disabled {
                Status::Disabled { is_checked }
            } else if is_mouse_over {
                Status::Hovered { is_checked }
            } else {
                Status::Active { is_checked }
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
        if cursor.is_over(layout.bounds()) && self.on_toggle.is_some() {
            mouse::Interaction::Pointer
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
        let mut children = layout.children();

        let style = theme.style(
            &self.class,
            self.last_status.unwrap_or(Status::Disabled {
                is_checked: self.state.is_marked(),
            }),
        );

        {
            let layout = children.next().unwrap();
            let bounds = layout.bounds();

            // Ring: paint a halo *outside* the box bounds, behind the main
            // fill, when `style.ring` is `Some`. Mirrors the text_input
            // painter (see `widget/src/text_input/widget.rs` lines 503-516).
            // `border.width = ring.width` paints inward from the expanded
            // bounds, so the visible halo wraps around the box.
            if let Some(ring) = style.ring {
                let r = ring.width;
                let mut radius = style.border.radius;
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
                    border: style.border,
                    ..renderer::Quad::default()
                },
                style.background,
            );

            let Icon {
                font,
                code_point,
                size,
                line_height,
                shaping,
            } = &self.icon;
            let size = size.unwrap_or(Pixels(bounds.height * 0.7));

            match self.state {
                State::Checked => {
                    renderer.fill_text(
                        text::Text {
                            content: code_point.to_string(),
                            font: *font,
                            size,
                            line_height: *line_height,
                            bounds: bounds.size(),
                            align_x: text::Alignment::Center,
                            align_y: alignment::Vertical::Center,
                            shaping: *shaping,
                            wrapping: text::Wrapping::default(),
                            ellipsis: text::Ellipsis::default(),
                            letter_spacing: Em::default(),
                            font_features: Vec::new(),
                            font_variations: Vec::new(),
                            weight: None,
                            hint_factor: None,
                        },
                        bounds.center(),
                        style.icon_color,
                        *viewport,
                    );
                }
                State::Indeterminate => {
                    // Horizontal stroke in the center of the box, mirroring
                    // shadcn's `MinusIcon` for `data-state="indeterminate"`.
                    // Width: 50% of the box; height: ~10% (1.5px on a 16px
                    // box at default sizing).
                    let stroke_w = bounds.width * 0.5;
                    let stroke_h = (bounds.height * 0.1).max(1.0);
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: bounds.center_x() - stroke_w / 2.0,
                                y: bounds.center_y() - stroke_h / 2.0,
                                width: stroke_w,
                                height: stroke_h,
                            },
                            ..renderer::Quad::default()
                        },
                        Background::Color(style.icon_color),
                    );
                }
                State::Unchecked => {}
            }
        }

        if self.label.is_none() {
            return;
        }

        {
            let label_layout = children.next().unwrap();
            let state: &text_helpers::State<Renderer::Paragraph> = tree.state.downcast_ref();

            // **Deviation from upstream iced**: stock iced says
            // `crate::text::draw` / `crate::text::Style` because its widget
            // crate re-exports `core::widget::text::*` at `crate::text`. Our
            // `crate::text` is a local typography module; we route through
            // `iced_widget::text::{draw, Style}` (which DOES re-export those)
            // via `use` imports below to keep call-site call paths shallow.
            // Functionally identical.
            text_helpers::draw(
                renderer,
                defaults,
                label_layout.bounds(),
                state.raw(),
                text_helpers::Style {
                    color: style.text_color,
                },
                viewport,
            );
        }
    }

    fn operate(
        &mut self,
        _tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        if let Some(label) = self.label.as_deref() {
            operation.text(None, layout.bounds(), label);
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Checkbox<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + text::Renderer,
{
    fn from(
        checkbox: Checkbox<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(checkbox)
    }
}

/// The icon in a [`Checkbox`].
#[derive(Debug, Clone, PartialEq)]
pub struct Icon<Font> {
    /// Font that will be used to display the `code_point`,
    pub font: Font,
    /// The unicode code point that will be used as the icon.
    pub code_point: char,
    /// Font size of the content.
    pub size: Option<Pixels>,
    /// The line height of the icon.
    pub line_height: text::LineHeight,
    /// The shaping strategy of the icon.
    pub shaping: text::Shaping,
}

/// The check / mark state of a [`Checkbox`].
///
/// **Deviation from upstream iced**: stock iced represents the check as a
/// `bool`; we use a domain enum so `Indeterminate` (matching shadcn /
/// radix's `data-state="indeterminate"`) is a first-class state. The
/// `From<bool> for State` impl preserves the bool-shaped call site:
/// `checkbox(self.is_checked)` keeps working.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Checked. Renders the check glyph.
    Checked,
    /// Unchecked. Renders an empty box.
    Unchecked,
    /// Indeterminate. Renders a horizontal stroke (the `MinusIcon` shape
    /// shadcn uses for `data-state="indeterminate"`).
    Indeterminate,
}

impl State {
    /// Whether the box renders ANY mark (check OR indeterminate stroke).
    /// Used for catalog `Status` dispatch — nova styles `data-state=indeterminate`
    /// and `data-state=checked` identically (both show the primary surface).
    pub fn is_marked(self) -> bool {
        matches!(self, State::Checked | State::Indeterminate)
    }
}

impl From<bool> for State {
    fn from(b: bool) -> Self {
        if b { State::Checked } else { State::Unchecked }
    }
}

/// The possible status of a [`Checkbox`].
/// **Deviation from upstream iced**: `Status` is re-exported from
/// `icedxdy_core` so the whole crate (theme + widget + wrapper) speaks one
/// canonical type. Variants match upstream 1:1 so the rest of this file's
/// pattern matches keep working.
pub use icedxdy_core::theme::checkbox::Status;

/// A focus ring rendered around the checkbox — a solid colored halo of
/// `width` pixels painted behind the main fill. Equivalent to CSS
/// `box-shadow: 0 0 0 {width}px {color}` (Tailwind `ring-*`).
///
/// **Deviation from upstream iced**: this widget renders a focus ring in
/// `draw()` when `Style::ring` is `Some`. Iced's stock `checkbox::Style` has
/// no ring field; we add it here so our nova-correct focus halo
/// (`focus-visible:ring-3 ring-ring/50`) renders.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    /// Halo color (already alpha-resolved by the catalog).
    pub color: Color,
    /// Halo thickness in pixels.
    pub width: f32,
}

/// The style of a checkbox.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Background`] of the checkbox.
    pub background: Background,
    /// The icon [`Color`] of the checkbox.
    pub icon_color: Color,
    /// The [`Border`] of the checkbox.
    pub border: Border,
    /// The text [`Color`] of the checkbox.
    pub text_color: Option<Color>,
    /// Optional focus ring (Tailwind `ring-*`). `None` = no halo rendered.
    pub ring: Option<Ring>,
}

/// The theme catalog of a [`Checkbox`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Checkbox`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// A primary checkbox; denoting a main toggle.
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    match status {
        Status::Active { is_checked } => styled(
            palette.background.strong.color,
            palette.background.base,
            palette.primary.base.text,
            palette.primary.base,
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            palette.background.strong.color,
            palette.background.weak,
            palette.primary.base.text,
            palette.primary.strong,
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            palette.background.weak.color,
            palette.background.weaker,
            palette.primary.base.text,
            palette.background.strong,
            is_checked,
        ),
    }
}

/// A secondary checkbox; denoting a complementary toggle.
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    match status {
        Status::Active { is_checked } => styled(
            palette.background.strong.color,
            palette.background.base,
            palette.background.base.text,
            palette.background.strong,
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            palette.background.strong.color,
            palette.background.weak,
            palette.background.base.text,
            palette.background.strong,
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            palette.background.weak.color,
            palette.background.weak,
            palette.background.base.text,
            palette.background.weak,
            is_checked,
        ),
    }
}

/// A success checkbox; denoting a positive toggle.
pub fn success(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    match status {
        Status::Active { is_checked } => styled(
            palette.background.weak.color,
            palette.background.base,
            palette.success.base.text,
            palette.success.base,
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            palette.background.strong.color,
            palette.background.weak,
            palette.success.base.text,
            palette.success.strong,
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            palette.background.weak.color,
            palette.background.weak,
            palette.success.base.text,
            palette.success.weak,
            is_checked,
        ),
    }
}

/// A danger checkbox; denoting a negative toggle.
pub fn danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    match status {
        Status::Active { is_checked } => styled(
            palette.background.strong.color,
            palette.background.base,
            palette.danger.base.text,
            palette.danger.base,
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            palette.background.strong.color,
            palette.background.weak,
            palette.danger.base.text,
            palette.danger.strong,
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            palette.background.weak.color,
            palette.background.weak,
            palette.danger.base.text,
            palette.danger.weak,
            is_checked,
        ),
    }
}

fn styled(
    border_color: Color,
    base: palette::Pair,
    icon_color: Color,
    accent: palette::Pair,
    is_checked: bool,
) -> Style {
    let (background, border) = if is_checked {
        (accent, accent.color)
    } else {
        (base, border_color)
    };

    Style {
        background: Background::Color(background.color),
        icon_color,
        border: Border {
            radius: 2.0.into(),
            width: 1.0,
            color: border,
        },
        text_color: None,
        ring: None,
    }
}

/// Convert from iced's stock `checkbox::Style` into the vendored widget's
/// `Style`. Lossless in this direction (iced has no `ring` field, so we set
/// `ring: None`). Used by `Catalog for iced_widget::Theme` to satisfy callers
/// that pick stock iced::Theme instead of `icedxdy_core::Theme`.
impl From<iced_widget::checkbox::Style> for Style {
    fn from(s: iced_widget::checkbox::Style) -> Self {
        Style {
            background: s.background,
            icon_color: s.icon_color,
            border: s.border,
            text_color: s.text_color,
            ring: None,
        }
    }
}
