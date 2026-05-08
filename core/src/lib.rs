//! Core types for icedxdy - design tokens, colors, and theme definitions
//!
//! This crate provides the foundational types for the icedxdy design system,
//! inspired by shadcn/ui.

pub mod background;
pub mod border;
mod color;
mod constants;
mod design;
mod orientation;
mod palette;
pub mod shadow;
pub mod theme;

pub use background::Background;
pub use border::{Border, Radius, radius};
pub use color::{Color, oklch, oklch_alpha};
pub use constants::*;
pub use design::{Design, FontFamily};
pub use orientation::Orientation;
// Re-export the variants at module scope so a bare `Vertical` /
// `Horizontal` import from `icedxdy_core` (or, transitively, from `icedxdy`)
// can be passed directly to any `.orientation()` builder via `impl Into`.
pub use orientation::Orientation::{Horizontal, Vertical};
pub use palette::{Palette, SemanticColor, SidebarPalette};
pub use shadow::Shadow;
pub use theme::Theme;
