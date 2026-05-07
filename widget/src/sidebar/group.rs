//! SidebarGroup, SidebarGroupLabel, SidebarGroupContent

use crate::column::Column;
use crate::component::Component;
use crate::text::Text;
use iced_core::Length::Fill;
use icedxdy_core::theme;

/// Create a sidebar group.
///
/// A column container with 8px padding and full width, used to group
/// related sidebar items together.
pub fn group<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children).padding(8).width(Fill)
}

/// Create a sidebar group label.
///
/// Renders as text-xs (12px), font-medium, with muted foreground color.
/// Corresponds to shadcn's SidebarGroupLabel (h-8, text-xs, font-medium).
pub fn group_label<'a>(label: impl iced_core::text::IntoFragment<'a>) -> Text<'a> {
    Text::new(label)
        .size(12.0)
        .medium()
        .style(theme::text::Muted)
}

/// Create sidebar group content.
///
/// A simple column wrapper with full width for the group's children.
pub fn group_content<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children).width(Fill)
}
