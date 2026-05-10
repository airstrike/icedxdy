//! Palette — a single 4-slot Hue.
//!
//! The active color context for whatever's currently being styled.
//! `Theme::tinted(tint)` returns one of `Design`'s per-role palettes;
//! all four slots resolve in lockstep against that role.
//!
//! Light/dark per-role palette values populate `Design::light()` /
//! `Design::dark()` in `core/src/design.rs` — the canonical shadcn
//! OKLCH constants live there.
//!
//! `SidebarPalette` stays as a side-field on `Design` for now; a sidebar
//! context that swaps the full Design is deferred.

use crate::color::Color;

/// A single Hue — 4 colors that everything styled under this palette
/// pulls from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    /// Primary fill / body color of the active role.
    pub base: Color,
    /// Text color on top of `base`.
    pub foreground: Color,
    /// Border drawn from this role.
    pub border: Color,
    /// Focus ring color.
    pub ring: Color,
}

/// Sidebar-specific color palette.
///
/// Deferred reshape — kept as the legacy shape for Phase 1; a future
/// pass dissolves this into a swapped `Design` via a sidebar context.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarPalette {
    pub background: Color,
    pub foreground: Color,
    pub primary: SidebarPair,
    pub accent: SidebarPair,
    pub border: Color,
    pub ring: Color,
}

/// Local base+foreground pair for `SidebarPalette`. Replaces the old
/// `SemanticColor` used inside the sidebar struct.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarPair {
    pub base: Color,
    pub foreground: Color,
}
