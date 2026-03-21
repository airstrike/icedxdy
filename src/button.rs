//! Button components
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::button;
//!
//! button::primary("Submit").on_press(Message::Submit)
//! button::primary("Go").size(Size::Lg).on_press(Message::Go)
//! button::icon(icon::chevron_right()).on_press(Message::Next)
//! ```

mod helpers;
pub use helpers::*;

use iced_core::text::Span;
use iced_core::Font;
use iced_widget::text::Rich;

use crate::element::{Element, IntoElement};
use crate::theme;
use crate::Size;
use crate::Theme;

type StyleFn = fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style;

/// A button that wraps our [`Element`] system.
///
/// Holds a [`Size`] which is propagated to its content when converting
/// to an `iced::Element`. This means a `Size::Lg` button automatically
/// gets larger text, larger icons, etc.
pub struct Button<'a, Message, Theme = crate::Theme, Renderer = iced_widget::Renderer>
where
    Renderer: iced_core::Renderer,
    Theme: iced_widget::button::Catalog,
{
    content: Element<'a, Message, Theme, Renderer>,
    size: Size,
    style: Theme::Class<'a>,
    on_press: Option<Message>,
}

/// Create a button with default size and style
pub fn button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    Button {
        content: content.into(),
        size: Size::Default,
        style: Box::new(theme::button::default),
        on_press: None,
    }
}

impl<'a, Message: Clone + 'a> Button<'a, Message> {
    /// Set the size variant
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the style
    pub fn style(mut self, style: StyleFn) -> Self {
        self.style = Box::new(style);
        self
    }

    /// Set the on_press message
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>>
    for iced_core::Element<'a, Message, Theme, iced_widget::Renderer>
{
    fn from(mut btn: Button<'a, Message>) -> Self {
        // Propagate size to content
        btn.content.size(btn.size);

        // Build the iced button
        let size = btn.size;
        let mut iced_btn = iced_widget::button::Button::new(btn.content.into_iced())
            .padding(size.padding())
            .style(move |t, s| (btn.style)(t, s));

        if let Some(on_press) = btn.on_press {
            iced_btn = iced_btn.on_press(on_press);
        }

        iced_btn.into()
    }
}

// Our Button is also an IntoElement so it can be nested
impl<'a, Message: Clone + 'a> IntoElement<'a, Message> for Button<'a, Message> {
    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, iced_widget::Renderer> {
        (*self).into()
    }
}

/// A link widget that looks like a text link with underline on hover
pub struct Link {
    label: String,
}

impl Link {
    pub fn new(label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
        }
    }

    pub fn on_press<'a, Message: Clone + 'a>(
        self,
        message: Message,
    ) -> Rich<'a, Message, Message, Theme> {
        let span: Span<'a, Message, Font> = Span::new(self.label).link(message);
        Rich::with_spans([span])
            .on_link_click(|msg| msg)
            .style(theme::text::link)
    }
}
