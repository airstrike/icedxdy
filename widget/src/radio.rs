//! Radio — iced's radio button, linked to labels through the
//! [`control`](crate::operation::control) operations.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::radio;
//!
//! radio("Card", Method::Card, Some(self.method), Message::Method)
//! ```

mod widget;

use iced_core::Pixels;
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;

use crate::compose::{Compose, Slot};

/// A radio button bound to one value of a set.
///
/// Controlled only: it shows the app's `selected` value and publishes the
/// message for its own value when chosen, by a click or through a linked
/// label.
pub struct Radio<'a, Message> {
    radio: iced_widget::Radio<'a, Message, Theme>,
    message: Message,
    is_selected: bool,
}

/// Create a [`Radio`] for `value`, selected when `selected` holds it.
pub fn radio<'a, Message, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl Fn(V) -> Message,
) -> Radio<'a, Message>
where
    Message: Clone + 'a,
    V: Eq + Copy,
{
    Radio {
        radio: iced_widget::radio(label, value, selected, &on_click),
        message: on_click(value),
        is_selected: selected == Some(value),
    }
}

impl<'a, Message: Clone + 'a> Radio<'a, Message> {
    /// Set the gap between the button and its label.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.radio = self.radio.spacing(spacing);
        self
    }

    /// Set the diameter of the button.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.radio = self.radio.size(size);
        self
    }

    /// Set the label's text size.
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.radio = self.radio.text_size(text_size);
        self
    }
}

impl<'a, Message: Clone + 'a> From<Radio<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(radio: Radio<'a, Message>) -> Self {
        widget::Radio::new(radio.radio.boxed(), radio.message, radio.is_selected).boxed()
    }
}

impl<'a, Message: Clone + 'a> From<Radio<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Radio<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Radio<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
