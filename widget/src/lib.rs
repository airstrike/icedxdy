use iced_core as core;

pub mod button;
pub mod column;
pub mod component;
pub mod container;
pub mod operation;
pub mod row;
pub mod rule;
pub mod separator;
pub mod size;
#[cfg(feature = "spinner")]
pub mod spinner;
pub mod text;
pub mod weight;

pub use button::Button;
pub use column::Column;
pub use component::{Component, IntoElement};
pub use row::Row;
pub use size::Size;
pub use text::Text;
pub use weight::Weight;

/// Create a [`Button`] with the given content and default style.
pub fn button<'a, Message: Clone + 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Button<'a, Message> {
    button::button(content)
}

/// Create a [`Text`] widget.
pub fn text<'a>(content: impl iced_core::text::IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
}

/// Create a [`Column`] with the given children.
pub fn column<'a, Message: 'a>(
    children: impl IntoIterator<Item = component::Component<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children)
}

/// Create a [`Container`] wrapping the given content.
pub fn container<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> container::Container<'a, Message> {
    container::Container::new(content)
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
