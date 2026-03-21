pub use crate::button::{self, Button};
pub use crate::element::{self, Element, IntoElement};
pub use crate::size::Size;
pub use crate::text::{self, Text};

/// Create a [`Button`] with the given content and default style.
pub fn button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    crate::button::button(content)
}

/// Create a [`Text`] widget.
pub fn text<'a>(content: impl iced_core::text::IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
}
