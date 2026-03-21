//! Size variants for icedxdy components
//!
//! [`Size`] is the shared language between parent and child widgets.
//! Each widget interprets it in its own context (padding for buttons,
//! font size for text, diameter for spinners, etc.).

use iced_core::Padding;

/// Component size, matching shadcn/ui conventions.
///
/// Named variants map to the shadcn scale. [`Custom`](Size::Custom) allows
/// arbitrary values — each widget decides what the `f32` means (px for
/// text, padding scale for buttons, etc.).
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
            Size::Lg => Padding { top: 10.0, bottom: 10.0, left: 32.0, right: 32.0 },
            Size::Icon => Padding { top: 8.0, bottom: 8.0, left: 8.0, right: 8.0 },
            Size::Custom(_) => Padding { top: 8.0, bottom: 8.0, left: 16.0, right: 16.0 },
        }
    }

    /// Text/font size for this size
    pub const fn text_size(self) -> f32 {
        match self {
            Size::Xs => 12.0,
            Size::Sm => 12.8,
            Size::Default => 14.0,
            Size::Lg => 14.0,
            Size::Icon => 14.0,
            Size::Custom(v) => v,
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
