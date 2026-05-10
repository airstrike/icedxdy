//! Sidebar container styling
//!
//! `Style` is OKLCH-native — sidebar reuses container's `Style` field-for-field.
//! Variant fns + Spec/atom apply tweaks before conversion.
//!
//! Other sidebar styling (menu buttons, separators, group labels) is handled
//! inside each respective widget module, matching docs-sidebar rendering rather
//! than base shadcn `SidebarMenuButton`.

use std::ops::Add;

use iced_core::Vector;

use super::Theme;
use super::container::Style;
use super::tweak::{Tweak, TweakSet};
use crate::{Background, Border, Color, Shadow, radius};

// --- Variant fns -------------------------------------------------------------

/// Sidebar background — flush variant, solid `palette.sidebar.background`.
pub fn default(theme: &Theme) -> Style {
    Style {
        background: Some(theme.sidebar.background.into()),
        ..Style::default()
    }
}

/// Floating sidebar — sidebar bg, 1px border, drop shadow, rounded-lg.
pub fn floating(theme: &Theme) -> Style {
    let sb = &theme.sidebar;
    Style {
        background: Some(sb.background.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: sb.border,
        },
        shadow: Shadow {
            color: Color::BLACK.alpha(0.1),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Style::default()
    }
}

/// Inset sidebar — sidebar bg, NO border, NO shadow.
///
/// Per shadcn nova `sidebar-inner` (sidebar.tsx:244): the floating variant
/// adds `rounded-lg shadow-sm ring-1`, but the inset variant does not. The
/// content area (`SidebarInset`) is what gets the rounded/border/shadow
/// treatment when paired with this variant — see
/// `super::container::sidebar_inset`.
pub fn inset(theme: &Theme) -> Style {
    Style {
        background: Some(theme.sidebar.background.into()),
        ..Style::default()
    }
}

// --- Variant unit structs ----------------------------------------------------

/// Default (flush) sidebar variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Floating sidebar variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Floating;

/// Inset sidebar variant — no border on the sidebar itself; the content
/// area (`SidebarInset`) carries the inset chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Inset;

// --- Spec --------------------------------------------------------------------

/// A sidebar style: a base variant function plus a sequence of tweaks
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

impl From<Floating> for Spec {
    fn from(_: Floating) -> Spec {
        Spec::variant(floating)
    }
}

impl From<Inset> for Spec {
    fn from(_: Inset) -> Spec {
        Spec::variant(inset)
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

variant_add!(Default, Floating, Inset);

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        self.push(rhs.into())
    }
}

// --- Resolver ----------------------------------------------------------------
//
// Sidebar has no status, so `Tweak::State(...)` silently drops. Identical body
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

// --- Bridge to iced::container::StyleFn --------------------------------------

impl<'a> From<Default> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(|t| default(t).into())
    }
}

impl<'a> From<Floating> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: Floating) -> Self {
        Box::new(|t| floating(t).into())
    }
}

impl<'a> From<Inset> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: Inset) -> Self {
        Box::new(|t| inset(t).into())
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

    fn bg_color(style: &Style) -> Color {
        match style.background {
            Some(Background::Color(c)) => c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    /// `Default` resolves to `palette.sidebar.background`.
    #[test]
    fn default_uses_sidebar_background() {
        let theme = Theme::dark();
        let resolved = default(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.sidebar.background;
        assert!(
            (bg.l - expected.l).abs() < 1e-4
                && (bg.c - expected.c).abs() < 1e-4
                && (bg.a - expected.a).abs() < 1e-4,
            "expected sidebar.background, got {bg:?}",
        );
    }

    /// `Floating` resolves to `palette.sidebar.background` with a shadow + LG radius.
    #[test]
    fn floating_uses_sidebar_background_with_shadow() {
        let theme = Theme::dark();
        let resolved = floating(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.sidebar.background;
        assert!(
            (bg.l - expected.l).abs() < 1e-4
                && (bg.c - expected.c).abs() < 1e-4
                && (bg.a - expected.a).abs() < 1e-4,
            "expected sidebar.background, got {bg:?}",
        );
        assert!(
            resolved.shadow.blur_radius > 0.0,
            "Floating must carry a non-zero shadow",
        );
        assert_eq!(
            resolved.border.radius,
            radius::LG.into(),
            "Floating must use rounded-lg corners",
        );
    }

    /// `Inset` carries `palette.sidebar.background` only — no border, no
    /// shadow. The sibling content area (`SidebarInset`) is what gets the
    /// rounded/border/shadow treatment in this variant.
    #[test]
    fn inset_has_no_border_or_shadow() {
        let theme = Theme::dark();
        let resolved = inset(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.sidebar.background;
        assert!(
            (bg.l - expected.l).abs() < 1e-4
                && (bg.c - expected.c).abs() < 1e-4
                && (bg.a - expected.a).abs() < 1e-4,
            "expected sidebar.background, got {bg:?}",
        );
        assert_eq!(
            resolved.border.width, 0.0,
            "Inset sidebar must have no border (width=0)",
        );
        assert_eq!(
            resolved.shadow.blur_radius, 0.0,
            "Inset sidebar must have no shadow",
        );
    }
}
