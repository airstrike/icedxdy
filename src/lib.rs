//! icedxdy - A shadcn-inspired component library for Iced
//!
//! This library provides a design system and themed components for Iced applications,
//! inspired by the shadcn/ui design system.

pub mod card;
pub mod separator;

pub use icedxdy_core as core;
pub use icedxdy_core::{Color, Design, FontFamily, Palette, Theme};
pub use icedxdy_core::theme;
pub use icedxdy_core::{font_size, height, icon_size, radius, spacing};

#[allow(hidden_glob_reexports)]
pub mod widget {
    pub use icedxdy_widget::*;
}

pub use widget::Component;
pub use widget::Size;
pub use widget::Weight;

/// Raw iced widgets with modifications (centering, focus/blur, etc.)
pub mod raw {
    pub use icedxdy_widget::button::widget::Button;
}
