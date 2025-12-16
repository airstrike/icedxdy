//! icedxdy - A shadcn-inspired component library for Iced
//!
//! This library provides a design system and themed components for Iced applications,
//! inspired by the shadcn/ui design system.

pub mod theme;

pub use icedxdy_core as core;
pub use icedxdy_fonts as fonts;

// Re-export key types at the crate root
pub use icedxdy_core::{Color, Design, Palette};
pub use theme::Theme;
