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

use iced_widget::text::Text;

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
/// 36px, extra bold
pub fn h1(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::XL4)
        .style(theme::text::foreground)
}

/// H2 heading - section heading
///
/// 30px, semibold
pub fn h2(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::XL3)
        .style(theme::text::foreground)
}

/// H3 heading - subsection heading
///
/// 24px, semibold
pub fn h3(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::XL2)
        .style(theme::text::foreground)
}

/// H4 heading - small heading
///
/// 20px, semibold
pub fn h4(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::XL)
        .style(theme::text::foreground)
}

/// Paragraph text - default body text
///
/// 16px, normal weight
pub fn p(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::BASE)
        .style(theme::text::foreground)
}

/// Lead text - intro paragraph style
///
/// 20px, muted color
pub fn lead(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::XL)
        .style(theme::text::muted)
}

/// Large text - emphasized text
///
/// 18px, semibold
pub fn large(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::LG)
        .style(theme::text::foreground)
}

/// Small text - fine print
///
/// 14px, medium weight
pub fn small(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::SM)
        .style(theme::text::foreground)
}

/// Muted text - secondary/helper text
///
/// 14px, muted color
pub fn muted(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::SM)
        .style(theme::text::muted)
}

/// Extra small text - fine print, labels
///
/// 12px
pub fn xs(content: impl ToString) -> Text<'static, Theme> {
    Text::new(content.to_string())
        .size(size::XS)
        .style(theme::text::foreground)
}
