//! Input component — shadcn/ui faithful port
//!
//! A styled text input field. Wraps our vendored `text_input::TextInput` so
//! `.style()` can carry the focus ring (Tailwind `ring-*`) end-to-end.
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
use crate::text_input;
use crate::text_input::element::from_core;
use iced_core::{Length, Padding};
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme;

/// A styled text input field matching shadcn/ui's Input component.
///
/// Disabled state follows iced's convention: an input with no `.on_input(...)`
/// is disabled (you can't type into it). To toggle disabled at runtime, use
/// `.on_input_maybe(Option<Fn>)` — though our wrapper exposes only the
/// always-Some `.on_input(...)` builder, so the natural pattern is to omit
/// the call entirely.
pub struct Input<'a, Message> {
    placeholder: &'a str,
    value: &'a str,
    size: Size,
    width: Length,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    on_focus: Option<Message>,
    on_blur: Option<Message>,
    style: Option<theme::text_input::StyleFn<'a, Theme>>,
}

/// Create an [`Input`] with the given placeholder and current value.
pub fn input<'a, Message: Clone + 'a>(placeholder: &'a str, value: &'a str) -> Input<'a, Message> {
    Input {
        placeholder,
        value,
        size: Size::Default,
        width: Length::Shrink,
        on_input: None,
        on_submit: None,
        on_focus: None,
        on_blur: None,
        style: None,
    }
}

impl<'a, Message: Clone + 'a> Input<'a, Message> {
    /// Set the size variant.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the input's width. Defaults to `Length::Shrink` — shadcn's
    /// `<Input />` is `w-full` only because the parent `Field` lays children
    /// out as `flex flex-col`; the input itself doesn't force `w-full`. Set
    /// `Length::Fill` (or a numeric value) when the call site wants the input
    /// to occupy its parent.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
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

    /// Set the message to emit when the input gains focus.
    pub fn on_focus(mut self, message: Message) -> Self {
        self.on_focus = Some(message);
        self
    }

    /// Set the message to emit when the input loses focus.
    pub fn on_blur(mut self, message: Message) -> Self {
        self.on_blur = Some(message);
        self
    }

    /// Override the input's style. Accepts our OKLCH-native `Default` variant
    /// or any `Spec` (e.g. `Default + Bg(...) + focus(Ring(...))`). Ring is
    /// preserved across the crate boundary by the bridge in
    /// [`text_input::element`](crate::text_input::element).
    pub fn style(mut self, style: impl Into<theme::text_input::StyleFn<'a, Theme>>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Input height — delegates to `Size::height` so size variants stay in
    /// sync with buttons. Nova `h-8` (32px) is `Size::Default`.
    fn height(&self) -> f32 {
        self.size.height()
    }

    /// Text size based on size variant.
    /// Default/Sm: 14px (text-sm), Lg: 16px (text-base)
    fn text_size(&self) -> f32 {
        match self.size {
            Size::Lg => 16.0,
            _ => 14.0,
        }
    }

    /// Padding: nova `px-2.5 py-1` → `{4, 10, 4, 10}`.
    fn padding(&self) -> Padding {
        Padding {
            top: 4.0,
            bottom: 4.0,
            left: 10.0,
            right: 10.0,
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

        let mut ti: text_input::TextInput<'a, Message, Theme, Renderer> =
            text_input::TextInput::new(input.placeholder, input.value)
                .padding(padding)
                .size(text_size)
                .width(input.width)
                .height(height);

        if let Some(style) = input.style {
            // Wrap the core StyleFn so it produces widget::Style (with ring).
            // Status types match across crates (vendored re-exports core's),
            // so no Status conversion is needed here.
            let bridge: text_input::StyleFn<'a, Theme> =
                Box::new(move |theme, status| from_core(style(theme, status)));
            ti = ti.class(bridge);
        }

        if let Some(on_input) = input.on_input {
            ti = ti.on_input(on_input);
        }
        if let Some(on_submit) = input.on_submit {
            ti = ti.on_submit(on_submit);
        }
        if let Some(on_focus) = input.on_focus {
            ti = ti.on_focus(on_focus);
        }
        if let Some(on_blur) = input.on_blur {
            ti = ti.on_blur(on_blur);
        }

        ti.into()
    }
}

impl<'a, Message: Clone + 'a> From<Input<'a, Message>> for Component<'a, Message> {
    fn from(input: Input<'a, Message>) -> Self {
        Component::new(input)
    }
}

impl<'a, Message: Clone + 'a> IntoElement<Message> for Input<'a, Message> {
    fn size(&mut self, size: Size) {
        // Only override if user didn't explicitly set
        if self.size == Size::Default {
            self.size = size;
        }
    }

    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
