//! Popover and menu panel styling: nova style.
//!
//! `rounded-lg bg-popover text-popover-foreground shadow-md ring-1
//! ring-foreground/10`, shared by `PopoverContent`, `HoverCardContent` and
//! the menu contents (dropdown, context menu, select, combobox).

use iced_core::Vector;

use super::Theme;
use super::container::Style;
use crate::{Border, Color, Shadow, radius};

/// A popover's panel.
pub fn content(theme: &Theme) -> Style {
    Style {
        background: Some(theme.popover.base.into()),
        text_color: Some(theme.popover.foreground),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: theme.neutral.foreground.alpha(0.1),
        },
        // `shadow-md`: 0 4px 6px -1px rgb(0 0 0 / 0.1).
        shadow: Shadow {
            color: Color::BLACK.alpha(0.1),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 6.0,
        },
        ..Style::default()
    }
}
