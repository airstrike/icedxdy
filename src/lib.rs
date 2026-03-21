//! icedxdy - A shadcn-inspired component library for Iced
//!
//! This library provides a design system and themed components for Iced applications,
//! inspired by the shadcn/ui design system.

pub mod card;
pub mod separator;

pub use icedxdy_core as core;
pub use icedxdy_core::{Color, Design, FontFamily, Palette, Theme};
pub use icedxdy_core::theme;

#[allow(hidden_glob_reexports)]
pub mod widget {
    pub use icedxdy_widget::*;
}

pub use widget::Size;
pub use widget::Weight;
