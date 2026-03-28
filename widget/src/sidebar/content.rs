//! SidebarContent: scrollable, flex-1, gap-2, overflow-auto

use crate::column::Column;
use crate::element::Element;
use iced_core::Length;

/// Create scrollable sidebar content.
///
/// A vertical column with 8px spacing that fills available height.
/// The caller can wrap this in an iced scrollable if needed.
/// Uses `height(Fill)` to expand into available space.
pub fn content<'a, Message: 'a>(
    children: impl IntoIterator<Item = Element<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children)
        .spacing(8)
        .height(Length::Fill)
}
