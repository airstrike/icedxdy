//! Palette slot tokens — one variant per addressable color slot, plus a
//! `Custom` escape hatch for arbitrary colors.
//!
//! Mirrors the `widget::Size` precedent: named variants + `Custom(_)` +
//! `From<Color>` blanket. See `widget/src/size.rs`.

use crate::{Color, Theme};

/// A semantic color slot on the active theme, or a literal color.
///
/// Resolves to the crate's `Color` (OKLCH) against a given `Theme` at
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
    /// Resolve this token to a concrete color against the given theme.
    pub fn resolve(self, theme: &Theme) -> Color {
        match self {
            Token::Background => theme.neutral.base,
            Token::Foreground => theme.neutral.foreground,
            Token::Card => theme.card.base,
            Token::CardForeground => theme.card.foreground,
            Token::Popover => theme.popover.base,
            Token::PopoverForeground => theme.popover.foreground,
            Token::Primary => theme.primary.base,
            Token::PrimaryForeground => theme.primary.foreground,
            Token::Secondary => theme.secondary.base,
            Token::SecondaryForeground => theme.secondary.foreground,
            Token::Muted => theme.muted.base,
            Token::MutedForeground => theme.muted.foreground,
            Token::Accent => theme.accent.base,
            Token::AccentForeground => theme.accent.foreground,
            Token::Destructive => theme.destructive.base,
            Token::DestructiveForeground => theme.destructive.foreground,
            Token::Info => theme.info.base,
            Token::InfoForeground => theme.info.foreground,
            Token::Success => theme.success.base,
            Token::SuccessForeground => theme.success.foreground,
            Token::Warning => theme.warning.base,
            Token::WarningForeground => theme.warning.foreground,
            Token::Code => theme.code.base,
            Token::CodeForeground => theme.code.foreground,
            // `input` merged into `border` per Phase 1.
            Token::Border | Token::Input => theme.neutral.border,
            Token::Ring => theme.neutral.ring,
            Token::Sidebar => theme.sidebar.background,
            Token::SidebarForeground => theme.sidebar.foreground,
            Token::SidebarPrimary => theme.sidebar.primary.base,
            Token::SidebarPrimaryForeground => theme.sidebar.primary.foreground,
            Token::SidebarAccent => theme.sidebar.accent.base,
            Token::SidebarAccentForeground => theme.sidebar.accent.foreground,
            Token::SidebarBorder => theme.sidebar.border,
            Token::SidebarRing => theme.sidebar.ring,
            Token::Custom(c) => c,
        }
    }
}
