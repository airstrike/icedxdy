//! Core types for icedxdy - design tokens, colors, and theme definitions
//!
//! This crate provides the foundational types for the icedxdy design system,
//! inspired by shadcn/ui.

pub mod background;
pub mod border;
mod color;
mod constants;
mod design;
mod palette;
pub mod shadow;
pub mod theme;

pub use background::Background;
pub use border::{Border, Radius, radius};
pub use color::{Color, oklch, oklch_alpha};
pub use constants::*;
pub use design::{Design, FontFamily};
pub use palette::{Palette, SemanticColor, SidebarPalette};
pub use shadow::Shadow;
pub use theme::Theme;
