//! Typography components
//!
//! Semantic text helpers based on shadcn/ui typography patterns.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::text;
//!
//! text::h1("Page Title")
//! text::muted("Secondary text")
//! text::small("Fine print")
//! ```
//!
//! # TODO: Text wrapper + fonts architecture
//!
//! See `plan/COMPONENTS.md` for the full plan. Summary:
//!
//! 1. Create our own `Text<'a>` wrapper (not iced's Text directly)
//! 2. Add `.font()` method that takes `impl Into<FontFamily>` - allows `.font("inter")`
//! 3. Font resolution happens in `impl From<Text> for Element`:
//!    - Look up font name in icedxdy_fonts registry
//!    - If found, use resolved iced::Font with proper weight
//!    - If not found, use None (iced default) + debug print missing font
//! 4. This lets us use iced's default font normally, but users can opt into
//!    vendored fonts with a clean API like `.font("inter")`

use iced_core::text::IntoFragment;
use iced_widget::text;
use iced_widget::text::Text;

use crate::fonts;
use crate::theme;
use crate::Theme;

/// Font sizes based on Tailwind/shadcn scale
mod size {
    /// 12px - text-xs
    pub const XS: f32 = 12.0;
    /// 14px - text-sm
    pub const SM: f32 = 14.0;
    /// 16px - text-base
    pub const BASE: f32 = 16.0;
    /// 18px - text-lg
    pub const LG: f32 = 18.0;
    /// 20px - text-xl
    pub const XL: f32 = 20.0;
    /// 24px - text-2xl
    pub const XL2: f32 = 24.0;
    /// 30px - text-3xl
    pub const XL3: f32 = 30.0;
    /// 36px - text-4xl
    pub const XL4: f32 = 36.0;
}

/// H1 heading - largest heading style
///
/// 36px, bold
pub fn h1<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::XL4)
        .font(fonts::INTER_BOLD.get())
        .style(theme::text::foreground)
}

/// H2 heading - section heading
///
/// 30px, semibold
pub fn h2<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::XL3)
        .font(fonts::INTER_SEMIBOLD.get())
        .style(theme::text::foreground)
}

/// H3 heading - subsection heading
///
/// 24px, semibold
pub fn h3<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::XL2)
        .font(fonts::INTER_SEMIBOLD.get())
        .style(theme::text::foreground)
}

/// H4 heading - small heading
///
/// 20px, semibold
pub fn h4<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::XL)
        .font(fonts::INTER_SEMIBOLD.get())
        .style(theme::text::foreground)
}

/// Paragraph text - default body text
///
/// 16px, normal weight
pub fn p<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::BASE)
        .style(theme::text::foreground)
}

/// Lead text - intro paragraph style
///
/// 20px, muted color
pub fn lead<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::XL)
        .style(theme::text::muted)
}

/// Large text - emphasized text
///
/// 18px, semibold
pub fn large<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::LG)
        .font(fonts::INTER_SEMIBOLD.get())
        .style(theme::text::foreground)
}

/// Small text - fine print
///
/// 14px, medium weight
pub fn small<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::SM)
        .style(theme::text::foreground)
}

/// Muted text - secondary/helper text
///
/// 14px, muted color
pub fn muted<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::SM)
        .style(theme::text::muted)
}

/// Extra small text - fine print, labels
///
/// 12px
pub fn xs<'a>(content: impl IntoFragment<'a>) -> Text<'a, Theme> {
    text(content)
        .size(size::XS)
        .style(theme::text::foreground)
}
