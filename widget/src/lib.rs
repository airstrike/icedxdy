use iced_core as core;

pub mod button;
pub mod element;
pub mod row;
pub mod size;
#[cfg(feature = "spinner")]
pub mod spinner;
pub mod text;
pub mod weight;

pub use button::Button;
pub use element::{Element, IntoElement};
pub use row::Row;
pub use size::Size;
pub use text::Text;
pub use weight::Weight;

/// Create a [`Button`] with the given content and default style.
pub fn button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    button::button(content)
}

/// Create a [`Text`] widget.
pub fn text<'a>(content: impl iced_core::text::IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
}

/// Create a [`Row`].
pub fn row<'a, Message: 'a>() -> Row<'a, Message> {
    Row::new()
}

/// Create a [`Spinner`].
#[cfg(feature = "spinner")]
pub fn spinner<'a>() -> spinner::Spinner<'a> {
    spinner::Spinner::new()
}
