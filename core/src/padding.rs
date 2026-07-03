//! Tailwind-shaped padding tokens.

use std::ops::{Add, AddAssign};

use iced_core::Pixels;

use crate::Gap;

/// Edge padding in logical pixels.
///
/// Mirrors iced's edge order (`top`, `right`, `bottom`, `left`) while exposing
/// Tailwind-shaped constants like [`X4`] (`px-4`) and [`Y4`] (`py-4`).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Padding {
    /// Top edge.
    pub top: f32,
    /// Right edge.
    pub right: f32,
    /// Bottom edge.
    pub bottom: f32,
    /// Left edge.
    pub left: f32,
}

macro_rules! padding_tokens {
    ($(($p:ident, $x:ident, $y:ident, $t:ident, $r:ident, $b:ident, $l:ident, $amount:expr)),+ $(,)?) => {
        impl Padding {
            $(
                #[doc = "Uniform padding token."]
                pub const $p: Self = Self::all($amount);
                #[doc = "Horizontal padding token."]
                pub const $x: Self = Self::axis(0.0, $amount);
                #[doc = "Vertical padding token."]
                pub const $y: Self = Self::axis($amount, 0.0);
                #[doc = "Top padding token."]
                pub const $t: Self = Self::sides($amount, 0.0, 0.0, 0.0);
                #[doc = "Right padding token."]
                pub const $r: Self = Self::sides(0.0, $amount, 0.0, 0.0);
                #[doc = "Bottom padding token."]
                pub const $b: Self = Self::sides(0.0, 0.0, $amount, 0.0);
                #[doc = "Left padding token."]
                pub const $l: Self = Self::sides(0.0, 0.0, 0.0, $amount);
            )+
        }

        $(
            #[doc = "Uniform padding token."]
            pub const $p: Padding = Padding::$p;
            #[doc = "Horizontal padding token."]
            pub const $x: Padding = Padding::$x;
            #[doc = "Vertical padding token."]
            pub const $y: Padding = Padding::$y;
            #[doc = "Top padding token."]
            pub const $t: Padding = Padding::$t;
            #[doc = "Right padding token."]
            pub const $r: Padding = Padding::$r;
            #[doc = "Bottom padding token."]
            pub const $b: Padding = Padding::$b;
            #[doc = "Left padding token."]
            pub const $l: Padding = Padding::$l;
        )+
    };
}

impl Padding {
    /// Zero padding.
    pub const ZERO: Self = Self::all(0.0);

    /// Uniform padding on all edges.
    pub const fn all(amount: f32) -> Self {
        Self {
            top: amount,
            right: amount,
            bottom: amount,
            left: amount,
        }
    }

    /// Axis padding: `y` applies to top/bottom and `x` to left/right.
    pub const fn axis(y: f32, x: f32) -> Self {
        Self {
            top: y,
            right: x,
            bottom: y,
            left: x,
        }
    }

    /// Edge padding in `top`, `right`, `bottom`, `left` order.
    pub const fn sides(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Uniform padding on all edges.
    pub fn new(amount: impl Into<Pixels>) -> Self {
        Self::all(amount.into().0)
    }

    /// Set top padding.
    pub fn top(mut self, amount: impl Into<Pixels>) -> Self {
        self.top = amount.into().0;
        self
    }

    /// Set right padding.
    pub fn right(mut self, amount: impl Into<Pixels>) -> Self {
        self.right = amount.into().0;
        self
    }

    /// Set bottom padding.
    pub fn bottom(mut self, amount: impl Into<Pixels>) -> Self {
        self.bottom = amount.into().0;
        self
    }

    /// Set left padding.
    pub fn left(mut self, amount: impl Into<Pixels>) -> Self {
        self.left = amount.into().0;
        self
    }

    /// Total horizontal padding.
    pub const fn x(self) -> f32 {
        self.left + self.right
    }

    /// Total vertical padding.
    pub const fn y(self) -> f32 {
        self.top + self.bottom
    }
}

padding_tokens! {
    (P0, X0, Y0, T0, R0, B0, L0, 0.0),
    (P0_5, X0_5, Y0_5, T0_5, R0_5, B0_5, L0_5, 2.0),
    (P1, X1, Y1, T1, R1, B1, L1, 4.0),
    (P1_5, X1_5, Y1_5, T1_5, R1_5, B1_5, L1_5, 6.0),
    (P2, X2, Y2, T2, R2, B2, L2, 8.0),
    (P2_5, X2_5, Y2_5, T2_5, R2_5, B2_5, L2_5, 10.0),
    (P3, X3, Y3, T3, R3, B3, L3, 12.0),
    (P4, X4, Y4, T4, R4, B4, L4, 16.0),
    (P5, X5, Y5, T5, R5, B5, L5, 20.0),
    (P6, X6, Y6, T6, R6, B6, L6, 24.0),
    (P7, X7, Y7, T7, R7, B7, L7, 28.0),
    (P8, X8, Y8, T8, R8, B8, L8, 32.0),
}

impl Default for Padding {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Add for Padding {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            top: self.top + rhs.top,
            right: self.right + rhs.right,
            bottom: self.bottom + rhs.bottom,
            left: self.left + rhs.left,
        }
    }
}

impl AddAssign for Padding {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl From<iced_core::Padding> for Padding {
    fn from(padding: iced_core::Padding) -> Self {
        Self {
            top: padding.top,
            right: padding.right,
            bottom: padding.bottom,
            left: padding.left,
        }
    }
}

impl From<Padding> for iced_core::Padding {
    fn from(padding: Padding) -> Self {
        Self {
            top: padding.top,
            right: padding.right,
            bottom: padding.bottom,
            left: padding.left,
        }
    }
}

impl From<Gap> for Padding {
    fn from(gap: Gap) -> Self {
        Self::all(gap.0)
    }
}

impl From<Pixels> for Padding {
    fn from(pixels: Pixels) -> Self {
        Self::all(pixels.0)
    }
}

impl From<f32> for Padding {
    fn from(amount: f32) -> Self {
        Self::all(amount)
    }
}

impl From<u16> for Padding {
    fn from(amount: u16) -> Self {
        Self::all(f32::from(amount))
    }
}

impl From<u32> for Padding {
    fn from(amount: u32) -> Self {
        Self::all(amount as f32)
    }
}

impl From<i32> for Padding {
    fn from(amount: i32) -> Self {
        Self::all(amount as f32)
    }
}

impl From<[f32; 2]> for Padding {
    fn from(amounts: [f32; 2]) -> Self {
        Self::axis(amounts[0], amounts[1])
    }
}

impl From<[u16; 2]> for Padding {
    fn from(amounts: [u16; 2]) -> Self {
        Self::axis(f32::from(amounts[0]), f32::from(amounts[1]))
    }
}

impl From<[u32; 2]> for Padding {
    fn from(amounts: [u32; 2]) -> Self {
        Self::axis(amounts[0] as f32, amounts[1] as f32)
    }
}

impl From<[i32; 2]> for Padding {
    fn from(amounts: [i32; 2]) -> Self {
        Self::axis(amounts[0] as f32, amounts[1] as f32)
    }
}

impl From<[f32; 4]> for Padding {
    fn from(amounts: [f32; 4]) -> Self {
        Self::sides(amounts[0], amounts[1], amounts[2], amounts[3])
    }
}

impl From<[u16; 4]> for Padding {
    fn from(amounts: [u16; 4]) -> Self {
        Self::sides(
            f32::from(amounts[0]),
            f32::from(amounts[1]),
            f32::from(amounts[2]),
            f32::from(amounts[3]),
        )
    }
}

impl From<[u32; 4]> for Padding {
    fn from(amounts: [u32; 4]) -> Self {
        Self::sides(
            amounts[0] as f32,
            amounts[1] as f32,
            amounts[2] as f32,
            amounts[3] as f32,
        )
    }
}

impl From<[i32; 4]> for Padding {
    fn from(amounts: [i32; 4]) -> Self {
        Self::sides(
            amounts[0] as f32,
            amounts[1] as f32,
            amounts[2] as f32,
            amounts[3] as f32,
        )
    }
}
