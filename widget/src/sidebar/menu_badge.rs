//! SidebarMenuBadge: h-5, min-w-5, text-xs, font-medium, tabular-nums

use crate::container::Container;
use crate::text::Text;
use iced_core::Length;
use icedxdy_core::theme;

/// Create a sidebar menu badge.
///
/// A small badge displaying a count or label — h-5 (20px), text-xs (12px),
/// font-medium, with muted foreground. Used for notification counts or
/// status indicators next to menu items.
pub fn menu_badge<'a, Message: 'a>(
    label: impl iced_core::text::IntoFragment<'a>,
) -> Container<'a, Message> {
    let text = Text::new(label)
        .size(12.0)
        .medium()
        .style(theme::text::muted);

    Container::new(text)
        .height(Length::Fixed(20.0))
        .center_y(Length::Fixed(20.0))
}
