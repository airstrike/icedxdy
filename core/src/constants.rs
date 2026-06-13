//! Design constants based on shadcn/ui
//!
//! These are the specific values from shadcn's design system.
//! Use iced's native Radius, Padding, etc. types with these values.

/// Border radius constants (in pixels)
///
/// Base radius is 10px, with variants calculated relative to it.
pub mod radius {
    /// 4px - extra small radius. shadcn checkbox uses this verbatim
    /// (`rounded-[4px]` — a literal pixel value, not a Tailwind preset).
    pub const XS: f32 = 4.0;
    /// 6px - small radius
    pub const SM: f32 = 6.0;
    /// 8px - medium radius
    pub const MD: f32 = 8.0;
    /// 10px - large/default radius
    pub const LG: f32 = 10.0;
    /// 14px - extra large
    pub const XL: f32 = 14.0;
    /// 18px
    pub const XL2: f32 = 18.0;
    /// 22px
    pub const XL3: f32 = 22.0;
    /// 26px
    pub const XL4: f32 = 26.0;
}

/// Spacing constants (in pixels)
///
/// Based on Tailwind's 4px base unit.
pub mod spacing {
    pub const UNIT: f32 = 4.0;
    /// 2px
    pub const XS2: f32 = 2.0;
    /// 4px
    pub const XS: f32 = 4.0;
    /// 6px
    pub const SM: f32 = 6.0;
    /// 8px
    pub const MD: f32 = 8.0;
    /// 10px
    pub const LG: f32 = 10.0;
    /// 12px
    pub const XL: f32 = 12.0;
    /// 16px
    pub const XL2: f32 = 16.0;
    /// 24px
    pub const XL4: f32 = 24.0;
    /// 32px
    pub const XL5: f32 = 32.0;
}

/// A spacing scalar — Tailwind `gap-N` / `p-N` units mapped to pixels.
///
/// `Space::U4` reads as "Tailwind spacing unit 4" (`16px`) and can be
/// used anywhere iced accepts `Pixels`; it also converts to uniform
/// `Padding` for `p-N` call sites. `Gap::G4` remains as the gap-specific
/// spelling for source ports that mirror `gap-4` literally.
///
/// Wraps a raw `f32` so iced builders that take `impl Into<Pixels>`
/// (`Column::spacing`, `Padding::top`, `Padding::ZERO.bottom`, …) accept
/// `Space::U2` / `Gap::G2` directly via the `From<Gap> for Pixels` impl
/// below. Custom values escape via the public `f32` field (`Gap(13.0)`)
/// or the `From<f32>` impl that mirrors `Pixels` itself.
///
/// Modeled as a struct with associated constants (rather than an enum)
/// for parity with iced's own scalar wrappers (`Pixels`, `Em`) and to
/// avoid a `Custom(f32)` arm cluttering pattern matches we don't need
/// — every consumer just converts to `Pixels` and forgets the original.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Gap(pub f32);

impl Gap {
    /// Raw logical pixels for struct literals that require `f32` fields.
    pub const fn px(self) -> f32 {
        self.0
    }

    /// `0` — 0px.
    pub const U0: Gap = Gap(0.0);
    /// `0.5` — 2px.
    pub const U0_5: Gap = Gap(2.0);
    /// `1` — 4px.
    pub const U1: Gap = Gap(4.0);
    /// `1.5` — 6px.
    pub const U1_5: Gap = Gap(6.0);
    /// `2` — 8px.
    pub const U2: Gap = Gap(8.0);
    /// `2.5` — 10px.
    pub const U2_5: Gap = Gap(10.0);
    /// `3` — 12px.
    pub const U3: Gap = Gap(12.0);
    /// `4` — 16px.
    pub const U4: Gap = Gap(16.0);
    /// `5` — 20px.
    pub const U5: Gap = Gap(20.0);
    /// `6` — 24px.
    pub const U6: Gap = Gap(24.0);
    /// `7` — 28px.
    pub const U7: Gap = Gap(28.0);
    /// `8` — 32px.
    pub const U8: Gap = Gap(32.0);

    /// `gap-1` — 4px.
    pub const G1: Gap = Gap(4.0);
    /// `gap-2` — 8px.
    pub const G2: Gap = Gap(8.0);
    /// `gap-3` — 12px.
    pub const G3: Gap = Gap(12.0);
    /// `gap-4` — 16px.
    pub const G4: Gap = Gap(16.0);
    /// `gap-5` — 20px.
    pub const G5: Gap = Gap(20.0);
    /// `gap-6` — 24px.
    pub const G6: Gap = Gap(24.0);
    /// `gap-7` — 28px.
    pub const G7: Gap = Gap(28.0);
    /// `gap-8` — 32px.
    pub const G8: Gap = Gap(32.0);
}

impl From<f32> for Gap {
    fn from(amount: f32) -> Self {
        Gap(amount)
    }
}

impl From<Gap> for f32 {
    fn from(gap: Gap) -> Self {
        gap.0
    }
}

impl From<Gap> for iced_core::Pixels {
    fn from(gap: Gap) -> Self {
        iced_core::Pixels(gap.0)
    }
}

impl From<Gap> for iced_core::Padding {
    fn from(gap: Gap) -> Self {
        iced_core::Padding::new(gap.0)
    }
}

/// Semantic alias for call sites that are not specifically translating a
/// `gap-*` class, such as `p-4`, `pt-6`, or component layout constants.
pub type Space = Gap;

/// Font size constants (in pixels)
pub mod font_size {
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
}

/// Component height constants (in pixels)
///
/// Based on shadcn button sizes.
pub mod height {
    /// 24px - xs
    pub const XS: f32 = 24.0;
    /// 32px - sm
    pub const SM: f32 = 32.0;
    /// 36px - default
    pub const MD: f32 = 36.0;
    /// 40px - lg
    pub const LG: f32 = 40.0;
}

/// Icon size constants (in pixels)
pub mod icon_size {
    /// 12px
    pub const XS: f32 = 12.0;
    /// 16px
    pub const SM: f32 = 16.0;
    /// 20px
    pub const MD: f32 = 20.0;
    /// 24px
    pub const LG: f32 = 24.0;
}

/// Focus ring width
pub const RING_WIDTH: f32 = 3.0;

/// Default transition duration in milliseconds
pub const TRANSITION_DURATION_MS: u64 = 150;
