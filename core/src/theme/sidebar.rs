//! Sidebar-specific container styles (flush + floating variants).
//!
//! Other sidebar styling (menu buttons, separators, group labels) is
//! handled inside each respective widget module, matching docs-sidebar
//! rendering rather than base shadcn SidebarMenuButton.

use iced_core::{Border, Shadow, Vector};
use iced_widget::container;

use super::Theme;
use crate::radius;

// ---------------------------------------------------------------------------
// Container
// ---------------------------------------------------------------------------

/// Sidebar background container
pub fn sidebar_background(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(theme.palette().sidebar.background.into()),
        ..Default::default()
    }
}

/// Sidebar floating container: background, border, shadow, rounded corners
pub fn sidebar_floating(theme: &Theme) -> container::Style {
    let sb = &theme.palette().sidebar;

    container::Style {
        background: Some(sb.background.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: sb.border.into(),
        },
        shadow: Shadow {
            color: iced_core::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}
