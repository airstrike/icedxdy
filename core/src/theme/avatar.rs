//! Avatar styling
//!
//! Mirrors shadcn nova's three-size avatar shape: `Sm` = 24px, `Default` =
//! 32px, `Lg` = 40px (`size-6`, `size-8`, `size-10`). The visual atom set
//! is intentionally small — there's no border / shadow story on a stock
//! avatar; the only atoms a Spec needs to know about are
//!
//! - the fallback bg (`bg-muted`) and fallback fg (`text-muted-foreground`),
//!   shared with our muted palette;
//! - a 2px outer ring (in group context) drawn as a `border-color =
//!   neutral.base` outset on the circular container.
//!
//! Avatar's container border is *circular* (PILL_RADIUS clamp); a separate
//! Spec layer is overkill at this stage, so the variants ship as
//! ready-to-use `(diameter, font_size, badge_diameter, badge_icon_size)`
//! geometry tuples plus a single `style(theme)` resolver for the fallback
//! container.

use super::Theme;
use super::container::Style;
use crate::{Border, Color};

/// Pill-shape radius — clamps the avatar to a perfect circle regardless of
/// pixel diameter. Same trick `theme::badge` uses.
const PILL_RADIUS: f32 = u32::MAX as f32;

/// Avatar size variant. Geometry only — visual styling is shared across
/// variants and lives in [`fallback_style`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Small (24px) avatar variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sm;

/// Large (40px) avatar variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lg;

/// Geometry for an avatar at a given size.
///
/// All values in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geometry {
    /// Diameter of the circular avatar surface (`size-6` / `size-8` / `size-10`).
    pub diameter: f32,
    /// Font size for the fallback initials text. Matches shadcn's
    /// `text-sm` (14) on default/lg and `text-xs` (12) on sm.
    pub font_size: f32,
    /// Diameter of the AvatarBadge dot when shown in this avatar's corner.
    /// Matches shadcn's `size-2` / `size-2.5` / `size-3` for sm/default/lg.
    pub badge_diameter: f32,
    /// Edge length of an icon rendered inside the AvatarBadge for this
    /// avatar. Matches shadcn's `[&>svg]:size-2` cascade — note that on
    /// the `sm` size shadcn hides the icon entirely (`[&>svg]:hidden`);
    /// callers should treat `badge_icon_size == 0.0` as the hide signal.
    pub badge_icon_size: f32,
}

impl Geometry {
    /// Default-size geometry. Public for downstream callers that need to
    /// reach the same constants the widget uses.
    pub const DEFAULT: Self = Self {
        diameter: 32.0,
        font_size: 14.0,
        badge_diameter: 10.0,
        badge_icon_size: 8.0,
    };

    /// Small-size geometry.
    pub const SM: Self = Self {
        diameter: 24.0,
        font_size: 12.0,
        badge_diameter: 8.0,
        badge_icon_size: 0.0,
    };

    /// Large-size geometry.
    pub const LG: Self = Self {
        diameter: 40.0,
        font_size: 14.0,
        badge_diameter: 12.0,
        badge_icon_size: 8.0,
    };
}

impl From<Default> for Geometry {
    fn from(_: Default) -> Self {
        Self::DEFAULT
    }
}

impl From<Sm> for Geometry {
    fn from(_: Sm) -> Self {
        Self::SM
    }
}

impl From<Lg> for Geometry {
    fn from(_: Lg) -> Self {
        Self::LG
    }
}

/// Container style for the avatar's circular surface.
///
/// Mirrors shadcn `bg-muted text-muted-foreground` for the fallback layer.
/// `ring` controls the 2px outer border that AvatarGroup adds via
/// `*:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background` —
/// pass `true` from `avatar::group` and `false` for stand-alone avatars.
pub fn fallback_style(theme: &Theme, ring: bool) -> Style {
    let p = theme;
    Style {
        background: Some(p.muted.base.into()),
        text_color: Some(p.muted.foreground),
        border: Border {
            radius: PILL_RADIUS.into(),
            // Shadcn's group ring is 2px and matches the page background so
            // overlapping avatars look visually separated. Outside group
            // context, no border (the bare `Avatar` ships an inset
            // `after:border` 1px with `mix-blend-darken` in shadcn that we
            // intentionally skip — the blend mode doesn't replicate cleanly
            // in iced and the visual delta is negligible against the muted
            // fallback bg).
            width: if ring { 2.0 } else { 0.0 },
            color: if ring {
                p.neutral.base
            } else {
                Color::TRANSPARENT
            },
        },
        ..Style::default()
    }
}

/// Style for the AvatarBadge corner dot — shadcn `bg-primary
/// text-primary-foreground ring-2 ring-background` (the inner badge always
/// has a 2px ring against the page background so it reads as a separate
/// dot punched into the avatar corner).
pub fn badge_style(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.primary.base.into()),
        text_color: Some(p.primary.foreground),
        border: Border {
            radius: PILL_RADIUS.into(),
            width: 2.0,
            color: p.neutral.base,
        },
        ..Style::default()
    }
}

/// Style for the AvatarGroupCount circle — same surface as a fallback
/// avatar plus the 2px page-bg ring (group context is implied by the
/// existence of a count).
pub fn group_count_style(theme: &Theme) -> Style {
    fallback_style(theme, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pill-radius clamp resolves to the same large value across all
    /// variants — geometry is the only difference between them.
    #[test]
    fn fallback_style_radius_is_pill() {
        let theme = Theme::dark();
        let s = fallback_style(&theme, false);
        assert!(s.background.is_some());
        // The radius is a `Radius` of 4 corners; the absolute value is
        // PILL_RADIUS but we only need to confirm it's the pill clamp,
        // not a fractional/rounded-rectangle radius.
        let r: f32 = s.border.radius.top_left;
        assert!(r >= PILL_RADIUS, "expected pill-clamped radius, got {r}",);
    }

    /// In-group avatars carry a 2px ring; stand-alone avatars do not.
    #[test]
    fn ring_toggles_border_width() {
        let theme = Theme::dark();
        assert_eq!(fallback_style(&theme, false).border.width, 0.0);
        assert_eq!(fallback_style(&theme, true).border.width, 2.0);
    }
}
