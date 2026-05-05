//! SidebarInset — main content area paired with a sidebar.
//!
//! Two flavors, mirroring shadcn's `SidebarInset` rendering:
//!
//! - [`main`] — the baseline content-area wrapper (`flex-1` equivalent),
//!   used with `fixed` or `floating` sidebars.
//! - [`main_styled`] — the inset-variant chrome (`rounded-xl border
//!   shadow-sm`), paired with [`super::inset`] sidebars.

use crate::component::Component;
use crate::container::Container;
use iced_core::Length::Fill;
use icedxdy_core::theme::container;

/// Create a sidebar inset content area (baseline).
///
/// The main content area next to the sidebar. Takes full width and
/// height (flex-1 equivalent), used as the primary content region with
/// `fixed` and `floating` sidebars.
pub fn main<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Container<'a, Message> {
    Container::new(content).width(Fill).height(Fill)
}

/// Create a sidebar inset content area styled for the Inset sidebar variant.
///
/// Adds the inset-variant chrome (`rounded-xl border shadow-sm`) so the
/// content area reads as a card floating against the sidebar background.
/// Pair with [`super::inset`] for the matching sidebar.
pub fn main_styled<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Container<'a, Message> {
    Container::new(content)
        .width(Fill)
        .height(Fill)
        .class(container::SidebarInset)
}
