//! Toggle — shadcn/ui faithful port (Radix `Toggle`).
//!
//! A two-state button: transparent at rest, `bg-muted` while on.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::toggle;
//!
//! // Uncontrolled: the toggle keeps its own value.
//! toggle(icon::bookmark()).default_pressed(true)
//!
//! // Controlled: the app owns the value.
//! toggle(icon::bold())
//!     .pressed(self.bold)
//!     .on_pressed_change(Message::Bold)
//!     .style(toggle::Outline)
//! ```
//!
//! `toggle![icon::italic(), "Italic"]` lays an icon and a label out with
//! the nova `gap-1` (4px).
//!
//! # Variants
//!
//! shadcn's `variant` prop is a style: `.style(toggle::Outline)`.

mod widget;

use iced_core::alignment::{Horizontal, Vertical};
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;
use icedxdy_core::theme::button::Spec;

use crate::compose::{Compose, Inherited, Slot};
use crate::{Size, Weight};

pub use icedxdy_core::theme::toggle::{Default, Outline};

/// Create a toggle with one or more children.
///
/// - `toggle!(content)` defers to [`toggle()`].
/// - `toggle![child1, child2, ...]` wraps the children in a row with the
///   nova `gap-1` (4px).
/// - `toggle![italic, "Italic"]` reads a bare leading identifier as
///   `icon::italic()`, as `button!` does.
#[macro_export]
macro_rules! toggle {
    ($single:expr $(,)?) => {
        $crate::toggle::toggle($single)
    };
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::toggle::toggle(
            $crate::compose::Slot::new(
                $crate::row::Row::with_children([
                    Into::<$crate::compose::Slot<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::compose::Slot<'_, _>>::into($rest)),+
                ])
                .spacing(4)
            )
        )
    };
    ($($child:expr),+ $(,)?) => {
        $crate::toggle::toggle(
            $crate::compose::Slot::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::compose::Slot<'_, _>>::into($child)),+
                ])
                .spacing(4)
            )
        )
    };
}

/// A button that is either on or off.
///
/// Radix state: uncontrolled with [`default_pressed`](Self::default_pressed),
/// or controlled with [`pressed`](Self::pressed) plus
/// [`on_pressed_change`](Self::on_pressed_change). It is enabled unless
/// [`disabled`](Self::disabled) or a disabled ancestor says otherwise.
pub struct Toggle<'a, Message> {
    content: Slot<'a, Message>,
    pressed: Option<bool>,
    default_pressed: bool,
    on_pressed_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    disabled: bool,
    size: Option<Size>,
    style: Spec,
}

/// Create an off, uncontrolled [`Toggle`] showing `content`.
pub fn toggle<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Toggle<'a, Message> {
    Toggle {
        content: content.into(),
        pressed: None,
        default_pressed: false,
        on_pressed_change: None,
        disabled: false,
        size: None,
        style: Default.into(),
    }
}

impl<'a, Message: 'a> Toggle<'a, Message> {
    /// Control the toggle: it always shows `pressed`, and a press only
    /// publishes [`on_pressed_change`](Self::on_pressed_change).
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = Some(pressed);
        self
    }

    /// The initial value of an uncontrolled toggle. Later rebuilds keep the
    /// value the user toggled to.
    pub fn default_pressed(mut self, pressed: bool) -> Self {
        self.default_pressed = pressed;
        self
    }

    /// Set the message published with the new value on every press.
    pub fn on_pressed_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_pressed_change = Some(Box::new(on_change));
        self
    }

    /// Disable the toggle.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the size: `Sm` (28px), `Default` (32px) or `Lg` (36px).
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set the style: a variant (`toggle::Outline`) or a tweaked `Spec`.
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Toggle<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.size = self.size.or(from.size());
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Toggle {
            mut content,
            pressed,
            default_pressed,
            on_pressed_change,
            disabled,
            size,
            style,
        } = *self;

        let size = size.unwrap_or_default();
        content.inherit(
            &Inherited::default()
                .with_size(size)
                .with_weight(Weight::Medium),
        );

        // `h-8 min-w-8 px-2.5` (sm: `h-7 min-w-7`, lg: `h-9 min-w-9`).
        let height = size.height();
        let surface = crate::container(content)
            .height(height)
            .min_width(height)
            .padding([0.0, size.padding().left])
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let surface: iced_core::Element<'a, Message, Theme, Renderer> = surface.into();

        widget::Toggle::new(surface, style)
            .pressed(pressed)
            .default_pressed(default_pressed)
            .on_change(on_pressed_change)
            .disabled(disabled)
            .boxed()
    }
}

impl<'a, Message: 'a> From<Toggle<'a, Message>> for Slot<'a, Message> {
    fn from(toggle: Toggle<'a, Message>) -> Self {
        Slot::new(toggle)
    }
}

impl<'a, Message: 'a> From<Toggle<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(toggle: Toggle<'a, Message>) -> Self {
        Box::new(toggle).compose()
    }
}
