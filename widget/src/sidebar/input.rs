//! SidebarInput: h-8 text input with sidebar styling

use crate::container::Container;
use crate::element::Element;
use iced_core::Length;
use icedxdy_core::Theme;

/// Create a sidebar text input.
///
/// A text input styled for sidebar use — h-8 (32px) height.
/// Takes a placeholder string and the current value.
pub fn input<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Container<'a, Message> {
    let text_input: iced_core::Element<'a, Message, Theme, iced_widget::Renderer> =
        iced_widget::text_input(placeholder, value)
            .on_input(on_input)
            .size(14.0)
            .into();
    let element = Element::from(text_input);
    Container::new(element)
        .width(Length::Fill)
        .height(Length::Fixed(32.0))
}
