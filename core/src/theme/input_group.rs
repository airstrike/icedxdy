//! InputGroup styling - nova style.
//!
//! The group paints the Input's surface (border, focus ring, invalid and
//! disabled states, from [`text_input`]) around its addons; the control
//! inside is bare: `rounded-none border-0 bg-transparent ring-0`.

use super::Theme;
use super::text_input::{self, Status, Style};
use crate::{Background, Border, Color};

/// An Input inside a group: the Input's text colors on no surface.
pub fn control(theme: &Theme, status: Status) -> Style {
    Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        ring: None,
        ..text_input::default(theme, status)
    }
}

/// A Textarea inside a group: the textarea's text colors on no surface.
pub fn textarea(
    theme: &Theme,
    status: iced_widget::text_editor::Status,
) -> iced_widget::text_editor::Style {
    iced_widget::text_editor::Style {
        background: iced_core::Background::Color(iced_core::Color::TRANSPARENT),
        border: iced_core::Border::default(),
        ..super::textarea::default(theme, status)
    }
}
