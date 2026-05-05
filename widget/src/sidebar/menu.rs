//! SidebarMenu and SidebarMenuItem

use crate::column::Column;
use crate::component::Component;
use crate::row::Row;
use iced_core::Length::Fill;

/// Create a sidebar menu.
///
/// A column with 4px spacing (gap-1) and full width, serving as a list
/// container for menu items.
pub fn menu<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children)
        .spacing(4)
        .width(Fill)
}

/// Create a sidebar menu item.
///
/// A row wrapper with full width, containing the item's content.
pub fn menu_item<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Row<'a, Message> {
    Row::with_children(children)
}
