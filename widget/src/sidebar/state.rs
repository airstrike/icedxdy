//! Sidebar geometry: sides, collapse modes and widths

/// Which side the sidebar is on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// How the sidebar collapses (shadcn's `collapsible` prop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Collapsible {
    /// Slides out of view entirely.
    #[default]
    Offcanvas,
    /// Shrinks to its icons ([`WIDTH_ICON`]).
    Icon,
    /// Never collapses.
    None,
}

/// Default sidebar width (256px)
pub const WIDTH: f32 = 256.0;
/// Mobile sidebar width (288px)
pub const WIDTH_MOBILE: f32 = 288.0;
/// Collapsed icon-only width (48px)
pub const WIDTH_ICON: f32 = 48.0;
