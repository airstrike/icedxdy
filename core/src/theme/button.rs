//! Button styling — nova style
//!
//! See plan/nova-style-guide.md for full spec.
//!
//! `Style` is OKLCH-native — variants and tweaks operate on our `Color`,
//! `Border`, `Shadow`, `Background` end-to-end. Conversion to iced's sRGB
//! types happens once at the catalog boundary (`From<Style> for
//! iced::Style`).

pub use iced_widget::button::Status;

use std::ops::Add;

use super::Theme;
use super::tweak::{Trigger, Tweak, TweakSet};
use crate::{Background, Border, Color, Shadow, radius};

/// OKLCH-native button style. Mirrors `iced_widget::button::Style`
/// field-for-field, but the color-bearing fields use our types so the
/// whole resolve pipeline stays in OKLCH.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The background of the button.
    pub background: Option<Background>,
    /// The text color of the button.
    pub text_color: Color,
    /// The border of the button.
    pub border: Border,
    /// The drop shadow of the button.
    pub shadow: Shadow,
    /// Whether the button should be snapped to the pixel grid.
    pub snap: bool,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            // Matches iced's `button::Style::default` which uses Color::BLACK
            // explicitly. (`Color::default()` is transparent, which would
            // produce invisible text — bad fallback.)
            text_color: Color::BLACK,
            border: Border::default(),
            shadow: Shadow::default(),
            // The `crisp` feature lives on the widget crate; `snap` is set
            // to match its default-off value here. iced's default uses
            // `renderer::CRISP` which we don't depend on from core.
            snap: false,
        }
    }
}

/// A styling closure that returns our OKLCH-native `Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl From<Style> for iced_widget::button::Style {
    fn from(s: Style) -> Self {
        iced_widget::button::Style {
            background: s.background.map(Into::into),
            text_color: s.text_color.into(),
            border: s.border.into(),
            shadow: s.shadow.into(),
            snap: s.snap,
        }
    }
}

impl From<iced_widget::button::Style> for Style {
    fn from(s: iced_widget::button::Style) -> Self {
        Style {
            background: s.background.map(Into::into),
            text_color: s.text_color.into(),
            border: s.border.into(),
            shadow: s.shadow.into(),
            snap: s.snap,
        }
    }
}

impl iced_widget::button::Catalog for Theme {
    type Class<'a> = iced_widget::button::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|t, s| default(t, s).into())
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> iced_widget::button::Style {
        class(self, status)
    }
}

fn is_dark(theme: &Theme) -> bool {
    theme.neutral.base.l < 0.5
}

/// Default/primary: bg-primary text-primary-foreground
/// Nova: NO hover fill change on buttons ([a]:hover only applies inside <a>)
pub fn default(theme: &Theme, status: Status) -> Style {
    let p = theme;

    let base = Style {
        background: Some(p.primary.base.into()),
        text_color: p.primary.foreground,
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    };

    match status {
        Status::Active | Status::Hovered | Status::Pressed => base,
        Status::Disabled => Style {
            background: Some(p.primary.base.alpha(0.5).into()),
            text_color: p.primary.foreground.alpha(0.5),
            ..base
        },
    }
}

/// Secondary: bg-secondary text-secondary-foreground hover:bg-secondary/80
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let p = theme;

    let base = Style {
        background: Some(p.secondary.base.into()),
        text_color: p.secondary.foreground,
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(p.secondary.base.alpha(0.8).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(p.secondary.base.alpha(0.7).into()),
            ..base
        },
        Status::Disabled => Style {
            background: Some(p.secondary.base.alpha(0.5).into()),
            text_color: p.secondary.foreground.alpha(0.5),
            ..base
        },
    }
}

/// Outline:
/// Light: border-border bg-background hover:bg-muted hover:text-foreground
/// Dark:  border-input  bg-input/30  hover:bg-input/50
/// Nova: no shadow
pub fn outline(theme: &Theme, status: Status) -> Style {
    let p = theme;
    let dark = is_dark(theme);

    // Dark: input is oklch(1 0 0 / 15%). Tailwind /30 and /50 MULTIPLY the alpha.
    let input_a = p.input.a;
    let (bg, border_color, hover_bg) = if dark {
        (
            p.input.alpha(input_a * 0.3),
            p.input,
            p.input.alpha(input_a * 0.5),
        )
    } else {
        (
            p.neutral.base,
            p.neutral.border,
            p.muted.base,
        )
    };

    let base = Style {
        background: Some(bg.into()),
        text_color: p.neutral.foreground,
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: border_color,
        },
        ..Style::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(hover_bg.into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(
                if dark {
                    p.input.alpha(input_a * 0.6)
                } else {
                    p.muted.base.alpha(0.9)
                }
                .into(),
            ),
            ..base
        },
        Status::Disabled => Style {
            background: Some(bg.alpha(0.5).into()),
            text_color: p.neutral.foreground.alpha(0.5),
            border: Border {
                color: border_color.alpha(0.5),
                ..base.border
            },
            ..base
        },
    }
}

/// Ghost:
/// hover:bg-muted hover:text-foreground
/// dark:hover:bg-muted/50
pub fn ghost(theme: &Theme, status: Status) -> Style {
    let p = theme;
    let dark = is_dark(theme);

    let hover_bg = if dark {
        p.muted.base.alpha(0.5)
    } else {
        p.muted.base
    };

    let base = Style {
        background: None,
        text_color: p.neutral.foreground,
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(hover_bg.into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(hover_bg.alpha(0.9).into()),
            ..base
        },
        Status::Disabled => Style {
            text_color: p.neutral.foreground.alpha(0.5),
            ..base
        },
    }
}

/// Destructive:
/// Light: bg-destructive/10 hover:bg-destructive/20 text-destructive
/// Dark:  bg-destructive/20 hover:bg-destructive/30 text-destructive
pub fn destructive(theme: &Theme, status: Status) -> Style {
    let p = theme;
    let dark = is_dark(theme);

    let (rest_alpha, hover_alpha) = if dark { (0.2, 0.3) } else { (0.1, 0.2) };

    let base = Style {
        background: Some(p.destructive.base.alpha(rest_alpha).into()),
        text_color: p.destructive.base,
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(p.destructive.base.alpha(hover_alpha).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(p.destructive.base.alpha(hover_alpha + 0.05).into()),
            ..base
        },
        Status::Disabled => Style {
            background: Some(p.destructive.base.alpha(rest_alpha * 0.5).into()),
            text_color: p.destructive.base.alpha(0.5),
            ..base
        },
    }
}

/// Sidebar: inactive sidebar item
/// No bg at rest. Hover/pressed match the registry default
/// `hover:bg-sidebar-accent` — full-strength sidebar accent. The
/// active item uses the main `palette.accent` (via [`sidebar_active`]),
/// which is visibly stronger than `sidebar.accent`, so the active
/// state stays clearly distinct from a hovered sibling without
/// alpha-attenuating hover.
pub fn sidebar(theme: &Theme, status: Status) -> Style {
    let sb = &theme.sidebar;

    let base = Style {
        background: None,
        text_color: sb.foreground,
        border: Border {
            radius: radius::SM.into(),
            ..Border::default()
        },
        ..Style::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered | Status::Pressed => Style {
            background: Some(sb.accent.base.into()),
            text_color: sb.accent.foreground,
            ..base
        },
        Status::Disabled => Style {
            text_color: sb.foreground.alpha(0.5),
            ..base
        },
    }
}

/// Sidebar active: currently selected sidebar item
/// Matches nova `SidebarMenuButton` `data-active:bg-sidebar-accent
/// data-active:text-sidebar-accent-foreground` — uses sidebar palette
/// (subtler than main accent, especially in dark mode).
pub fn sidebar_active(theme: &Theme, status: Status) -> Style {
    let p = theme;

    // Docs-site override (`.nova/apps/v4/components/docs-sidebar.tsx`):
    // `data-[active=true]:border-accent data-[active=true]:bg-accent`. The
    // accent is the main palette accent (NOT sidebar.accent) — visibly
    // stronger than the inactive hover surface so the current selection
    // stands out from the row a user is mousing over.
    let base = Style {
        background: Some(p.accent.base.into()),
        text_color: p.accent.foreground,
        border: Border {
            radius: radius::SM.into(),
            width: 1.0,
            color: p.accent.base,
        },
        ..Style::default()
    };

    match status {
        Status::Active | Status::Hovered | Status::Pressed => base,
        Status::Disabled => Style {
            background: Some(p.accent.base.alpha(0.5).into()),
            text_color: p.accent.foreground.alpha(0.5),
            ..base
        },
    }
}

/// Link: text-primary underline-offset-4 hover:underline
/// No bg, no border. Opacity fallback since iced lacks underline.
pub fn link(theme: &Theme, status: Status) -> Style {
    let p = theme;

    let base = Style {
        background: None,
        text_color: p.primary.base,
        border: Border::default(),
        ..Style::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            text_color: p.primary.base.alpha(0.8),
            ..base
        },
        Status::Pressed => Style {
            text_color: p.primary.base.alpha(0.7),
            ..base
        },
        Status::Disabled => Style {
            text_color: p.primary.base.alpha(0.5),
            ..base
        },
    }
}

// --- Variant unit structs ----------------------------------------------------
//
// Tag types matching shadcn variant names. Each impls `Add<T: Into<Tweak>>` to
// produce a `Spec` (tweak-augmented variant) and `From<Variant> for Spec` for
// implicit promotion. The lowercase fns above remain as the resolution backing.

/// Default/primary button variant (`bg-primary`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Secondary button variant (`bg-secondary`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Secondary;

/// Outline button variant (`border bg-background`, dark `bg-input/30`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outline;

/// Ghost button variant (no fill at rest).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ghost;

/// Destructive button variant (`bg-destructive/10..20`, `text-destructive`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Destructive;

/// Inactive sidebar item variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sidebar;

/// Active sidebar item variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SidebarActive;

/// Link-styled button variant (`text-primary`, no fill).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link;

// --- Spec --------------------------------------------------------------------

/// A button style: a base variant function plus a sequence of tweaks
/// applied at resolve time.
#[derive(Debug, Clone)]
pub struct Spec {
    variant: fn(&Theme, Status) -> Style,
    tweaks: TweakSet,
}

impl Spec {
    fn variant(f: fn(&Theme, Status) -> Style) -> Spec {
        Spec {
            variant: f,
            tweaks: TweakSet::default(),
        }
    }

    fn push(mut self, t: Tweak) -> Spec {
        self.tweaks.0.push(t);
        self
    }

    /// Resolve this spec to a concrete `Style` for the given theme and
    /// button status. The base variant is computed first, then each
    /// tweak is applied in order.
    pub fn resolve(&self, theme: &Theme, status: Status) -> Style {
        let mut style = (self.variant)(theme, status);
        for t in &self.tweaks.0 {
            apply(t, &mut style, theme, status);
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

impl From<Destructive> for Spec {
    fn from(_: Destructive) -> Spec {
        Spec::variant(destructive)
    }
}

impl From<Sidebar> for Spec {
    fn from(_: Sidebar) -> Spec {
        Spec::variant(sidebar)
    }
}

impl From<SidebarActive> for Spec {
    fn from(_: SidebarActive) -> Spec {
        Spec::variant(sidebar_active)
    }
}

impl From<Link> for Spec {
    fn from(_: Link) -> Spec {
        Spec::variant(link)
    }
}

impl<T: Into<Tweak>> Add<T> for Default {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Secondary {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Outline {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Ghost {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Destructive {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Sidebar {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for SidebarActive {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Link {
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

fn apply(tweak: &Tweak, style: &mut Style, theme: &Theme, status: Status) {
    match tweak {
        Tweak::Bg(t) => {
            style.background = Some(t.resolve(theme).into());
        }
        Tweak::Fg(t) => {
            style.text_color = t.resolve(theme);
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
            // OKLCH alpha-composite the variant's bg over palette.background
            // for ALL statuses. `Color::blend(dst, t)` is OKLCH lerp; at
            // t = 1 - src.a it's the alpha-compositing formula. Then force
            // alpha=1.0 since the result is meant to be opaque.
            //
            // Applies to every status so hover/pressed/disabled also become
            // opaque while keeping their per-state luminance progression
            // (Active 0.045 → Hovered 0.075 → Pressed 0.09 → Disabled 0.022
            // alpha differences become visible luminance lifts).
            if let Some(Background::Color(src)) = style.background {
                let dst = theme.neutral.base;
                let composed = src.blend(dst, 1.0 - src.a).alpha(1.0);
                style.background = Some(Background::Color(composed));
            }
            // Background::Gradient: pass through unchanged.
        }
        Tweak::Alpha(factor) => {
            if let Some(bg) = style.background {
                style.background = Some(bg.scale_alpha(*factor));
            }
        }
        Tweak::State(trigger, inner) => {
            if triggered(*trigger, status) {
                for inner_tweak in &inner.0 {
                    apply(inner_tweak, style, theme, status);
                }
            }
        }
        // CSS-style cascade: tweaks that don't apply to button styling are
        // silently dropped. Ring rendering is deferred to Phase 5.
        Tweak::Ring { .. }
        | Tweak::Size(_)
        | Tweak::Weight(_)
        | Tweak::Family(_)
        | Tweak::LineHeight(_)
        | Tweak::Italic
        | Tweak::Align(_) => {}
    }
}

fn triggered(trigger: Trigger, status: Status) -> bool {
    match (trigger, status) {
        (Trigger::Hover, Status::Hovered | Status::Pressed) => true,
        (Trigger::Active, Status::Pressed) => true,
        (Trigger::Disabled, Status::Disabled) => true,
        // Button has no focus state in iced; needs widget-level focus tracking.
        (Trigger::Focus, _) => false,
        _ => false,
    }
}

// --- StyleFn conversions -----------------------------------------------------
//
// Allow variants and Specs to be passed directly to `Button::style()` (which
// takes `impl Into<StyleFn<'a, Theme>>`). For variants, this just boxes the
// existing fn pointer; for Spec, it captures the spec and resolves at call.

impl<'a> From<Default> for StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(default)
    }
}

impl<'a> From<Secondary> for StyleFn<'a, Theme> {
    fn from(_: Secondary) -> Self {
        Box::new(secondary)
    }
}

impl<'a> From<Outline> for StyleFn<'a, Theme> {
    fn from(_: Outline) -> Self {
        Box::new(outline)
    }
}

impl<'a> From<Ghost> for StyleFn<'a, Theme> {
    fn from(_: Ghost) -> Self {
        Box::new(ghost)
    }
}

impl<'a> From<Destructive> for StyleFn<'a, Theme> {
    fn from(_: Destructive) -> Self {
        Box::new(destructive)
    }
}

impl<'a> From<Sidebar> for StyleFn<'a, Theme> {
    fn from(_: Sidebar) -> Self {
        Box::new(sidebar)
    }
}

impl<'a> From<SidebarActive> for StyleFn<'a, Theme> {
    fn from(_: SidebarActive) -> Self {
        Box::new(sidebar_active)
    }
}

impl<'a> From<Link> for StyleFn<'a, Theme> {
    fn from(_: Link) -> Self {
        Box::new(link)
    }
}

impl<'a> From<Spec> for StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |theme, status| spec.resolve(theme, status))
    }
}

// --- Bridge to iced::button::StyleFn -----------------------------------------
//
// These let variants/Spec be passed straight to widgets whose `.style()`
// accepts `impl Into<iced_widget::button::StyleFn<...>>`. Each closure runs
// our OKLCH-native pipeline and converts to iced's sRGB Style at the end.

impl<'a> From<Default> for iced_widget::button::StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(|t, s| default(t, s).into())
    }
}

impl<'a> From<Secondary> for iced_widget::button::StyleFn<'a, Theme> {
    fn from(_: Secondary) -> Self {
        Box::new(|t, s| secondary(t, s).into())
    }
}

impl<'a> From<Outline> for iced_widget::button::StyleFn<'a, Theme> {
    fn from(_: Outline) -> Self {
        Box::new(|t, s| outline(t, s).into())
    }
}

impl<'a> From<Ghost> for iced_widget::button::StyleFn<'a, Theme> {
    fn from(_: Ghost) -> Self {
        Box::new(|t, s| ghost(t, s).into())
    }
}

impl<'a> From<Destructive> for iced_widget::button::StyleFn<'a, Theme> {
    fn from(_: Destructive) -> Self {
        Box::new(|t, s| destructive(t, s).into())
    }
}

impl<'a> From<Sidebar> for iced_widget::button::StyleFn<'a, Theme> {
    fn from(_: Sidebar) -> Self {
        Box::new(|t, s| sidebar(t, s).into())
    }
}

impl<'a> From<SidebarActive> for iced_widget::button::StyleFn<'a, Theme> {
    fn from(_: SidebarActive) -> Self {
        Box::new(|t, s| sidebar_active(t, s).into())
    }
}

impl<'a> From<Link> for iced_widget::button::StyleFn<'a, Theme> {
    fn from(_: Link) -> Self {
        Box::new(|t, s| link(t, s).into())
    }
}

impl<'a> From<Spec> for iced_widget::button::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |t, s| spec.resolve(t, s).into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tweak::{Bg, Opaque, Token};
    use super::*;

    fn bg_color(style: &Style) -> Color {
        match style.background {
            Some(Background::Color(c)) => c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    /// Outline + Opaque on dark, Active: bg must be opaque, near-but-slightly-
    /// lifted-from palette.background (the variant's input/0.045 white
    /// composited over background ≈ 0.184 luminance vs background's 0.145).
    /// This is the regression test for the early "alpha-strip → white-on-white"
    /// bug.
    #[test]
    fn outline_plus_opaque_dark_active_opaque_and_lifted() {
        let theme = Theme::dark();
        let spec: Spec = (Outline + Opaque).into();
        let resolved = spec.resolve(&theme, Status::Active);
        let bg = bg_color(&resolved);

        // Always opaque
        assert!((bg.a - 1.0).abs() < 1e-4, "expected opaque, got a={}", bg.a);

        // Should be a small lift above palette.background luminance, NOT
        // near-white. palette.background.l = 0.145; expected ≈ 0.184.
        let bg_l = theme.neutral.base.l;
        assert!(
            bg.l > bg_l && bg.l < bg_l + 0.10,
            "expected small lift above bg.l={bg_l}, got l={}",
            bg.l
        );
        assert!(
            bg.l < 0.5,
            "dark-mode bg must not be near-white: l={}",
            bg.l
        );
    }

    /// Outline + Opaque on light: bg ≈ palette.background (which is white).
    /// Compositing white over white is white — no visible change.
    #[test]
    fn outline_plus_opaque_light_active_remains_near_white() {
        let theme = Theme::light();
        let spec: Spec = (Outline + Opaque).into();
        let resolved = spec.resolve(&theme, Status::Active);
        let bg = bg_color(&resolved);

        assert!((bg.a - 1.0).abs() < 1e-4);
        // Light bg luminance is 1.0; outline.bg on light is also background
        // (white), so compositing white-over-white stays white.
        assert!(bg.l > 0.95, "expected near-white, got l={}", bg.l);
    }

    /// Ghost has no background at rest; Opaque is a no-op.
    #[test]
    fn ghost_plus_opaque_remains_backgroundless() {
        let theme = Theme::dark();
        let spec: Spec = (Ghost + Opaque).into();
        let resolved = spec.resolve(&theme, Status::Active);
        assert!(
            resolved.background.is_none(),
            "ghost has no bg; opaque must not synthesize one",
        );
    }

    /// Bg(Token) atom should override the variant's bg with the token color.
    /// Sanity check that the atom plumbing reaches apply().
    #[test]
    fn bg_atom_overrides_variant_background() {
        let theme = Theme::dark();
        let spec: Spec = (Outline + Bg(Token::Primary)).into();
        let resolved = spec.resolve(&theme, Status::Active);

        let expected = theme.primary.base;
        let actual = bg_color(&resolved);
        assert!(
            (actual.l - expected.l).abs() < 1e-4
                && (actual.c - expected.c).abs() < 1e-4
                && (actual.a - expected.a).abs() < 1e-4,
            "expected bg = primary ({expected:?}), got {actual:?}",
        );
    }

    /// Opaque applies to all states, AND the variant's per-state alpha
    /// progression (Outline dark: 0.045 / 0.075 / 0.090) becomes a visible
    /// luminance progression (Active < Hovered < Pressed). Every state is
    /// opaque (alpha=1.0).
    #[test]
    fn outline_plus_opaque_preserves_state_lift() {
        let theme = Theme::dark();
        let spec: Spec = (Outline + Opaque).into();

        let active = bg_color(&spec.resolve(&theme, Status::Active));
        let hovered = bg_color(&spec.resolve(&theme, Status::Hovered));
        let pressed = bg_color(&spec.resolve(&theme, Status::Pressed));

        for s in [active, hovered, pressed] {
            assert!((s.a - 1.0).abs() < 1e-4, "all states must be opaque");
        }

        assert!(
            active.l < hovered.l,
            "Active.l ({}) must be < Hovered.l ({})",
            active.l,
            hovered.l
        );
        assert!(
            hovered.l < pressed.l,
            "Hovered.l ({}) must be < Pressed.l ({})",
            hovered.l,
            pressed.l
        );
    }

    /// `sidebar` (inactive): hover/pressed bg uses `palette.sidebar.accent.base`
    /// (the subtler sidebar palette, NOT the main palette accent). Active
    /// has no bg (transparent on rest).
    #[test]
    fn sidebar_hover_uses_sidebar_palette_accent() {
        let theme = Theme::dark();
        let resting = sidebar(&theme, Status::Active);
        assert!(
            resting.background.is_none(),
            "sidebar at rest must be backgroundless, got {:?}",
            resting.background,
        );

        let hovered = sidebar(&theme, Status::Hovered);
        let bg = bg_color(&hovered);
        let expected = theme.sidebar.accent.base;
        assert!(
            (bg.l - expected.l).abs() < 1e-4 && (bg.c - expected.c).abs() < 1e-4,
            "sidebar hover must use sidebar.accent.base ({expected:?}), got {bg:?}",
        );
    }

    /// `sidebar_active` (selected): bg uses the main `palette.accent.base`
    /// (NOT `sidebar.accent`) per the docs-sidebar override — active items
    /// are visibly stronger than mere hover surfaces on inactive siblings.
    #[test]
    fn sidebar_active_uses_main_palette_accent() {
        let theme = Theme::dark();
        let active = sidebar_active(&theme, Status::Active);
        let bg = bg_color(&active);
        let main_accent = theme.accent.base;
        let sb_accent = theme.sidebar.accent.base;

        assert!(
            (bg.l - main_accent.l).abs() < 1e-4 && (bg.c - main_accent.c).abs() < 1e-4,
            "sidebar_active must use main palette.accent.base ({main_accent:?}), got {bg:?}",
        );
        assert!(
            (bg.l - sb_accent.l).abs() > 1e-3,
            "sidebar_active must NOT use sidebar.accent (would defeat the docs-sidebar override)",
        );
    }

    /// Default + Opaque is a no-op — the primary variant is already opaque
    /// (alpha=1.0), so compositing src.a=1 over dst gives src unchanged.
    #[test]
    fn default_plus_opaque_no_change() {
        let theme = Theme::dark();
        let bare = bg_color(&default(&theme, Status::Active));
        let with_opaque = bg_color(&(Spec::from(Default) + Opaque).resolve(&theme, Status::Active));
        assert!(
            (bare.l - with_opaque.l).abs() < 1e-4 && (bare.c - with_opaque.c).abs() < 1e-4,
            "Default already opaque; Opaque must be a no-op. bare={bare:?}, with={with_opaque:?}",
        );
    }
}
