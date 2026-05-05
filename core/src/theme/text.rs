//! Text styling
//!
//! `Style` is OKLCH-native — variants and tweaks operate on our `Color`.
//! Conversion to iced's sRGB types happens once at the catalog boundary.
//!
//! Text only carries a `text_color`, so most tweak atoms (`Bg`, border,
//! `Radius`, `Shadow`, `Opaque`) silently drop. `Alpha` scales the
//! resolved text-color alpha (since text has no background).

use std::ops::Add;

use super::Theme;
use super::tweak::{Tweak, TweakSet};
use crate::{Color, Palette};

/// OKLCH-native text style. Mirrors `iced_widget::text::Style` field-for-field
/// using our `Color`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Style {
    /// Optional text color override; `None` inherits the parent.
    pub color: Option<Color>,
}

/// A styling closure that returns our OKLCH-native `Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl From<Style> for iced_widget::text::Style {
    fn from(s: Style) -> Self {
        iced_widget::text::Style {
            color: s.color.map(Into::into),
        }
    }
}

impl From<iced_widget::text::Style> for Style {
    fn from(s: iced_widget::text::Style) -> Self {
        Style {
            color: s.color.map(Into::into),
        }
    }
}

impl iced_widget::text::Catalog for Theme {
    type Class<'a> = iced_widget::text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|t| default(t).into())
    }

    fn style(&self, class: &Self::Class<'_>) -> iced_widget::text::Style {
        class(self)
    }
}

// --- Variant fns -------------------------------------------------------------

/// Inherit-from-parent text — no override.
pub fn default(_theme: &Theme) -> Style {
    Style::default()
}

/// Body foreground color.
pub fn foreground(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().foreground),
    }
}

/// Muted/secondary text color.
pub fn muted(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().muted.foreground),
    }
}

/// Primary-coloured link text.
pub fn link(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().primary.base),
    }
}

/// Destructive (error) text color.
pub fn destructive(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().destructive.base),
    }
}

// --- Variant unit structs ----------------------------------------------------

/// Default text variant — inherits parent color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Foreground text variant — body text color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Foreground;

/// Muted text variant — secondary text color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Muted;

/// Link text variant — primary brand color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link;

/// Destructive text variant — error/destructive color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Destructive;

// --- Spec --------------------------------------------------------------------

/// A text style: a base variant function plus a sequence of tweaks
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
        let palette = theme.palette();
        for t in &self.tweaks.0 {
            apply(t, &mut style, palette);
        }
        style
    }
}

impl From<Default> for Spec {
    fn from(_: Default) -> Spec {
        Spec::variant(default)
    }
}

impl From<Foreground> for Spec {
    fn from(_: Foreground) -> Spec {
        Spec::variant(foreground)
    }
}

impl From<Muted> for Spec {
    fn from(_: Muted) -> Spec {
        Spec::variant(muted)
    }
}

impl From<Link> for Spec {
    fn from(_: Link) -> Spec {
        Spec::variant(link)
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

variant_add!(Default, Foreground, Muted, Link, Destructive);

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        self.push(rhs.into())
    }
}

// --- Resolver ----------------------------------------------------------------
//
// Text only has a `color` field. `Fg` overrides it; `Alpha` scales its alpha
// (interpreted as text-color alpha since there's no background). Every other
// tweak silently drops (CSS-style cascade).

fn apply(tweak: &Tweak, style: &mut Style, palette: &Palette) {
    match tweak {
        Tweak::Fg(t) => {
            style.color = Some(t.resolve(palette));
        }
        Tweak::Alpha(factor) => {
            // Text has no background, so Alpha targets the text color.
            // No-op when color is `None` (parent-inherited).
            if let Some(c) = style.color {
                style.color = Some(c.scale_alpha(*factor));
            }
        }
        // Everything else: text has no background/border/shadow/state, and
        // typography atoms (Size, Weight, Family, LineHeight, Italic, Align)
        // are widget-builder concerns, not part of `iced::text::Style`.
        Tweak::Bg(_)
        | Tweak::BorderColor(_)
        | Tweak::BorderWidth(_)
        | Tweak::Radius(_)
        | Tweak::Shadow(_)
        | Tweak::Ring { .. }
        | Tweak::Opaque
        | Tweak::Size(_)
        | Tweak::Weight(_)
        | Tweak::Family(_)
        | Tweak::LineHeight(_)
        | Tweak::Italic
        | Tweak::Align(_)
        | Tweak::State(_, _) => {}
    }
}

// --- StyleFn conversions -----------------------------------------------------

impl<'a> From<Default> for StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(default)
    }
}

impl<'a> From<Foreground> for StyleFn<'a, Theme> {
    fn from(_: Foreground) -> Self {
        Box::new(foreground)
    }
}

impl<'a> From<Muted> for StyleFn<'a, Theme> {
    fn from(_: Muted) -> Self {
        Box::new(muted)
    }
}

impl<'a> From<Link> for StyleFn<'a, Theme> {
    fn from(_: Link) -> Self {
        Box::new(link)
    }
}

impl<'a> From<Destructive> for StyleFn<'a, Theme> {
    fn from(_: Destructive) -> Self {
        Box::new(destructive)
    }
}

impl<'a> From<Spec> for StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |theme| spec.resolve(theme))
    }
}

// --- Bridge to iced::text::StyleFn -------------------------------------------

impl<'a> From<Default> for iced_widget::text::StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(|t| default(t).into())
    }
}

impl<'a> From<Foreground> for iced_widget::text::StyleFn<'a, Theme> {
    fn from(_: Foreground) -> Self {
        Box::new(|t| foreground(t).into())
    }
}

impl<'a> From<Muted> for iced_widget::text::StyleFn<'a, Theme> {
    fn from(_: Muted) -> Self {
        Box::new(|t| muted(t).into())
    }
}

impl<'a> From<Link> for iced_widget::text::StyleFn<'a, Theme> {
    fn from(_: Link) -> Self {
        Box::new(|t| link(t).into())
    }
}

impl<'a> From<Destructive> for iced_widget::text::StyleFn<'a, Theme> {
    fn from(_: Destructive) -> Self {
        Box::new(|t| destructive(t).into())
    }
}

impl<'a> From<Spec> for iced_widget::text::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |t| spec.resolve(t).into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tweak::{Alpha, Fg, Token};
    use super::*;

    #[test]
    fn alpha_scales_resolved_text_color_alpha() {
        let theme = Theme::dark();
        let spec: Spec = (Foreground + Alpha(0.5)).into();
        let resolved = spec.resolve(&theme);
        let color = resolved.color.expect("foreground sets color");
        let expected = theme.palette().foreground.a * 0.5;
        assert!(
            (color.a - expected).abs() < 1e-4,
            "expected a={expected}, got a={}",
            color.a,
        );
    }

    #[test]
    fn fg_overrides_variant_color() {
        let theme = Theme::dark();
        let spec: Spec = (Default + Fg(Token::Primary)).into();
        let resolved = spec.resolve(&theme);
        let expected = theme.palette().primary.base;
        let actual = resolved.color.expect("Fg sets color");
        assert!(
            (actual.l - expected.l).abs() < 1e-4 && (actual.c - expected.c).abs() < 1e-4,
            "expected primary, got {actual:?}",
        );
    }
}
