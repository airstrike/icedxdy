//! Sidebar components — a faithful port of shadcn/ui's Sidebar suite.

mod content;
mod footer;
mod group;
mod header;
mod inset;
mod input;
mod menu;
mod menu_action;
mod menu_badge;
mod menu_button;
mod menu_skeleton;
mod separator;
mod state;
mod submenu;
mod trigger;

pub use content::content;
pub use footer::footer;
pub use group::{group, group_content, group_label};
pub use header::header;
pub use inset::inset;
pub use input::input;
pub use menu::{menu, menu_item};
pub use menu_action::{menu_action, SidebarMenuAction};
pub use menu_badge::menu_badge;
pub use menu_button::{menu_button, MenuButtonSize, MenuButtonVariant, SidebarMenuButton};
pub use menu_skeleton::menu_skeleton;
pub use separator::separator;
pub use state::{Collapsible, Side, State, WIDTH, WIDTH_ICON, WIDTH_MOBILE};
pub use submenu::{submenu, submenu_button, submenu_item};
pub use trigger::trigger;

use crate::component::Component;
use crate::container::Container;
use icedxdy_core::theme::sidebar;

/// A sidebar flush against the window edge.
pub fn fixed<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Container<'a, Message> {
    Container::new(content).style(sidebar::default)
}

/// A sidebar floating with margin, rounded corners, border, and shadow.
pub fn floating<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Container<'a, Message> {
    Container::new(content).style(sidebar::floating)
}
