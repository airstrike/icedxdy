use iced_core as core;

pub mod alert;
pub mod animation;
pub mod badge;
pub mod button;
pub mod column;
pub mod component;
pub mod container;
pub mod input;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod operation;
pub mod progress;
pub mod row;
pub mod rule;
pub mod separator;
pub mod size;
#[cfg(feature = "spinner")]
pub mod spinner;
pub mod text;
pub mod weight;

pub use iced_widget::space;

pub use alert::Alert;
pub use badge::Badge;
pub use button::Button;
pub use column::Column;
pub use component::{Component, IntoElement};
pub use input::Input;
pub use progress::Progress;
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

/// Create a [`Container`] that centers its content (both axes), filling
/// the available space. Mirrors `iced::widget::center` but returns our
/// wrapped Container so the tweak-aware `.style()` is available.
pub fn center<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> container::Container<'a, Message> {
    container::Container::new(content).center(iced_core::Length::Fill)
}

/// Create a [`Row`].
pub fn row<'a, Message: 'a>() -> Row<'a, Message> {
    Row::new()
}

/// Create a [`Badge`] with the given content.
pub fn badge<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Badge<'a, Message> {
    badge::badge(content)
}

/// Create an [`Alert`].
pub fn alert<'a, Message: 'a>() -> Alert<'a, Message> {
    alert::alert()
}

/// Create an [`Input`].
pub fn input<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    value: &'a str,
) -> Input<'a, Message> {
    input::input(placeholder, value)
}

/// Create a [`Progress`] bar with the given value (0..=100).
pub fn progress<'a, Message: 'a>(value: f32) -> Progress<'a, Message> {
    progress::progress(value)
}

/// Create a [`Spinner`].
#[cfg(feature = "spinner")]
pub fn spinner<'a>() -> spinner::Spinner<'a> {
    spinner::Spinner::new()
}
