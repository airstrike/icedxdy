//! Tint enum — selects which palette resolves under `StyleFn`.
//!
//! Variant consts (`destructive`, `secondary`, etc.) ship in Phase 2.

/// The active palette role at style-resolution time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tint {
    Neutral,
    Primary,
    Secondary,
    Muted,
    Accent,
    Destructive,
    Info,
    Success,
    Warning,
}
