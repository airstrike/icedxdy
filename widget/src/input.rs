//! Input component — shadcn/ui faithful port
//!
//! A styled text input field.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::input;
//!
//! input("Email", &self.email).on_input(Message::EmailChanged)
//! ```

use crate::Size;
use crate::component::{Component, IntoElement};
use icedxdy_core::Theme;
use iced_core::{Length, Padding};
use iced_widget::Renderer;

/// A styled text input field matching shadcn/ui's Input component.
pub struct Input<'a, Message> {
    placeholder: &'a str,
    value: &'a str,
    size: Size,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    disabled: bool,
}

/// Create an [`Input`] with the given placeholder and current value.
pub fn input<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    value: &'a str,
) -> Input<'a, Message> {
    Input {
        placeholder,
        value,
        size: Size::Default,
        on_input: None,
        on_submit: None,
        disabled: false,
    }
}

impl<'a, Message: Clone + 'a> Input<'a, Message> {
    /// Set the size variant.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the callback for when the input value changes.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Set the message to emit when the user presses Enter.
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Disable the input.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Input height based on size variant.
    /// Default: h-9 (36px), Sm: 32px, Lg: 40px
    fn height(&self) -> f32 {
        match self.size {
            Size::Sm => 32.0,
            Size::Default => 36.0,
            Size::Lg => 40.0,
            _ => 36.0,
        }
    }

    /// Text size based on size variant.
    /// Default/Sm: 14px (text-sm), Lg: 16px (text-base)
    fn text_size(&self) -> f32 {
        match self.size {
            Size::Lg => 16.0,
            _ => 14.0,
        }
    }

    /// Padding: py-1 px-3 → [4, 12]
    fn padding(&self) -> Padding {
        Padding {
            top: 4.0,
            bottom: 4.0,
            left: 12.0,
            right: 12.0,
        }
    }
}

impl<'a, Message: Clone + 'a> From<Input<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(input: Input<'a, Message>) -> Self {
        let height = input.height();
        let text_size = input.text_size();
        let padding = input.padding();

        let mut ti = iced_widget::text_input(input.placeholder, input.value)
            .padding(padding)
            .size(text_size)
            .width(Length::Fill);

        if !input.disabled {
            if let Some(on_input) = input.on_input {
                ti = ti.on_input(on_input);
            }
            if let Some(on_submit) = input.on_submit {
                ti = ti.on_submit(on_submit);
            }
        }

        // Wrap in a container to enforce the exact height
        iced_widget::container(ti)
            .height(height)
            .width(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

impl<'a, Message: Clone + 'a> From<Input<'a, Message>> for Component<'a, Message> {
    fn from(input: Input<'a, Message>) -> Self {
        Component::new(input)
    }
}

impl<'a, Message: Clone + 'a> IntoElement<'a, Message> for Input<'a, Message> {
    fn size(&mut self, size: Size) {
        // Only override if user didn't explicitly set
        if self.size == Size::Default {
            self.size = size;
        }
    }

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        (*self).into()
    }
}
