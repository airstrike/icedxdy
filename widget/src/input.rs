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
//! // Uncontrolled: the input keeps what the user types.
//! input("Name").default_value("Pedro Duarte")
//!
//! // Controlled: the app owns the value.
//! input("Email").value(&self.email).on_input(Message::EmailChanged)
//! ```

use crate::Size;
use crate::compose::{Compose, Inherited, Slot};
use crate::text_input;
use crate::text_input::element::from_core;
use iced_core::text::{Fragment, IntoFragment};
use iced_core::{Length, Padding};
use iced_widget::Renderer;
use iced_widget::Widget;
use icedxdy_core::Theme;
use icedxdy_core::theme;

/// A styled text input field matching shadcn/ui's Input component.
///
/// React state: uncontrolled by default, keeping what the user types in the
/// widget tree (seeded by [`default_value`](Self::default_value)), or
/// controlled with [`value`](Self::value) plus [`on_input`](Self::on_input).
/// A controlled input without `on_input` is read-only. It is enabled unless
/// [`disabled`](Self::disabled) or a disabled ancestor says otherwise.
pub struct Input<'a, Message> {
    placeholder: Fragment<'a>,
    value: Option<Fragment<'a>>,
    default_value: Fragment<'a>,
    disabled: bool,
    size: Option<Size>,
    width: Length,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    on_focus: Option<Message>,
    on_blur: Option<Message>,
    style: Option<theme::text_input::StyleFn<'a, Theme>>,
}

/// Create an empty [`Input`] with the given placeholder.
///
/// Both the placeholder and the [`value`](Input::value) accept owned or
/// borrowed text, so an input can be built from state a view does not
/// outlive (an iced `Component`'s view, for one).
pub fn input<'a, Message: Clone + 'a>(placeholder: impl IntoFragment<'a>) -> Input<'a, Message> {
    Input {
        placeholder: placeholder.into_fragment(),
        value: None,
        default_value: Fragment::Borrowed(""),
        disabled: false,
        size: None,
        width: Length::Shrink,
        on_input: None,
        on_submit: None,
        on_focus: None,
        on_blur: None,
        style: None,
    }
}

impl<'a, Message: Clone + 'a> Input<'a, Message> {
    /// Control the input: it always shows `value`, and edits only publish
    /// [`on_input`](Self::on_input). Owned or borrowed.
    pub fn value(mut self, value: impl IntoFragment<'a>) -> Self {
        self.value = Some(value.into_fragment());
        self
    }

    /// The initial value of an uncontrolled input. Later rebuilds keep what
    /// the user typed.
    pub fn default_value(mut self, value: impl IntoFragment<'a>) -> Self {
        self.default_value = value.into_fragment();
        self
    }

    /// Disable the input: not focusable, not editable, muted.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the size variant.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());
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

    /// Set the message published with the new value on every edit.
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
        self.size.unwrap_or_default().height()
    }

    /// Text size based on size variant.
    /// Default/Sm: 14px (text-sm), Lg: 16px (text-base)
    fn text_size(&self) -> f32 {
        match self.size.unwrap_or_default() {
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

        let ti: text_input::TextInput<'a, Message, Theme, Renderer> = match &input.value {
            Some(value) => text_input::TextInput::new(&input.placeholder, value),
            None => text_input::TextInput::new(&input.placeholder, "")
                .uncontrolled(&input.default_value),
        };
        let mut ti = ti
            .disabled(input.disabled)
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

        ti.boxed()
    }
}

impl<'a, Message: Clone + 'a> From<Input<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Input<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Input<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.size = self.size.or(from.size());
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
