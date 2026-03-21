//! Size variants for icedxdy components
//!
//! [`Size`] is the shared language between parent and child widgets.
//! Each widget interprets it in its own context (padding for buttons,
//! font size for text, diameter for spinners, etc.).

use iced_core::Padding;

/// Component size, matching shadcn/ui conventions.
///
/// shadcn sizes:
/// - `Xs`:      h-6  (24px), px-2, text-xs (12px), gap-1 (4px)
/// - `Sm`:      h-8  (32px), px-3, text-sm (14px), gap-1.5 (6px)
/// - `Default`: h-9  (36px), px-4 py-2, text-sm (14px), gap-2 (8px)
/// - `Lg`:      h-10 (40px), px-6, text-sm (14px), gap-2 (8px)
/// - `Icon`:    h-9  (36px), square, text-sm (14px)
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
    /// Button padding for this size
    pub const fn padding(self) -> Padding {
        match self {
            Size::Xs => Padding { top: 4.0, bottom: 4.0, left: 8.0, right: 8.0 },
            Size::Sm => Padding { top: 6.0, bottom: 6.0, left: 12.0, right: 12.0 },
            Size::Default => Padding { top: 8.0, bottom: 8.0, left: 16.0, right: 16.0 },
            Size::Lg => Padding { top: 10.0, bottom: 10.0, left: 24.0, right: 24.0 },
            Size::Icon => Padding { top: 8.0, bottom: 8.0, left: 8.0, right: 8.0 },
            Size::Custom(_) => Padding { top: 8.0, bottom: 8.0, left: 16.0, right: 16.0 },
        }
    }

    /// Button height for this size (used for square icon buttons)
    pub const fn height(self) -> f32 {
        match self {
            Size::Xs => 24.0,
            Size::Sm => 32.0,
            Size::Default | Size::Icon => 36.0,
            Size::Lg => 40.0,
            Size::Custom(_) => 36.0,
        }
    }

    /// Text/font size for this size
    pub const fn text_size(self) -> f32 {
        match self {
            Size::Xs => 12.0,
            Size::Custom(v) => v,
            _ => 14.0,
        }
    }

    /// Square padding for icon-only buttons (equal on all sides)
    pub const fn icon_padding(self) -> Padding {
        let v = self.padding().top;
        Padding { top: v, bottom: v, left: v, right: v }
    }

    /// Gap between children (icon + text) for this size
    pub const fn gap(self) -> f32 {
        match self {
            Size::Xs => 4.0,
            Size::Sm => 6.0,
            _ => 8.0,
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
