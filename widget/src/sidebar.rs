//! Sidebar components — a faithful port of shadcn/ui's Sidebar suite.

mod content;
mod footer;
mod group;
mod header;
mod input;
mod inset;
mod menu;
mod menu_action;
mod menu_badge;
mod menu_button;
mod menu_skeleton;
mod rail;
mod separator;
mod state;
mod submenu;
mod trigger;

pub use content::content;
pub use footer::footer;
pub use group::{group, group_content, group_label};
pub use header::header;
pub use input::input;
pub use inset::{main, main_styled};
pub use menu::{menu, menu_item};
pub use menu_action::{SidebarMenuAction, menu_action};
pub use menu_badge::menu_badge;
pub use menu_button::{MenuButtonSize, SidebarMenuButton, menu_button};
pub use menu_skeleton::menu_skeleton;
pub use rail::{RAIL_WIDTH, SidebarRail, rail};
pub use separator::separator;
pub use state::{Collapsible, Side, State, WIDTH, WIDTH_ICON, WIDTH_MOBILE};
pub use submenu::{submenu, submenu_button, submenu_item};
pub use trigger::trigger;

use crate::compose::Slot;
use crate::container::Container;
use icedxdy_core::theme::sidebar;

/// A sidebar flush against the window edge.
pub fn fixed<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Container<'a, Message> {
    Container::new(content).style(sidebar::default)
}

/// A sidebar floating with margin, rounded corners, border, and shadow.
pub fn floating<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Container<'a, Message> {
    Container::new(content).style(sidebar::floating)
}

/// An inset sidebar — no border on the sidebar itself; the content area
/// (paired via [`main_styled`]) carries the rounded/border/shadow chrome.
pub fn inset<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Container<'a, Message> {
    Container::new(content).style(sidebar::inset)
}
