//! Design tokens for spacing, sizing, and border radius
//!
//! Based on shadcn/ui design system values.

/// Border radius values
///
/// Base radius is 10px (0.625rem), with variants calculated relative to it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radius;

impl Radius {
    /// 6px - calc(var(--radius) - 4px)
    pub const SM: f32 = 6.0;
    /// 8px - calc(var(--radius) - 2px)
    pub const MD: f32 = 8.0;
    /// 10px - var(--radius) - default
    pub const LG: f32 = 10.0;
    /// 14px - calc(var(--radius) + 4px)
    pub const XL: f32 = 14.0;
    /// 18px - calc(var(--radius) + 8px)
    pub const XL2: f32 = 18.0;
    /// 22px - calc(var(--radius) + 12px)
    pub const XL3: f32 = 22.0;
    /// 26px - calc(var(--radius) + 16px)
    pub const XL4: f32 = 26.0;
    /// Full/pill radius
    pub const FULL: f32 = 9999.0;
}

/// Spacing values in pixels
///
/// Based on Tailwind's spacing scale (1 unit = 4px)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spacing;

impl Spacing {
    /// 0px
    pub const ZERO: f32 = 0.0;
    /// 2px - 0.5 units
    pub const XS2: f32 = 2.0;
    /// 4px - 1 unit
    pub const XS: f32 = 4.0;
    /// 6px - 1.5 units
    pub const SM: f32 = 6.0;
    /// 8px - 2 units
    pub const MD: f32 = 8.0;
    /// 10px - 2.5 units
    pub const LG: f32 = 10.0;
    /// 12px - 3 units
    pub const XL: f32 = 12.0;
    /// 16px - 4 units
    pub const XL2: f32 = 16.0;
    /// 20px - 5 units
    pub const XL3: f32 = 20.0;
    /// 24px - 6 units
    pub const XL4: f32 = 24.0;
    /// 32px - 8 units
    pub const XL5: f32 = 32.0;
    /// 40px - 10 units
    pub const XL6: f32 = 40.0;
    /// 48px - 12 units
    pub const XL7: f32 = 48.0;
    /// 64px - 16 units
    pub const XL8: f32 = 64.0;
}

/// Font size values in pixels
///
/// Based on Tailwind's font size scale
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontSize;

impl FontSize {
    /// 10px - text-[10px]
    pub const XS2: f32 = 10.0;
    /// 12px - text-xs
    pub const XS: f32 = 12.0;
    /// 14px - text-sm (default for buttons)
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

/// Component size values (heights) in pixels
///
/// Based on shadcn button size specifications
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size;

impl Size {
    /// 24px - xs size
    pub const XS: f32 = 24.0;
    /// 32px - sm size
    pub const SM: f32 = 32.0;
    /// 36px - default size
    pub const MD: f32 = 36.0;
    /// 40px - lg size
    pub const LG: f32 = 40.0;
    /// 48px - xl size
    pub const XL: f32 = 48.0;
}

/// Icon size values in pixels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IconSize;

impl IconSize {
    /// 12px - size-3
    pub const XS: f32 = 12.0;
    /// 16px - size-4
    pub const SM: f32 = 16.0;
    /// 20px - size-5
    pub const MD: f32 = 20.0;
    /// 24px - size-6
    pub const LG: f32 = 24.0;
}
