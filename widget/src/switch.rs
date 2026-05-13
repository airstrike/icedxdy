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
//! switch(self.is_on)
//!     .label("Airplane mode")
//!     .on_toggle(Message::ToggleAirplane)
//! ```

pub(crate) mod element;
pub mod widget;

pub use widget::{Catalog, Ring, Status, Style, StyleFn};

use crate::component::{Component, IntoElement};
use iced_core::Length;
use iced_core::text::IntoFragment;
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme;

use self::element::from_core;

/// A styled switch matching shadcn/ui's Switch component.
///
/// Disabled state follows iced's convention: a switch with no
/// `.on_toggle(...)` is disabled.
pub struct Switch<'a, Message> {
    is_toggled: bool,
    label: Option<iced_core::text::Fragment<'a>>,
    width: Length,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style: Option<theme::switch::StyleFn<'a, Theme>>,
}

/// Create a [`Switch`] with the given on/off state.
pub fn switch<'a, Message: 'a>(is_toggled: bool) -> Switch<'a, Message> {
    Switch {
        is_toggled,
        label: None,
        width: Length::Shrink,
        on_toggle: None,
        style: None,
    }
}

impl<'a, Message: 'a> Switch<'a, Message> {
    /// Set the optional inline label.
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Set the toggle callback. Without this call, the switch is disabled.
    pub fn on_toggle<F: Fn(bool) -> Message + 'a>(mut self, on_toggle: F) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// Set the toggle callback, if `Some`. `None` disables the switch.
    pub fn on_toggle_maybe<F: Fn(bool) -> Message + 'a>(mut self, on_toggle: Option<F>) -> Self {
        self.on_toggle = on_toggle.map(|f| Box::new(f) as _);
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
        let mut tg: widget::Toggler<'a, Message, Theme, Renderer> =
            widget::Toggler::new(s.is_toggled).width(s.width);

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

        if let Some(on_toggle) = s.on_toggle {
            tg = tg.on_toggle(on_toggle);
        }

        tg.into()
    }
}

impl<'a, Message: 'a> From<Switch<'a, Message>> for Component<'a, Message> {
    fn from(s: Switch<'a, Message>) -> Self {
        // Materialize the toggle message for !current_state BEFORE moving `s`
        // into Component::new. Field reads this via `toggle_target()` to wrap
        // the rendered label in mouse_area — matching shadcn's `<label for>`
        // semantic without any caller change.
        let toggle_target = s.on_toggle.as_ref().map(|f| f(!s.is_toggled));
        Component::new(s).with_toggle_target(toggle_target)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Switch<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
