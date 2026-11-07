//! Color types with OKLCH support
//!
//! shadcn/ui uses OKLCH color space for perceptually uniform colors.
//! OKLCH format: oklch(lightness chroma hue)
//! - lightness: 0.0 (black) to 1.0 (white)
//! - chroma: 0.0 (gray) to ~0.4 (vivid)
//! - hue: 0-360 degrees

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
    /// Fully opaque black (`oklch(0 0 0)`).
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);

    /// Fully opaque white (`oklch(1 0 0)`).
    pub const WHITE: Self = Self::new(1.0, 0.0, 0.0);

    /// Fully transparent. Matches `iced::Color::default()` — useful as a
    /// neutral filler for `Border`/`Shadow` defaults (a 0-alpha shadow
    /// renders as nothing, regardless of color).
    pub const TRANSPARENT: Self = Self::with_alpha(0.0, 0.0, 0.0, 0.0);

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

    /// Multiply the alpha channel by `factor` (clamped to [0.0, 1.0]).
    /// Mirrors `iced_core::Color::scale_alpha` so vendored types behave
    /// identically.
    pub const fn scale_alpha(mut self, factor: f32) -> Self {
        let scaled = self.a * factor;
        // const-clamp without f32::clamp (not const)
        self.a = if scaled < 0.0 {
            0.0
        } else if scaled > 1.0 {
            1.0
        } else {
            scaled
        };
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

    /// Rotate the hue by `degrees`. Wraps into [0, 360).
    pub fn shift_hue(mut self, degrees: f32) -> Self {
        self.h = (self.h + degrees).rem_euclid(360.0);
        self
    }

    /// Reduce chroma by `amount` (0.0 keeps, 1.0 fully greys out).
    pub fn desaturate(mut self, amount: f32) -> Self {
        self.c = (self.c * (1.0 - amount.clamp(0.0, 1.0))).max(0.0);
        self
    }

    /// Increase chroma by `amount` (0.0 keeps, 1.0 doubles).
    pub fn saturate(mut self, amount: f32) -> Self {
        self.c = (self.c * (1.0 + amount.max(0.0))).min(0.4);
        self
    }

    /// Interpolate toward `other` by `t` (0.0 = self, 1.0 = other). Hue
    /// takes the shorter arc around the wheel — blending blue (245°)
    /// with red (22°) at t=0.3 lands in the purple range, not by going
    /// the long way through green.
    pub fn blend(mut self, other: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        self.l += (other.l - self.l) * t;
        self.c += (other.c - self.c) * t;
        self.a += (other.a - self.a) * t;
        let mut diff = (other.h - self.h).rem_euclid(360.0);
        if diff > 180.0 {
            diff -= 360.0;
        }
        self.h = (self.h + diff * t).rem_euclid(360.0);
        self
    }

    /// Convert sRGB (gamma-encoded, [0, 1]) + alpha back to OKLCH.
    /// Inverse of `to_srgb` — sRGB → linear sRGB → OKLab → OKLCH per
    /// CSS Color Level 4.
    ///
    /// **Round-trip fidelity** depends on whether the source OKLCH is
    /// inside sRGB's gamut:
    /// - In-gamut: returns within ~3e-3 of the original (f32 + gamma noise).
    /// - Out-of-gamut: `to_srgb` clamps to [0, 1] and information is lost;
    ///   the reverse lands on the gamut boundary, not the original. Several
    ///   of our palette tokens (e.g. `info`, `success`) sit at chroma high
    ///   enough to clip — this is intentional (they're authored in OKLCH
    ///   for perceptual consistency across themes), but it means an
    ///   `iced::Color::into() -> Color::into() -> iced::Color` round trip
    ///   isn't bit-stable for those tokens. Don't rely on it.
    // OKLab matrix constants are kept at the precision the spec publishes.
    #[allow(clippy::excessive_precision)]
    pub fn from_srgb(c: iced_core::Color) -> Self {
        // sRGB to linear sRGB (inverse gamma)
        fn srgb_to_linear(x: f32) -> f32 {
            if x <= 0.04045 {
                x / 12.92
            } else {
                ((x + 0.055) / 1.055).powf(2.4)
            }
        }

        let r = srgb_to_linear(c.r);
        let g = srgb_to_linear(c.g);
        let b = srgb_to_linear(c.b);

        // Linear sRGB to OKLab (CSS Color Level 4 matrix + cube root)
        let l_ = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
        let m_ = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
        let s_ = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

        let l_cbrt = l_.cbrt();
        let m_cbrt = m_.cbrt();
        let s_cbrt = s_.cbrt();

        let lab_l = 0.2104542553 * l_cbrt + 0.7936177850 * m_cbrt - 0.0040720468 * s_cbrt;
        let lab_a = 1.9779984951 * l_cbrt - 2.4285922050 * m_cbrt + 0.4505937099 * s_cbrt;
        let lab_b = 0.0259040371 * l_cbrt + 0.7827717662 * m_cbrt - 0.8086757660 * s_cbrt;

        // OKLab to OKLCH (polar coords). atan2(0, 0) = 0 in IEEE 754, so
        // achromatic colors get hue = 0 by convention — caveat accepted.
        let chroma = (lab_a * lab_a + lab_b * lab_b).sqrt();
        let hue_deg = lab_b.atan2(lab_a).to_degrees();
        let hue = if hue_deg < 0.0 {
            hue_deg + 360.0
        } else {
            hue_deg
        };

        Self {
            l: lab_l,
            c: chroma,
            h: hue,
            a: c.a,
        }
    }

    /// Convert OKLCH to sRGB
    /// Based on the CSS Color Level 4 specification
    // OKLab matrix constants are kept at the precision the spec publishes.
    #[allow(clippy::excessive_precision)]
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

impl From<Color> for iced_core::Color {
    fn from(color: Color) -> Self {
        let (r, g, b) = color.to_srgb();
        iced_core::Color::from_rgba(r, g, b, color.a)
    }
}

impl From<iced_core::Color> for Color {
    fn from(c: iced_core::Color) -> Self {
        Self::from_srgb(c)
    }
}

impl From<Color> for iced_core::Background {
    fn from(color: Color) -> Self {
        iced_core::Background::Color(color.into())
    }
}

impl Default for Color {
    fn default() -> Self {
        // Match `iced::Color`'s auto-derived default — fully transparent.
        // This is what makes `Border::default()` / `Shadow::default()`
        // invisible (alpha=0), so unused defaults don't paint stray
        // outlines or boxes around widgets.
        Color::TRANSPARENT
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

    fn assert_color_close(a: Color, b: Color, tol: f32) {
        assert!((a.l - b.l).abs() < tol, "L mismatch: {a:?} vs {b:?}",);
        // Below this chroma threshold, hue is unstable (atan2 noise) — only
        // compare chroma. Above it, also compare hue with wrap.
        if a.c > 1e-3 && b.c > 1e-3 {
            assert!((a.c - b.c).abs() < tol, "C mismatch: {a:?} vs {b:?}",);
            let dh = (a.h - b.h).abs();
            let dh = dh.min(360.0 - dh);
            assert!(dh < 0.1, "H mismatch: {a:?} vs {b:?} (Δ {dh})");
        }
        assert!((a.a - b.a).abs() < tol, "alpha mismatch: {a:?} vs {b:?}",);
    }

    // Round-trip tolerance: f32 precision + gamma nonlinearity gives ~3e-3
    // worst-case delta on chromatic colors. Visually imperceptible (well
    // below 1 ΔE perceptual unit). Caveat documented in `from_srgb`.
    const ROUND_TRIP_TOL: f32 = 5e-3;

    #[test]
    fn srgb_round_trip_neutrals() {
        for original in [
            oklch(1.0, 0.0, 0.0),   // white
            oklch(0.5, 0.0, 0.0),   // mid gray
            oklch(0.145, 0.0, 0.0), // palette.dark.background
            oklch(0.0, 0.0, 0.0),   // black
        ] {
            let iced: iced_core::Color = original.into();
            let back: Color = iced.into();
            assert_color_close(original, back, ROUND_TRIP_TOL);
        }
    }

    #[test]
    fn srgb_round_trip_chromatic_in_gamut() {
        // Deliberately low-chroma so the colors stay inside sRGB's gamut.
        // Higher-chroma palette tokens (info/success/warning) clip; their
        // round-trip behavior is documented in `from_srgb`.
        for original in [
            oklch(0.50, 0.08, 30.0),  // dusty red
            oklch(0.60, 0.07, 120.0), // muted green
            oklch(0.55, 0.09, 250.0), // soft blue
        ] {
            let iced: iced_core::Color = original.into();
            let back: Color = iced.into();
            assert_color_close(original, back, ROUND_TRIP_TOL);
        }
    }

    /// Out-of-gamut OKLCH colors clip on the forward conversion. Verify
    /// the reverse lands on the gamut boundary deterministically (no
    /// NaN / inf), even if it doesn't match the original.
    #[test]
    fn srgb_round_trip_out_of_gamut_is_finite() {
        let original = oklch(0.55, 0.20, 245.0); // light.info — clips
        let iced: iced_core::Color = original.into();
        let back: Color = iced.into();
        assert!(back.l.is_finite() && back.c.is_finite() && back.h.is_finite());
        assert!(back.l >= 0.0 && back.l <= 1.0);
    }

    #[test]
    fn srgb_round_trip_with_alpha() {
        let original = oklch_alpha(1.0, 0.0, 0.0, 0.15); // palette.dark.input
        let iced: iced_core::Color = original.into();
        let back: Color = iced.into();
        assert!(
            (back.a - 0.15).abs() < 1e-3,
            "alpha must round-trip: {back:?}"
        );
    }
}
