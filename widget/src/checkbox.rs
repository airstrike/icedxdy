//! Checkbox component — shadcn/ui faithful port
//!
//! A 16×16 box that can be checked, unchecked, or indeterminate, with an
//! optional label. Wraps our vendored `checkbox::widget::Checkbox` so
//! `.style(...)` can carry the focus ring (Tailwind `ring-*`) end-to-end.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::checkbox;
//!
//! checkbox(self.is_checked)
//!     .label("Accept terms and conditions")
//!     .on_toggle(Message::ToggleTerms)
//! ```

pub(crate) mod element;
pub mod widget;

pub use widget::{Catalog, Ring, State, Status, Style, StyleFn};

use crate::component::{Component, IntoElement};
use iced_core::Length;
use iced_core::text::IntoFragment;
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme;

use self::element::from_core;

/// A styled checkbox matching shadcn/ui's Checkbox component.
///
/// Disabled state follows iced's convention: a checkbox with no
/// `.on_toggle(...)` is disabled.
pub struct Checkbox<'a, Message> {
    state: State,
    label: Option<iced_core::text::Fragment<'a>>,
    width: Length,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style: Option<theme::checkbox::StyleFn<'a, Theme>>,
}

/// Create a [`Checkbox`] in the given state.
///
/// Accepts a [`State`] (`Checked` / `Unchecked` / `Indeterminate`) or a
/// `bool` via the `From<bool> for State` impl.
pub fn checkbox<'a, Message: 'a>(state: impl Into<State>) -> Checkbox<'a, Message> {
    Checkbox {
        state: state.into(),
        label: None,
        width: Length::Shrink,
        on_toggle: None,
        style: None,
    }
}

impl<'a, Message: 'a> Checkbox<'a, Message> {
    /// Set the optional inline label.
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Set the toggle callback. Without this call, the checkbox is disabled.
    pub fn on_toggle<F: Fn(bool) -> Message + 'a>(mut self, on_toggle: F) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// Set the toggle callback, if `Some`. `None` disables the checkbox.
    pub fn on_toggle_maybe<F: Fn(bool) -> Message + 'a>(mut self, on_toggle: Option<F>) -> Self {
        self.on_toggle = on_toggle.map(|f| Box::new(f) as _);
        self
    }

    /// Set the checkbox's outer width. Defaults to `Length::Shrink`.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Override the checkbox's style. Accepts our OKLCH-native variants
    /// (`Default`, `Invalid`, `Disabled`) or any `Spec`. Ring is preserved
    /// across the crate boundary by the `from_core` bridge in this widget's
    /// `element` submodule.
    pub fn style(mut self, style: impl Into<theme::checkbox::StyleFn<'a, Theme>>) -> Self {
        self.style = Some(style.into());
        self
    }
}

impl<'a, Message: 'a> From<Checkbox<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(c: Checkbox<'a, Message>) -> Self {
        let mut cb: widget::Checkbox<'a, Message, Theme, Renderer> =
            widget::Checkbox::new(c.state).width(c.width);

        if let Some(label) = c.label {
            cb = cb.label(label);
        }

        if let Some(style) = c.style {
            // Wrap the core StyleFn so it produces widget::Style (with ring).
            // Status types match across crates (vendored re-exports core's),
            // so no Status conversion is needed here.
            let bridge: widget::StyleFn<'a, Theme> =
                Box::new(move |theme, status| from_core(style(theme, status)));
            cb = cb.class(bridge);
        }

        if let Some(on_toggle) = c.on_toggle {
            cb = cb.on_toggle(on_toggle);
        }

        cb.into()
    }
}

impl<'a, Message: 'a> From<Checkbox<'a, Message>> for Component<'a, Message> {
    fn from(c: Checkbox<'a, Message>) -> Self {
        // Materialize the toggle message for !current_state BEFORE moving `c`
        // into Component::new. Field reads this via `toggle_target()` to wrap
        // the rendered label in mouse_area — matching shadcn's `<label for>`
        // semantic without any caller change.
        let toggle_target = c.on_toggle.as_ref().map(|f| f(!c.state.is_marked()));
        Component::new(c).with_toggle_target(toggle_target)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Checkbox<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
