//! SidebarTrigger — toggle button for collapsing/expanding the sidebar
//!
//! A ghost-style button (28x28) that emits a message to toggle the sidebar.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::sidebar;
//!
//! sidebar::trigger(Message::ToggleSidebar)
//! ```

use iced_widget::Renderer;

use icedxdy_core::Theme;
use icedxdy_core::theme::button;

/// Create a sidebar trigger button (ghost, 28x28).
///
/// Emits `on_toggle` when pressed. Uses a hamburger icon placeholder.
pub fn trigger<'a, Message: Clone + 'a>(
    on_toggle: Message,
) -> iced_widget::Button<'a, Message, Theme, Renderer> {
    iced_widget::button(iced_widget::center(
        iced_widget::text("\u{2630}").size(16.0),
    ))
    .width(28.0)
    .height(28.0)
    .padding(0)
    .style(|t, s| button::ghost(t, s).into())
    .on_press(on_toggle)
}
