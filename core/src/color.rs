//! Color types with OKLCH support
//!
//! shadcn/ui uses OKLCH color space for perceptually uniform colors.
//! OKLCH format: oklch(lightness chroma hue)
//! - lightness: 0.0 (black) to 1.0 (white)
//! - chroma: 0.0 (gray) to ~0.4 (vivid)
//! - hue: 0-360 degrees

use iced_core::Color as IcedColor;

/// A color in OKLCH color space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Lightness: 0.0 (black) to 1.0 (white)
    pub l: f32,
    /// Chroma: 0.0 (gray) to ~0.4 (vivid)
    pub c: f32,
    /// Hue: 0-360 degrees
    pub h: f32,
    /// Alpha: 0.0 (transparent) to 1.0 (opaque)
    pub a: f32,
}

impl Color {
    /// Create a new OKLCH color
    pub const fn new(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h, a: 1.0 }
    }

    /// Create a new OKLCH color with alpha
    pub const fn with_alpha(l: f32, c: f32, h: f32, a: f32) -> Self {
        Self { l, c, h, a }
    }

    /// Set the alpha value
    pub const fn alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    /// Lighten the color by a percentage (0.0 to 1.0)
    pub fn lighten(mut self, amount: f32) -> Self {
        self.l = (self.l + amount).min(1.0);
        self
    }

    /// Darken the color by a percentage (0.0 to 1.0)
    pub fn darken(mut self, amount: f32) -> Self {
        self.l = (self.l - amount).max(0.0);
        self
    }

    /// Convert OKLCH to sRGB
    /// Based on the CSS Color Level 4 specification
    fn to_srgb(self) -> (f32, f32, f32) {
        let l = self.l;
        let c = self.c;
        let h = self.h.to_radians();

        // OKLCH to OKLab
        let a = c * h.cos();
        let b = c * h.sin();

        // OKLab to linear sRGB
        let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
        let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
        let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

        let l3 = l_ * l_ * l_;
        let m3 = m_ * m_ * m_;
        let s3 = s_ * s_ * s_;

        let r = 4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3;
        let g = -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3;
        let b = -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3;

        // Linear sRGB to sRGB (gamma correction)
        fn linear_to_srgb(x: f32) -> f32 {
            if x <= 0.0031308 {
                x * 12.92
            } else {
                1.055 * x.powf(1.0 / 2.4) - 0.055
            }
        }

        (
            linear_to_srgb(r).clamp(0.0, 1.0),
            linear_to_srgb(g).clamp(0.0, 1.0),
            linear_to_srgb(b).clamp(0.0, 1.0),
        )
    }
}

impl From<Color> for IcedColor {
    fn from(color: Color) -> Self {
        let (r, g, b) = color.to_srgb();
        IcedColor::from_rgba(r, g, b, color.a)
    }
}

impl From<Color> for iced_core::Background {
    fn from(color: Color) -> Self {
        iced_core::Background::Color(color.into())
    }
}

impl Default for Color {
    fn default() -> Self {
        // Default to white
        Self::new(1.0, 0.0, 0.0)
    }
}

/// Create an OKLCH color
///
/// # Example
/// ```
/// use icedxdy_core::oklch;
/// let white = oklch(1.0, 0.0, 0.0);
/// let black = oklch(0.0, 0.0, 0.0);
/// ```
pub const fn oklch(l: f32, c: f32, h: f32) -> Color {
    Color::new(l, c, h)
}

/// Create an OKLCH color with alpha
///
/// # Example
/// ```
/// use icedxdy_core::oklch_alpha;
/// let semi_transparent_white = oklch_alpha(1.0, 0.0, 0.0, 0.5);
/// ```
pub const fn oklch_alpha(l: f32, c: f32, h: f32, a: f32) -> Color {
    Color::with_alpha(l, c, h, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white() {
        let white = oklch(1.0, 0.0, 0.0);
        let (r, g, b) = white.to_srgb();
        assert!((r - 1.0).abs() < 0.01);
        assert!((g - 1.0).abs() < 0.01);
        assert!((b - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_black() {
        let black = oklch(0.0, 0.0, 0.0);
        let (r, g, b) = black.to_srgb();
        assert!((r - 0.0).abs() < 0.01);
        assert!((g - 0.0).abs() < 0.01);
        assert!((b - 0.0).abs() < 0.01);
    }
}
