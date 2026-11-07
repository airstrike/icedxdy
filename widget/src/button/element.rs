use iced_core::Length;
use iced_core::Length::Fill;
use iced_core::Padding;
use iced_core::alignment;
use iced_core::text::Span;
use iced_widget::Widget;
use iced_widget::text::Rich;

use crate::Size;
use crate::compose::{Compose, Inherited, Slot};
use icedxdy_core::Theme;
use icedxdy_core::theme;

use super::widget;

fn default(theme: &Theme, status: widget::Status) -> widget::Style {
    // theme::button::default returns our OKLCH-native Style; convert
    // through iced::button::Style → widget::Style.
    let iced: iced_widget::button::Style = theme::button::default(theme, status.into()).into();
    iced.into()
}

impl widget::Catalog for Theme {
    type Class<'a> = widget::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: widget::Status) -> widget::Style {
        class(self, status)
    }
}

/// A button that wraps our [`Element`] system.
///
/// Holds a [`Size`] which is propagated to its content when converting
/// to an `iced::Element`. This means a `Size::Lg` button automatically
/// gets larger text, larger icons, etc.
pub struct Button<'a, Message, Theme = icedxdy_core::Theme, Renderer = iced_widget::Renderer>
where
    Renderer: iced_core::Renderer,
    Theme: iced_widget::button::Catalog,
{
    content: Slot<'a, Message, Theme, Renderer>,
    size: Option<Size>,
    weight: crate::Weight,
    square: bool,
    rounded: bool,
    translate: bool,
    inset: Padding,
    interaction: Option<iced_core::mouse::Interaction>,
    width: Option<Length>,
    style: Theme::Class<'a>,
    on_press: Option<Message>,
    disabled: bool,
}

/// Create a button with default size and style
pub fn button<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Button<'a, Message> {
    Button {
        content: content.into(),
        size: None,
        weight: crate::Weight::Medium,
        square: false,
        rounded: false,
        translate: true,
        inset: Padding::ZERO,
        interaction: None,
        width: None,
        style: Box::new(|t, s| theme::button::default(t, s).into()),
        on_press: None,
        disabled: false,
    }
}

impl<'a, Message: Clone + 'a> Button<'a, Message> {
    /// Set the size variant
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Make the button square (equal width and height).
    pub fn square(mut self) -> Self {
        self.square = true;
        self
    }

    /// Make the button fully rounded (pill/circle shape).
    pub fn rounded(mut self) -> Self {
        self.rounded = true;
        self
    }

    /// Whether to translate down 1px on press. Defaults to `true`.
    ///
    /// Set to `false` for sidebar menu items or other elements styled
    /// as buttons that shouldn't have a press-down effect.
    pub fn translate(mut self, translate: bool) -> Self {
        self.translate = translate;
        self
    }

    /// Set the mouse interaction (cursor) when hovering.
    pub fn interaction(mut self, interaction: iced_core::mouse::Interaction) -> Self {
        self.interaction = Some(interaction);
        self
    }

    /// Set a transparent inset between the button's bounds (hit-test +
    /// hover region) and the painted bg/border. Used by sidebar items so
    /// adjacent buttons in a `spacing(0)` column hover continuously while
    /// the visible fills sit apart by `inset.top + inset.bottom`.
    pub fn inset(mut self, inset: impl Into<Padding>) -> Self {
        self.inset = inset.into();
        self
    }

    /// Set the layout width. Defaults to the vendored widget's intrinsic
    /// shrink-to-content sizing; pass `iced::Fill` to stretch (used by
    /// sidebar nav items so the hover/active surface spans the row).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Set the font weight
    pub fn weight(mut self, weight: impl Into<crate::Weight>) -> Self {
        self.weight = weight.into();
        self
    }

    /// Set the style
    pub fn style(mut self, style: impl Into<iced_widget::button::StyleFn<'a, Theme>>) -> Self {
        self.style = style.into();
        self
    }

    /// The button's size-driven height in pixels. Read by [`button::group`]
    /// to compute the cluster's tallest button so separators size their
    /// perpendicular axis to a concrete pixel value rather than `Length::Fill`.
    ///
    /// Mirrors the height the button will actually render at — same source
    /// as [`Size::height`] used inside the `From<Button> for Element` impl.
    ///
    /// [`button::group`]: crate::button::group
    pub fn size_height(&self) -> f32 {
        self.size.unwrap_or_default().height()
    }

    /// Wrap the existing style fn with a post-processor that gets to mutate
    /// the resolved [`iced_widget::button::Style`] before the bridge to our
    /// vendored widget. Used by [`button::group`] to selectively zero out
    /// edge-button corner radii at iteration time, after the user's style
    /// closure has run — matches shadcn's `[&>*:not(:first-child)]:rounded-l-none`
    /// child-position cascade, which can only be applied once the parent
    /// knows the child's position in the group.
    ///
    /// [`button::group`]: crate::button::group
    pub fn map_style<F>(mut self, transform: F) -> Self
    where
        F: Fn(iced_widget::button::Style) -> iced_widget::button::Style + 'a,
    {
        use iced_widget::button;
        let prev = std::mem::replace(&mut self.style, Box::new(|_, _| button::Style::default()));
        self.style = Box::new(move |t, s| transform(prev(t, s)));
        self
    }

    /// Set the message published when the button is pressed. A button
    /// without one stays enabled and publishes nothing; use
    /// [`disabled`](Self::disabled) to disable it.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Set the message published when the button is pressed, if `Some`.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Disable the button: it ignores input, is skipped by focus traversal
    /// and paints with the variant's disabled style (nova `disabled:opacity-50`).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>>
    for iced_core::Element<'a, Message, Theme, iced_widget::Renderer>
{
    fn from(mut btn: Button<'a, Message>) -> Self {
        let size = btn.size.unwrap_or_default();
        btn.content
            .inherit(&Inherited::default().with_size(size).with_weight(btn.weight));

        let content = btn.content.compose();

        let h = size.height();
        let mut iced_btn = if btn.square {
            super::widget::Button::new(iced_widget::center(content))
                .width(h)
                .height(h)
                .padding(0)
        } else {
            // shadcn buttons are `flex items-center justify-center` — content
            // is centered both axes. iced's `Button` doesn't itself center
            // children; padded layout puts content at the start. When the
            // user calls `.width(Fill)` (Card "Login" / "Login with Google"
            // etc.), wrap content in a `Fill`-width container with
            // `align_x(Center)` so the slack between the button's inner
            // bounds and the content centers the latter — matches shadcn.
            //
            // Intrinsic-sized buttons (no `.width()` call) are left alone:
            // there's no slack, centering would be a no-op, and any wrapper
            // would change the size_hint that the vendored Button uses for
            // its `fluid()` width seeding.
            let inner: iced_core::Element<'a, Message, Theme, iced_widget::Renderer> =
                if matches!(btn.width, Some(Fill)) {
                    iced_widget::container(content)
                        .width(Fill)
                        .align_x(alignment::Horizontal::Center)
                        .boxed()
                } else {
                    content
                };
            widget::Button::new(inner).height(h).padding(size.padding())
        };

        iced_btn = iced_btn.translate(btn.translate).inset(btn.inset);
        if let Some(interaction) = btn.interaction {
            iced_btn = iced_btn.interaction(interaction);
        }
        // `square` already set both width and height to the size's height;
        // skip the user width override in that case so the square invariant
        // holds.
        if let Some(width) = btn.width
            && !btn.square
        {
            iced_btn = iced_btn.width(width);
        }

        let rounded = btn.rounded;
        let bridge: super::widget::StyleFn<'a, Theme> =
            Box::new(move |t: &Theme, s: super::widget::Status| {
                let iced_status: iced_widget::button::Status = s.into();
                let mut style: super::widget::Style = (btn.style)(t, iced_status).into();
                if rounded {
                    style.border.radius = (u32::MAX as f32).into();
                }
                style
            });
        iced_btn = iced_btn.style(bridge);

        if let Some(on_press) = btn.on_press {
            iced_btn = iced_btn.on_press(on_press);
        }

        iced_btn.disabled(btn.disabled).into()
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Button<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Button<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.size = self.size.or(from.size());
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, iced_widget::Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}

/// A link widget that looks like a text link with underline on hover
pub struct Link {
    label: String,
}

impl Link {
    pub fn new(label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
        }
    }

    pub fn on_press<'a, Message: Clone + 'a>(
        self,
        message: Message,
    ) -> Rich<'a, Message, Message, Theme> {
        let span: Span<'a, Message> = Span::new(self.label).link(message);
        Rich::with_spans([span])
            .on_link_click(|msg| msg)
            .style(|t| theme::text::link(t).into())
    }
}
