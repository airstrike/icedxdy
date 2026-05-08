//! Layout orientation primitive shared across icedxdy widgets.
//!
//! Used by every widget that has a "horizontal vs. vertical" axis prop —
//! `Field`, `button::group::ButtonGroup`, etc. Living in the core crate
//! (rather than per-widget) means each widget's `.orientation()` builder
//! takes the SAME type, so callers can pass `Vertical` / `Horizontal` from
//! a single bare-variant import (`use icedxdy::{Vertical, Horizontal};`)
//! and it flows into every widget without per-widget enum collision.
//!
//! Each widget that has a stronger naming preference can still re-export
//! at its own module path (`field::Orientation`, `button::group::Orientation`,
//! etc.), but the underlying type is the one defined here so values cross
//! module boundaries seamlessly.

/// Layout orientation for a widget that has a single axis prop.
///
/// Mirrors shadcn's `<Field orientation="...">` / `<ButtonGroup orientation="...">`
/// prop. Each widget picks its own default (Field defaults to Vertical;
/// ButtonGroup to Horizontal) — this enum is just the value space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Stacked top-to-bottom (Field's default).
    Vertical,
    /// Laid out left-to-right (ButtonGroup's default).
    Horizontal,
}
