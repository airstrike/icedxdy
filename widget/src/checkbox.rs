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
//! // Uncontrolled: the checkbox keeps its own value.
//! checkbox().default_checked(true)
//!
//! // Controlled: the app owns the value.
//! checkbox()
//!     .checked(self.is_checked)
//!     .on_checked_change(Message::ToggleTerms)
//! ```

pub(crate) mod element;
pub mod widget;

pub use widget::{Catalog, Ring, State, Status, Style, StyleFn};

use crate::compose::{Compose, Inherited, Slot};
use iced_core::Length;
use iced_core::text::IntoFragment;
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme;

use self::element::from_core;

/// A styled checkbox matching shadcn/ui's Checkbox component.
///
/// Radix state: uncontrolled with [`default_checked`](Self::default_checked)
/// (the value lives in the widget tree), or controlled with
/// [`checked`](Self::checked) plus
/// [`on_checked_change`](Self::on_checked_change). It is enabled unless
/// [`disabled`](Self::disabled) or a disabled ancestor says otherwise.
pub struct Checkbox<'a, Message> {
    checked: Option<State>,
    default_checked: State,
    disabled: bool,
    label: Option<iced_core::text::Fragment<'a>>,
    width: Length,
    on_checked_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style: Option<theme::checkbox::StyleFn<'a, Theme>>,
}

/// Create an unchecked, uncontrolled [`Checkbox`].
pub fn checkbox<'a, Message: 'a>() -> Checkbox<'a, Message> {
    Checkbox {
        checked: None,
        default_checked: State::Unchecked,
        disabled: false,
        label: None,
        width: Length::Shrink,
        on_checked_change: None,
        style: None,
    }
}

impl<'a, Message: 'a> Checkbox<'a, Message> {
    /// Control the checkbox: it always shows `checked`, and a click only
    /// publishes [`on_checked_change`](Self::on_checked_change).
    ///
    /// Accepts a [`State`] (`Checked` / `Unchecked` / `Indeterminate`) or a
    /// `bool`.
    pub fn checked(mut self, checked: impl Into<State>) -> Self {
        self.checked = Some(checked.into());
        self
    }

    /// The initial state of an uncontrolled checkbox. Later rebuilds keep the
    /// value the user toggled to.
    pub fn default_checked(mut self, checked: impl Into<State>) -> Self {
        self.default_checked = checked.into();
        self
    }

    /// Set the message published with the new checked state on every toggle.
    pub fn on_checked_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_checked_change = Some(Box::new(on_change));
        self
    }

    /// Disable the checkbox.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the optional inline label.
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
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
        let cb: widget::Checkbox<'a, Message, Theme, Renderer> = match c.checked {
            Some(checked) => widget::Checkbox::new(checked),
            None => widget::Checkbox::new(State::Unchecked).uncontrolled(c.default_checked),
        };
        let mut cb = cb.width(c.width).disabled(c.disabled);

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

        if let Some(on_change) = c.on_checked_change {
            cb = cb.on_toggle(on_change);
        }

        cb.into()
    }
}

impl<'a, Message: 'a> From<Checkbox<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Checkbox<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Checkbox<'a, Message> {
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
