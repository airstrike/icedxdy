//! Badge styling
//!
//! `Style` is OKLCH-native — badge reuses container's `Style` field-for-field.
//! Variant fns + Spec/atom apply tweaks before conversion.

use std::ops::Add;

use super::Theme;
use super::container::Style;
use super::tweak::{Tweak, TweakSet};
use crate::palette::Palette;
use crate::style;
use crate::style::status;
use crate::{Background, Border};

/// Pill-shape radius — clamped to a huge value so the container renders as a
/// fully rounded capsule regardless of size.
const PILL_RADIUS: f32 = u32::MAX as f32;

// --- Variant fns -------------------------------------------------------------

/// Default: `bg-primary` text-`primary-foreground`.
pub fn default(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.primary.base.into()),
        text_color: Some(p.primary.foreground),
        border: Border {
            radius: PILL_RADIUS.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

/// Secondary: `bg-secondary` text-`secondary-foreground`.
pub fn secondary(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.secondary.base.into()),
        text_color: Some(p.secondary.foreground),
        border: Border {
            radius: PILL_RADIUS.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

/// Destructive: `bg-destructive/10 dark:bg-destructive/20` text-`destructive`.
///
/// Nova spec: `bg-destructive/10 text-destructive dark:bg-destructive/20` — fill
/// is the destructive hue at 10% alpha (light) / 20% (dark); text is the
/// destructive hue at full opacity (not `destructive-foreground`).
pub fn destructive(theme: &Theme) -> Style {
    let p = theme;
    let is_dark = p.neutral.base.l < 0.5;
    let alpha = if is_dark { 0.20 } else { 0.10 };
    Style {
        background: Some(p.destructive.base.scale_alpha(alpha).into()),
        text_color: Some(p.destructive.base),
        border: Border {
            radius: PILL_RADIUS.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

/// Outline: transparent bg, foreground text, 1px border.
pub fn outline(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: None,
        text_color: Some(p.neutral.foreground),
        border: Border {
            radius: PILL_RADIUS.into(),
            width: 1.0,
            color: p.neutral.border,
        },
        ..Style::default()
    }
}

/// Ghost: no background, no border, foreground text.
pub fn ghost(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: None,
        text_color: Some(p.neutral.foreground),
        border: Border {
            radius: PILL_RADIUS.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

// --- Variant unit structs ----------------------------------------------------

/// Default badge variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Secondary badge variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Secondary;

/// Destructive badge variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Destructive;

/// Outline badge variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outline;

/// Ghost badge variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ghost;

// --- Spec --------------------------------------------------------------------

/// A badge style: a base variant function plus a sequence of tweaks
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

impl From<Secondary> for Spec {
    fn from(_: Secondary) -> Spec {
        Spec::variant(secondary)
    }
}

impl From<Destructive> for Spec {
    fn from(_: Destructive) -> Spec {
        Spec::variant(destructive)
    }
}

impl From<Outline> for Spec {
    fn from(_: Outline) -> Spec {
        Spec::variant(outline)
    }
}

impl From<Ghost> for Spec {
    fn from(_: Ghost) -> Spec {
        Spec::variant(ghost)
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

variant_add!(Default, Secondary, Destructive, Outline, Ghost);

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        self.push(rhs.into())
    }
}

// --- Resolver ----------------------------------------------------------------
//
// Badge has no status, so `Tweak::State(...)` silently drops. Identical body
// to container.rs::apply.

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
            style.border.radius = *r;
        }
        Tweak::Shadow(s) => {
            style.shadow = *s;
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
        Tweak::State(_, _) => {}
        Tweak::Ring { .. }
        | Tweak::Size(_)
        | Tweak::Weight(_)
        | Tweak::Family(_)
        | Tweak::LineHeight(_)
        | Tweak::Italic
        | Tweak::Align(_) => {}
    }
}

/// Badge intrinsic default in the universal Style shape: pill radius only.
/// Tags overlay fill / text / border on top.
pub fn intrinsic_default(_theme: &Theme) -> style::Style {
    style::Style {
        border_radius: Some(PILL_RADIUS.into()),
        ..style::Style::default()
    }
}

fn default_impl(p: Palette, _s: status::Status) -> style::Style {
    style::Style {
        fill: Some(p.base.into()),
        text_color: Some(p.foreground),
        border_radius: Some(PILL_RADIUS.into()),
        ..style::Style::default()
    }
}

fn secondary_impl(p: Palette, _s: status::Status) -> style::Style {
    style::Style {
        fill: Some(p.base.into()),
        text_color: Some(p.foreground),
        border_radius: Some(PILL_RADIUS.into()),
        ..style::Style::default()
    }
}

fn destructive_impl(p: Palette, _s: status::Status) -> style::Style {
    let alpha = if p.is_dark { 0.20 } else { 0.10 };
    style::Style {
        fill: Some(p.base.scale_alpha(alpha).into()),
        text_color: Some(p.base),
        border_radius: Some(PILL_RADIUS.into()),
        ..style::Style::default()
    }
}

fn outline_impl(p: Palette, _s: status::Status) -> style::Style {
    style::Style {
        text_color: Some(p.tint_text),
        border_width: Some(1.0),
        border_color: Some(p.border),
        border_radius: Some(PILL_RADIUS.into()),
        ..style::Style::default()
    }
}

fn ghost_impl(p: Palette, _s: status::Status) -> style::Style {
    style::Style {
        text_color: Some(p.tint_text),
        border_radius: Some(PILL_RADIUS.into()),
        ..style::Style::default()
    }
}

impl From<Default> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Default) -> Self {
        style::StyleFn {
            inner: default_impl,
            tint: style::Tint::Primary,
        }
    }
}

impl From<Secondary> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Secondary) -> Self {
        style::StyleFn {
            inner: secondary_impl,
            tint: style::Tint::Secondary,
        }
    }
}

impl From<Destructive> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Destructive) -> Self {
        style::StyleFn {
            inner: destructive_impl,
            tint: style::Tint::Destructive,
        }
    }
}

impl From<Outline> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Outline) -> Self {
        style::StyleFn {
            inner: outline_impl,
            tint: style::Tint::Neutral,
        }
    }
}

impl From<Ghost> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Ghost) -> Self {
        style::StyleFn {
            inner: ghost_impl,
            tint: style::Tint::Neutral,
        }
    }
}

fn bridge<F>(sf: style::StyleFn<F>) -> iced_widget::container::StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
{
    Box::new(move |theme| {
        let palette = theme.tinted(sf.tint);
        let user = (sf.inner)(palette, status::Status::Active);
        let default = intrinsic_default(theme);
        let merged = default + user;
        super::container::project_container(merged)
    })
}

impl<'a> From<Default> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: Default) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge(sf)
    }
}

impl<'a> From<Secondary> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: Secondary) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge(sf)
    }
}

impl<'a> From<Destructive> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: Destructive) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge(sf)
    }
}

impl<'a> From<Outline> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: Outline) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge(sf)
    }
}

impl<'a> From<Ghost> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: Ghost) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge(sf)
    }
}

impl<'a> From<Spec> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |t| spec.resolve(t).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Background, Color};

    fn bg_color(style: &Style) -> Color {
        match style.background {
            Some(Background::Color(c)) => c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    /// `Default` resolves to `palette.primary.base` background.
    #[test]
    fn default_uses_primary_background() {
        let theme = Theme::dark();
        let resolved = default(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.primary.base;
        assert!(
            (bg.l - expected.l).abs() < 1e-4
                && (bg.c - expected.c).abs() < 1e-4
                && (bg.a - expected.a).abs() < 1e-4,
            "expected primary.base, got {bg:?}",
        );
    }

    /// `Outline` has no background — the variant must produce `None`.
    #[test]
    fn outline_has_no_background() {
        let theme = Theme::dark();
        let resolved = outline(&theme);
        assert!(
            resolved.background.is_none(),
            "Outline must not set a background, got {:?}",
            resolved.background,
        );
    }
}
