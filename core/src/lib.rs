//! Core types for icedxdy - design tokens, colors, and theme definitions
//!
//! This crate provides the foundational types for the icedxdy design system,
//! inspired by shadcn/ui.

mod color;
mod constants;
mod design;
mod palette;
pub mod theme;

pub use color::{oklch, oklch_alpha, Color};
pub use constants::*;
pub use design::{Design, FontFamily};
pub use palette::{Palette, SemanticColor, SidebarPalette};
pub use theme::Theme;
