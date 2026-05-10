//! Tint enum — selects which palette resolves under `StyleFn`.

#![allow(non_upper_case_globals)]

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

pub const neutral: Tint = Tint::Neutral;
pub const primary: Tint = Tint::Primary;
pub const secondary: Tint = Tint::Secondary;
pub const muted: Tint = Tint::Muted;
pub const accent: Tint = Tint::Accent;
pub const destructive: Tint = Tint::Destructive;
pub const info: Tint = Tint::Info;
pub const success: Tint = Tint::Success;
pub const warning: Tint = Tint::Warning;
