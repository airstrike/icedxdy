//! Textarea - shadcn/ui port over iced's `text_editor`.
//!
//! A multi-line field with the Input's look, at least 64px tall
//! (`min-h-16`) and growing with its text (`field-sizing-content`).
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::textarea;
//!
//! // Uncontrolled: the textarea keeps its own text.
//! textarea("Type your message here.")
//!
//! // Controlled: the app owns the text.
//! textarea("Type your message here.")
//!     .value(&self.message)
//!     .on_input(Message::MessageChanged)
//! ```
//!
//! # Variants
//!
//! `aria-invalid` is a style: `.style(textarea::Invalid)`.

mod widget;

use iced_core::Element;
use iced_widget::text_editor::StyleFn;
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;

use crate::compose::{Compose, Inherited, Slot};

pub use icedxdy_core::theme::textarea::{Default, Invalid};

/// A multi-line text field.
///
/// Radix-style state, like Input: uncontrolled with
/// [`default_value`](Self::default_value) (the text lives in the widget
/// tree), or controlled with [`value`](Self::value) plus
/// [`on_input`](Self::on_input).
pub struct Textarea<'a, Message> {
    placeholder: String,
    value: Option<String>,
    default_value: String,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    disabled: bool,
    min_height: f32,
    style: StyleFn<'a, Theme>,
}

/// Create an empty, uncontrolled [`Textarea`] showing `placeholder`.
pub fn textarea<'a, Message>(placeholder: impl Into<String>) -> Textarea<'a, Message> {
    Textarea {
        placeholder: placeholder.into(),
        value: None,
        default_value: String::new(),
        on_input: None,
        disabled: false,
        min_height: 64.0,
        style: Default.into(),
    }
}

impl<'a, Message> Textarea<'a, Message> {
    /// Control the textarea: it always shows `value`, and an edit only
    /// publishes [`on_input`](Self::on_input).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The initial text of an uncontrolled textarea.
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = value.into();
        self
    }

    /// Set the message published with the complete new text on every edit.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Disable the textarea.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the least height in logical pixels (`min-h-16`, 64px, by
    /// default).
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
        self
    }

    /// Set the style: `textarea::Invalid`, or any text editor style.
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Textarea<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Textarea {
            placeholder,
            value,
            default_value,
            on_input,
            disabled,
            min_height,
            style,
        } = *self;

        widget::Textarea::new(placeholder, style)
            .value(value)
            .default_value(default_value)
            .on_input(on_input)
            .disabled(disabled)
            .min_height(min_height)
            .boxed()
    }
}

impl<'a, Message: 'a> From<Textarea<'a, Message>> for Slot<'a, Message> {
    fn from(textarea: Textarea<'a, Message>) -> Self {
        Slot::new(textarea)
    }
}

impl<'a, Message: 'a> From<Textarea<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(textarea: Textarea<'a, Message>) -> Self {
        Box::new(textarea).compose()
    }
}
