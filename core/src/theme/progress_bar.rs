//! Progress-bar styling
//!
//! `Style` is OKLCH-native — variants and tweaks operate on our `Color`,
//! `Border`, `Background` end-to-end. Conversion to iced's sRGB types
//! happens once at the catalog boundary. Mirrors the structure of
//! `theme::container` (no Status — progress is stateless).

use std::ops::Add;

use super::Theme;
use super::tweak::{Tweak, TweakSet};
use crate::{Background, Border, Color};

/// Pill-shape radius — clamped to a huge value so the bar renders as a
/// fully rounded capsule regardless of size. Mirrors `theme::badge`.
const PILL_RADIUS: f32 = u32::MAX as f32;

/// OKLCH-native progress-bar style. Mirrors
/// `iced_widget::progress_bar::Style` field-for-field, but the
/// color-bearing fields use our types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The track behind the bar.
    pub background: Background,
    /// The filled portion of the bar.
    pub bar: Background,
    /// The bar's border.
    pub border: Border,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            background: Background::Color(Color::TRANSPARENT),
            bar: Background::Color(Color::TRANSPARENT),
            border: Border::default(),
        }
    }
}

/// A styling closure that returns our OKLCH-native `Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl From<Style> for iced_widget::progress_bar::Style {
    fn from(s: Style) -> Self {
        iced_widget::progress_bar::Style {
            background: s.background.into(),
            bar: s.bar.into(),
            border: s.border.into(),
        }
    }
}

impl From<iced_widget::progress_bar::Style> for Style {
    fn from(s: iced_widget::progress_bar::Style) -> Self {
        Style {
            background: s.background.into(),
            bar: s.bar.into(),
            border: s.border.into(),
        }
    }
}

impl iced_widget::progress_bar::Catalog for Theme {
    type Class<'a> = iced_widget::progress_bar::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|t| default(t).into())
    }

    fn style(&self, class: &Self::Class<'_>) -> iced_widget::progress_bar::Style {
        class(self)
    }
}

// --- Variant fns -------------------------------------------------------------

/// Nova default — pill-shaped track with `bg-muted`, fill with `bg-primary`.
pub fn default(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: p.muted.base.into(),
        bar: p.primary.base.into(),
        border: Border {
            radius: PILL_RADIUS.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
    }
}

// --- Variant unit structs ----------------------------------------------------

/// Default progress-bar variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

// --- Spec --------------------------------------------------------------------

/// A progress-bar style: a base variant function plus a sequence of tweaks
/// applied at resolve time.
#[derive(Debug, Clone)]
pub struct Spec {
    variant: fn(&Theme) -> Style,
    tweaks: TweakSet,
}

impl Spec {
    fn variant(f: fn(&Theme) -> Style) -> Spec {
        Spec {
            variant: f,
            tweaks: TweakSet::default(),
        }
    }

    fn push(mut self, t: Tweak) -> Spec {
        self.tweaks.0.push(t);
        self
    }

    /// Resolve this spec to a concrete `Style` for the given theme.
    pub fn resolve(&self, theme: &Theme) -> Style {
        let mut style = (self.variant)(theme);
        for t in &self.tweaks.0 {
            apply(t, &mut style, theme);
        }
        style
    }
}

impl From<Default> for Spec {
    fn from(_: Default) -> Spec {
        Spec::variant(default)
    }
}

impl<T: Into<Tweak>> Add<T> for Default {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        self.push(rhs.into())
    }
}

// --- Resolver ----------------------------------------------------------------
//
// Progress has no status, so `Tweak::State(...)` silently drops — there's no
// hover/pressed/disabled to react to.
//
// `Tweak::Bg` targets the track (the empty side of the bar); `Tweak::Fg`
// targets the filled indicator color (semantically the bar is the
// "foreground" of the progress widget).

fn apply(tweak: &Tweak, style: &mut Style, theme: &Theme) {
    match tweak {
        Tweak::Bg(t) => {
            style.background = t.resolve(theme).into();
        }
        Tweak::Fg(t) => {
            style.bar = t.resolve(theme).into();
        }
        Tweak::BorderColor(t) => {
            style.border.color = t.resolve(theme);
        }
        Tweak::BorderWidth(w) => {
            style.border.width = *w;
        }
        Tweak::Radius(r) => {
            style.border.radius = (*r).into();
        }
        Tweak::Opaque => {
            // Same OKLCH alpha-compositing as container — bake any translucent
            // track bg into a solid color over palette.background. Track-only
            // to match container's bg-only Opaque policy.
            if let Background::Color(src) = style.background {
                let dst = theme.neutral.base;
                let composed = src.blend(dst, 1.0 - src.a).alpha(1.0);
                style.background = Background::Color(composed);
            }
        }
        Tweak::Alpha(factor) => {
            // Track-only — matches container's bg-only Alpha policy.
            style.background = style.background.scale_alpha(*factor);
        }
        // No status → state combinators silently drop.
        Tweak::State(_, _) => {}
        // CSS-style cascade: tweaks with no progress-bar counterpart drop.
        Tweak::Ring { .. }
        | Tweak::Shadow(_)
        | Tweak::Size(_)
        | Tweak::Weight(_)
        | Tweak::Family(_)
        | Tweak::LineHeight(_)
        | Tweak::Italic
        | Tweak::Align(_) => {}
    }
}

// --- StyleFn conversions -----------------------------------------------------

impl<'a> From<Default> for StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(default)
    }
}

impl<'a> From<Spec> for StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |theme| spec.resolve(theme))
    }
}

// --- Bridge to iced::progress_bar::StyleFn -----------------------------------

impl<'a> From<Default> for iced_widget::progress_bar::StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(|t| default(t).into())
    }
}

impl<'a> From<Spec> for iced_widget::progress_bar::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |t| spec.resolve(t).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solid(bg: Background) -> Color {
        match bg {
            Background::Color(c) => c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    /// `default` track resolves to `palette.muted.base`, bar to
    /// `palette.primary.base`.
    #[test]
    fn default_uses_muted_track_and_primary_bar() {
        let theme = Theme::dark();
        let resolved = default(&theme);
        let track = solid(resolved.background);
        let bar = solid(resolved.bar);
        let p = theme;
        assert!(
            (track.l - p.muted.base.l).abs() < 1e-4
                && (track.c - p.muted.base.c).abs() < 1e-4
                && (track.a - p.muted.base.a).abs() < 1e-4,
            "track must be palette.muted.base, got {track:?}",
        );
        assert!(
            (bar.l - p.primary.base.l).abs() < 1e-4
                && (bar.c - p.primary.base.c).abs() < 1e-4
                && (bar.a - p.primary.base.a).abs() < 1e-4,
            "bar must be palette.primary.base, got {bar:?}",
        );
    }

    /// Border radius is the pill shape — not zero.
    #[test]
    fn default_border_is_pill_shaped() {
        let theme = Theme::dark();
        let resolved = default(&theme);
        assert!(
            resolved.border.radius.top_left > 1000.0,
            "expected pill radius, got {}",
            resolved.border.radius.top_left,
        );
    }
}
