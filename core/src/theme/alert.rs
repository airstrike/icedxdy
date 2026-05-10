//! Alert container styling
//!
//! `Style` is OKLCH-native — alert reuses container's `Style` field-for-field.
//! Variant fns + Spec/atom apply tweaks before conversion.

use std::ops::Add;

use super::Theme;
use super::container::Style;
use super::tweak::{Tweak, TweakSet};
use crate::{Background, Border, radius};

// --- Variant fns -------------------------------------------------------------

/// Default alert: card surface, standard border, rounded-lg, card-foreground text.
pub fn default(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.card.base.into()),
        text_color: Some(p.card.foreground),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.neutral.border,
        },
        ..Style::default()
    }
}

/// Destructive alert: card bg, destructive border + text.
pub fn destructive(theme: &Theme) -> Style {
    let p = theme;
    let is_dark = p.neutral.base.l < 0.5;
    let border_color = if is_dark {
        p.destructive.base
    } else {
        p.destructive.base.alpha(0.5)
    };
    Style {
        background: Some(p.card.base.into()),
        text_color: Some(p.destructive.base),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: border_color,
        },
        ..Style::default()
    }
}

// --- Variant unit structs ----------------------------------------------------

/// Default alert variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Destructive alert variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Destructive;

// --- Spec --------------------------------------------------------------------

/// An alert style: a base variant function plus a sequence of tweaks
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

impl From<Destructive> for Spec {
    fn from(_: Destructive) -> Spec {
        Spec::variant(destructive)
    }
}

macro_rules! variant_add {
    ($($variant:ident),* $(,)?) => {
        $(
            impl<T: Into<Tweak>> Add<T> for $variant {
                type Output = Spec;
                fn add(self, rhs: T) -> Spec {
                    Spec::from(self).push(rhs.into())
                }
            }
        )*
    };
}

variant_add!(Default, Destructive);

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        self.push(rhs.into())
    }
}

// --- Resolver ----------------------------------------------------------------
//
// Alert has no status, so `Tweak::State(...)` silently drops — there's no
// hover/pressed/disabled to react to. Identical body to container.rs::apply.

fn apply(tweak: &Tweak, style: &mut Style, theme: &Theme) {
    match tweak {
        Tweak::Bg(t) => {
            style.background = Some(t.resolve(theme).into());
        }
        Tweak::Fg(t) => {
            style.text_color = Some(t.resolve(theme));
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
        Tweak::Shadow(s) => {
            style.shadow = (*s).into();
        }
        Tweak::Opaque => {
            if let Some(Background::Color(src)) = style.background {
                let dst = theme.neutral.base;
                let composed = src.blend(dst, 1.0 - src.a).alpha(1.0);
                style.background = Some(Background::Color(composed));
            }
        }
        Tweak::Alpha(factor) => {
            if let Some(bg) = style.background {
                style.background = Some(bg.scale_alpha(*factor));
            }
        }
        // No status → state combinators silently drop.
        Tweak::State(_, _) => {}
        // CSS-style cascade: tweaks with no container counterpart drop.
        Tweak::Ring { .. }
        | Tweak::Size(_)
        | Tweak::Weight(_)
        | Tweak::Family(_)
        | Tweak::LineHeight(_)
        | Tweak::Italic
        | Tweak::Align(_) => {}
    }
}

// --- Bridge to iced::container::StyleFn --------------------------------------

impl<'a> From<Default> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(|t| default(t).into())
    }
}

impl<'a> From<Destructive> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: Destructive) -> Self {
        Box::new(|t| destructive(t).into())
    }
}

impl<'a> From<Spec> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |t| spec.resolve(t).into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tweak::{Bg, Token};
    use super::*;
    use crate::{Background, Color};

    fn bg_color(style: &Style) -> Color {
        match style.background {
            Some(Background::Color(c)) => c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    /// `Default` resolves to `palette.card.base` background.
    #[test]
    fn default_uses_card_background() {
        let theme = Theme::dark();
        let resolved = default(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.card.base;
        assert!(
            (bg.l - expected.l).abs() < 1e-4
                && (bg.c - expected.c).abs() < 1e-4
                && (bg.a - expected.a).abs() < 1e-4,
            "expected card.base, got {bg:?}",
        );
    }

    /// `Bg` atom on `Default` overrides the variant background.
    #[test]
    fn bg_atom_overrides_default_variant() {
        let theme = Theme::dark();
        let spec: Spec = (Default + Bg(Token::Primary)).into();
        let resolved = spec.resolve(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.primary.base;
        assert!(
            (bg.l - expected.l).abs() < 1e-4
                && (bg.c - expected.c).abs() < 1e-4
                && (bg.a - expected.a).abs() < 1e-4,
            "expected primary.base, got {bg:?}",
        );
    }
}
