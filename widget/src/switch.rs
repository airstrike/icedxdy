//! Switch component — shadcn/ui faithful port
//!
//! A pill-shaped on/off control with an animated thumb slide. Wraps our
//! vendored `switch::widget::Toggler` (sweetened from sweeten + a focus
//! ring painter) so `.style(...)` can carry the focus halo (Tailwind
//! `ring-*`) end-to-end.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::switch;
//!
//! // Uncontrolled: the switch keeps its own value.
//! switch().label("Airplane mode")
//!
//! // Controlled: the app owns the value.
//! switch()
//!     .checked(self.is_on)
//!     .on_checked_change(Message::ToggleAirplane)
//! ```

pub(crate) mod element;
pub mod widget;

pub use widget::{Catalog, Ring, Status, Style, StyleFn};

use crate::compose::{Compose, Inherited, Slot};
use iced_core::Length;
use iced_core::text::IntoFragment;
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme;

use self::element::from_core;

/// A styled switch matching shadcn/ui's Switch component.
///
/// Radix state: uncontrolled with [`default_checked`](Self::default_checked)
/// (the value lives in the widget tree), or controlled with
/// [`checked`](Self::checked) plus
/// [`on_checked_change`](Self::on_checked_change). It is enabled unless
/// [`disabled`](Self::disabled) or a disabled ancestor says otherwise.
pub struct Switch<'a, Message> {
    checked: Option<bool>,
    default_checked: bool,
    disabled: bool,
    label: Option<iced_core::text::Fragment<'a>>,
    width: Length,
    on_checked_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style: Option<theme::switch::StyleFn<'a, Theme>>,
}

/// Create an off, uncontrolled [`Switch`].
pub fn switch<'a, Message: 'a>() -> Switch<'a, Message> {
    Switch {
        checked: None,
        default_checked: false,
        disabled: false,
        label: None,
        width: Length::Shrink,
        on_checked_change: None,
        style: None,
    }
}

impl<'a, Message: 'a> Switch<'a, Message> {
    /// Control the switch: it always shows `checked`, and a click only
    /// publishes [`on_checked_change`](Self::on_checked_change).
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// The initial state of an uncontrolled switch. Later rebuilds keep the
    /// value the user toggled to.
    pub fn default_checked(mut self, checked: bool) -> Self {
        self.default_checked = checked;
        self
    }

    /// Set the message published with the new checked state on every toggle.
    pub fn on_checked_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_checked_change = Some(Box::new(on_change));
        self
    }

    /// Disable the switch.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the optional inline label.
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Set the switch's outer width. Defaults to `Length::Shrink`.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Override the switch's style. Accepts our OKLCH-native variants
    /// (`Default`, `Invalid`, `Disabled`) or any `Spec`. Ring is preserved
    /// across the crate boundary by the `from_core` bridge in this widget's
    /// `element` submodule.
    pub fn style(mut self, style: impl Into<theme::switch::StyleFn<'a, Theme>>) -> Self {
        self.style = Some(style.into());
        self
    }
}

impl<'a, Message: 'a> From<Switch<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(s: Switch<'a, Message>) -> Self {
        let tg: widget::Toggler<'a, Message, Theme, Renderer> = match s.checked {
            Some(checked) => widget::Toggler::new(checked),
            None => widget::Toggler::new(false).uncontrolled(s.default_checked),
        };
        let mut tg = tg.width(s.width).disabled(s.disabled);

        if let Some(label) = s.label {
            tg = tg.label(label);
        }

        if let Some(style) = s.style {
            // Wrap the core StyleFn so it produces widget::Style (with ring).
            // Status types match across crates (vendored re-exports core's),
            // so no Status conversion is needed here.
            let bridge: widget::StyleFn<'a, Theme> =
                Box::new(move |theme, status| from_core(style(theme, status)));
            tg = tg.class(bridge);
        }

        if let Some(on_change) = s.on_checked_change {
            tg = tg.on_toggle(on_change);
        }

        tg.into()
    }
}

impl<'a, Message: 'a> From<Switch<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Switch<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Switch<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
