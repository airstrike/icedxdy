//! SidebarInset: main content area, flex-1

use crate::container::Container;
use crate::element::Element;
use iced_core::Length::Fill;

/// Create a sidebar inset container.
///
/// The main content area next to the sidebar. Takes full width and height
/// (flex-1 equivalent), used as the primary content region.
pub fn inset<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    Container::new(content)
        .width(Fill)
        .height(Fill)
}
