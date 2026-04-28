//! SidebarMenuSub, SidebarMenuSubItem, SidebarMenuSubButton

use crate::column::Column;
use crate::container::Container;
use crate::element::Element;
use crate::row::Row;
use iced_core::Length::Fill;
use iced_core::Padding;
use icedxdy_core::Theme;

/// Create a sidebar submenu.
///
/// A nested list with a left border line, matching shadcn's SidebarMenuSub
/// (mx-3.5 = 14px, px-2.5 = 10px, py-0.5 = 2px). The left border provides
/// visual nesting indication.
pub fn submenu<'a, Message: 'a>(
    children: impl IntoIterator<Item = Element<'a, Message>>,
) -> Row<'a, Message> {
    // Left border indicator
    let border_line: iced_core::Element<'a, Message, Theme, iced_widget::Renderer> =
        iced_widget::container(iced_widget::space::horizontal())
            .width(1.0)
            .height(Fill)
            .style(|theme: &Theme| iced_widget::container::Style {
                background: Some(theme.palette().border.into()),
                ..Default::default()
            })
            .into();
    let border_element = Element::from(border_line);

    // Items column with padding
    let items = Column::with_children(children)
        .spacing(4)
        .padding(Padding {
            top: 2.0,
            bottom: 2.0,
            left: 10.0,
            right: 0.0,
        })
        .width(Fill);

    Row::with_children([
        border_element,
        items.into(),
    ])
    .spacing(0)
}

/// Create a sidebar submenu item.
///
/// A simple row wrapper for submenu content.
pub fn submenu_item<'a, Message: 'a>(
    children: impl IntoIterator<Item = Element<'a, Message>>,
) -> Row<'a, Message> {
    Row::with_children(children)
}

/// Create a sidebar submenu button (placeholder).
///
/// A smaller menu button (h-7 / 28px) for nested navigation items.
/// This is a placeholder that wraps content in a 28px-high container
/// until the full menu_button component is implemented.
pub fn submenu_button<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    Container::new(content)
        .width(Fill)
        .height(28)
}
