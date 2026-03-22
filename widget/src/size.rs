//! Size variants — nova style
//!
//! See plan/nova-style-guide.md for full spec.

use iced_core::Padding;

/// Component size, matching shadcn nova style.
///
/// Nova sizes:
/// - `Xs`:      h-6  (24px), px-2 (8px),   text-xs (12px),         gap-1 (4px)
/// - `Sm`:      h-7  (28px), px-2.5 (10px), text-[0.8rem] (12.8px), gap-1 (4px)
/// - `Default`: h-8  (32px), px-2.5 (10px), text-sm (14px),         gap-1.5 (6px)
/// - `Lg`:      h-9  (36px), px-2.5 (10px), text-sm (14px),         gap-1.5 (6px)
/// - `Icon`:    size-8 (32×32)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Xs,
    Sm,
    Default,
    Lg,
    Icon,
    Custom(f32),
}

impl std::default::Default for Size {
    fn default() -> Self {
        Size::Default
    }
}

impl Size {
    /// Button height
    pub const fn height(self) -> f32 {
        match self {
            Size::Xs => 24.0,
            Size::Sm => 28.0,
            Size::Default | Size::Icon => 32.0,
            Size::Lg => 36.0,
            Size::Custom(_) => 32.0,
        }
    }

    /// Button padding — vertical padding centers content within the height
    pub const fn padding(self) -> Padding {
        match self {
            // h-6 (24px), text-xs ~16px line height → (24-16)/2 = 4
            Size::Xs => Padding { top: 4.0, bottom: 4.0, left: 8.0, right: 8.0 },
            // h-7 (28px), text-[0.8rem] ~18px line height → (28-18)/2 = 5
            Size::Sm => Padding { top: 5.0, bottom: 5.0, left: 10.0, right: 10.0 },
            // h-8 (32px), text-sm ~18px line height → (32-18)/2 = 7
            Size::Default => Padding { top: 7.0, bottom: 7.0, left: 10.0, right: 10.0 },
            // h-9 (36px), text-sm ~18px line height → (36-18)/2 = 9
            Size::Lg => Padding { top: 9.0, bottom: 9.0, left: 10.0, right: 10.0 },
            // square — icon centered via center() wrapper in element.rs
            Size::Icon => Padding { top: 0.0, bottom: 0.0, left: 0.0, right: 0.0 },
            Size::Custom(_) => Padding { top: 6.0, bottom: 6.0, left: 10.0, right: 10.0 },
        }
    }

    /// Text/font size
    pub const fn text_size(self) -> f32 {
        match self {
            Size::Xs => 12.0,
            Size::Sm => 12.8,
            Size::Custom(v) => v,
            _ => 14.0,
        }
    }

    /// Gap between children
    pub const fn gap(self) -> f32 {
        match self {
            Size::Xs | Size::Sm => 4.0,
            _ => 6.0,
        }
    }

    /// Icon/SVG size
    pub const fn icon_size(self) -> f32 {
        match self {
            Size::Xs => 12.0,
            Size::Sm => 14.0,
            _ => 16.0,
        }
    }
}

impl From<f32> for Size {
    fn from(v: f32) -> Self {
        Size::Custom(v)
    }
}

impl From<u16> for Size {
    fn from(v: u16) -> Self {
        Size::Custom(v as f32)
    }
}

impl From<i32> for Size {
    fn from(v: i32) -> Self {
        Size::Custom(v as f32)
    }
}
