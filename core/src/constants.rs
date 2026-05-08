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
///
/// # `GAP_N` aliases
///
/// `GAP_1..GAP_6` mirror Tailwind's `gap-N` scale verbatim (`gap-N = N × 4px`).
/// Reach for these at `.spacing(...)` call sites that translate a shadcn
/// classlist literally (`gap-2`, `gap-4`, …) — the names are scannable next
/// to the source they're porting and need no translation table. The `XS / SM
/// / MD / LG / …` aliases above are kept for callsites that read in semantic
/// (rather than tailwind-numeric) terms.
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

    // Tailwind-numeric aliases — see module docs.

    /// `gap-1` — 4px (Tailwind `gap-1`).
    pub const GAP_1: f32 = 4.0;
    /// `gap-2` — 8px (Tailwind `gap-2`).
    pub const GAP_2: f32 = 8.0;
    /// `gap-3` — 12px (Tailwind `gap-3`).
    pub const GAP_3: f32 = 12.0;
    /// `gap-4` — 16px (Tailwind `gap-4`).
    pub const GAP_4: f32 = 16.0;
    /// `gap-5` — 20px (Tailwind `gap-5`).
    pub const GAP_5: f32 = 20.0;
    /// `gap-6` — 24px (Tailwind `gap-6`).
    pub const GAP_6: f32 = 24.0;
}

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
