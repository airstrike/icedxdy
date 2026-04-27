use iced_core::Gradient;
use iced_core::gradient;

use crate::Color;

/// The background of some element.
///
/// Vendored from `iced_core::Background` so the `Color` arm can carry our
/// OKLCH `Color`. The `Gradient` arm wraps iced's gradient as-is — we don't
/// vendor `gradient::*` (much larger surface) and gradients pass through to
/// the renderer untouched.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Background {
    /// A solid color.
    Color(Color),
    /// Linearly interpolate between several colors.
    Gradient(Gradient),
    // TODO: Add image variant
}

impl Background {
    /// Scales the alpha channel of the [`Background`] by the given
    /// factor.
    pub fn scale_alpha(self, factor: f32) -> Self {
        match self {
            Self::Color(color) => Self::Color(color.scale_alpha(factor)),
            Self::Gradient(gradient) => Self::Gradient(gradient.scale_alpha(factor)),
        }
    }
}

impl From<Color> for Background {
    fn from(color: Color) -> Self {
        Background::Color(color)
    }
}

impl From<Gradient> for Background {
    fn from(gradient: Gradient) -> Self {
        Background::Gradient(gradient)
    }
}

impl From<gradient::Linear> for Background {
    fn from(gradient: gradient::Linear) -> Self {
        Background::Gradient(Gradient::Linear(gradient))
    }
}

impl From<gradient::Radial> for Background {
    fn from(gradient: gradient::Radial) -> Self {
        Background::Gradient(Gradient::Radial(gradient))
    }
}

impl From<gradient::Conic> for Background {
    fn from(gradient: gradient::Conic) -> Self {
        Background::Gradient(Gradient::Conic(gradient))
    }
}

impl From<Background> for iced_core::Background {
    fn from(bg: Background) -> Self {
        match bg {
            Background::Color(c) => iced_core::Background::Color(c.into()),
            Background::Gradient(g) => iced_core::Background::Gradient(g),
        }
    }
}

impl From<iced_core::Background> for Background {
    fn from(bg: iced_core::Background) -> Self {
        match bg {
            iced_core::Background::Color(c) => Background::Color(c.into()),
            iced_core::Background::Gradient(g) => Background::Gradient(g),
        }
    }
}
