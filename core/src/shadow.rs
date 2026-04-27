use iced_core::Vector;

use crate::Color;

/// A shadow.
///
/// Vendored from `iced_core::Shadow` so we can carry our OKLCH `Color`.
/// `Vector` and `f32` carry no color semantics and stay as iced's.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Shadow {
    /// The color of the shadow.
    pub color: Color,

    /// The offset of the shadow.
    pub offset: Vector,

    /// The blur radius of the shadow.
    pub blur_radius: f32,
}

impl From<Shadow> for iced_core::Shadow {
    fn from(s: Shadow) -> Self {
        iced_core::Shadow {
            color: s.color.into(),
            offset: s.offset,
            blur_radius: s.blur_radius,
        }
    }
}

impl From<iced_core::Shadow> for Shadow {
    fn from(s: iced_core::Shadow) -> Self {
        Shadow {
            color: s.color.into(),
            offset: s.offset,
            blur_radius: s.blur_radius,
        }
    }
}
