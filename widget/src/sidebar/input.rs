//! SidebarInput: h-8 text input with sidebar styling

use crate::compose::Slot;
use crate::container::Container;
use iced_core::Length::Fill;
use iced_widget::Widget;
use icedxdy_core::Theme;

/// Create a sidebar text input.
///
/// A text input styled for sidebar use — h-8 (32px) height.
/// Takes a placeholder string and the current value.
pub fn input<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Container<'a, Message> {
    let text_input: iced_core::Element<'a, Message, Theme, iced_widget::Renderer> =
        iced_widget::text_input(placeholder, value)
            .on_input(on_input)
            .size(14.0)
            .boxed();
    Container::new(Slot::from(text_input))
        .width(Fill)
        .height(32)
}
