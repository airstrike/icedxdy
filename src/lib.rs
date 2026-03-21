//! icedxdy - A shadcn-inspired component library for Iced
//!
//! This library provides a design system and themed components for Iced applications,
//! inspired by the shadcn/ui design system.

pub mod button;
pub mod card;
pub mod element;
pub mod separator;
pub mod size;
pub mod text;
pub mod theme;

pub use icedxdy_core as core;

// Re-export key types at the crate root
pub use icedxdy_core::{Color, Design, FontFamily, Palette};
pub use element::wrap;
pub use size::Size;
pub use theme::Theme;
