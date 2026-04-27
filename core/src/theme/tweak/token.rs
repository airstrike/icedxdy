//! Palette slot tokens — one variant per `Palette` field, plus a `Custom`
//! escape hatch for arbitrary colors.
//!
//! Mirrors the `widget::Size` precedent: named variants + `Custom(_)` +
//! `From<Color>` blanket. See `widget/src/size.rs`.

use crate::{Color, Palette};

/// A semantic color slot in the palette, or a literal color.
///
/// Resolves to the crate's `Color` (OKLCH) against a given `Palette` at
/// style computation time. Convert to iced types via the existing
/// `From<Color>` impls at the apply site.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Background,
    Foreground,
    Card,
    CardForeground,
    Popover,
    PopoverForeground,
    Primary,
    PrimaryForeground,
    Secondary,
    SecondaryForeground,
    Muted,
    MutedForeground,
    Accent,
    AccentForeground,
    Destructive,
    DestructiveForeground,
    Info,
    InfoForeground,
    Success,
    SuccessForeground,
    Warning,
    WarningForeground,
    Code,
    CodeForeground,
    Border,
    Input,
    Ring,
    Sidebar,
    SidebarForeground,
    SidebarPrimary,
    SidebarPrimaryForeground,
    SidebarAccent,
    SidebarAccentForeground,
    SidebarBorder,
    SidebarRing,
    Custom(Color),
}

impl From<Color> for Token {
    fn from(c: Color) -> Self {
        Token::Custom(c)
    }
}

impl Token {
    /// Resolve this token to a concrete color against the given palette.
    pub fn resolve(self, palette: &Palette) -> Color {
        match self {
            Token::Background => palette.background,
            Token::Foreground => palette.foreground,
            Token::Card => palette.card.base,
            Token::CardForeground => palette.card.foreground,
            Token::Popover => palette.popover.base,
            Token::PopoverForeground => palette.popover.foreground,
            Token::Primary => palette.primary.base,
            Token::PrimaryForeground => palette.primary.foreground,
            Token::Secondary => palette.secondary.base,
            Token::SecondaryForeground => palette.secondary.foreground,
            Token::Muted => palette.muted.base,
            Token::MutedForeground => palette.muted.foreground,
            Token::Accent => palette.accent.base,
            Token::AccentForeground => palette.accent.foreground,
            Token::Destructive => palette.destructive.base,
            Token::DestructiveForeground => palette.destructive.foreground,
            Token::Info => palette.info.base,
            Token::InfoForeground => palette.info.foreground,
            Token::Success => palette.success.base,
            Token::SuccessForeground => palette.success.foreground,
            Token::Warning => palette.warning.base,
            Token::WarningForeground => palette.warning.foreground,
            Token::Code => palette.code.base,
            Token::CodeForeground => palette.code.foreground,
            Token::Border => palette.border,
            Token::Input => palette.input,
            Token::Ring => palette.ring,
            Token::Sidebar => palette.sidebar.background,
            Token::SidebarForeground => palette.sidebar.foreground,
            Token::SidebarPrimary => palette.sidebar.primary.base,
            Token::SidebarPrimaryForeground => palette.sidebar.primary.foreground,
            Token::SidebarAccent => palette.sidebar.accent.base,
            Token::SidebarAccentForeground => palette.sidebar.accent.foreground,
            Token::SidebarBorder => palette.sidebar.border,
            Token::SidebarRing => palette.sidebar.ring,
            Token::Custom(c) => c,
        }
    }
}
