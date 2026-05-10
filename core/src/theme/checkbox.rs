//! Checkbox styling — nova style
//!
//! `Style` is OKLCH-native — variants and tweaks operate on our `Color`,
//! `Border`, `Background` end-to-end. Conversion to iced's sRGB types happens
//! once at the catalog boundary (`From<Style> for iced::checkbox::Style`).
//!
//! Nova classlist (verbatim from `.nova/apps/v4/styles/radix-nova/ui/checkbox.tsx`):
//!
//! ```text
//! peer relative flex size-4 shrink-0 items-center justify-center
//! rounded-[4px] border border-input transition-colors outline-none
//! group-has-disabled/field:opacity-50
//! after:absolute after:-inset-x-3 after:-inset-y-2
//! focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50
//! disabled:cursor-not-allowed disabled:opacity-50
//! aria-invalid:border-destructive aria-invalid:ring-3
//! aria-invalid:ring-destructive/20 aria-invalid:aria-checked:border-primary
//! dark:bg-input/30 dark:aria-invalid:border-destructive/50
//! dark:aria-invalid:ring-destructive/40
//! data-checked:border-primary data-checked:bg-primary
//! data-checked:text-primary-foreground dark:data-checked:bg-primary
//! ```

pub use iced_widget::checkbox::Status;

use std::ops::Add;

use super::Theme;
use super::tweak::{Trigger, Tweak, TweakSet};
use crate::palette::Palette;
use crate::style;
use crate::style::status;
use crate::{Background, Border, Color, radius};

/// OKLCH-native checkbox style. Mirrors `iced_widget::checkbox::Style`
/// field-for-field, plus a `ring` field with no iced equivalent — rendered
/// by our vendored `widget::checkbox` as a halo behind the box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The background of the checkbox box.
    pub background: Background,
    /// The color of the check / indeterminate icon.
    pub icon_color: Color,
    /// The border of the checkbox box.
    pub border: Border,
    /// Optional label text color (None inherits the default).
    pub text_color: Option<Color>,
    /// Optional focus ring (Tailwind `ring-*`). `None` = no halo rendered.
    pub ring: Option<Ring>,
}

/// Resolved focus-ring fragment: a solid colored halo painted at `width`
/// pixels around the checkbox bounds, behind the main fill. Equivalent to
/// CSS `box-shadow: 0 0 0 {width}px {color}`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    pub color: Color,
    pub width: f32,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            background: Background::Color(Color::TRANSPARENT),
            // `Color::default()` is fully transparent — useless as a fallback
            // for an icon. Use BLACK so the check renders *something* if a
            // caller skips the catalog default.
            icon_color: Color::BLACK,
            border: Border::default(),
            text_color: None,
            ring: None,
        }
    }
}

/// A styling closure that returns our OKLCH-native `Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

/// Convert from iced's stock `checkbox::Style` into ours. Lossless in this
/// direction (iced has no `ring` field, so we set `ring: None`).
impl From<iced_widget::checkbox::Style> for Style {
    fn from(s: iced_widget::checkbox::Style) -> Self {
        Style {
            background: s.background.into(),
            icon_color: s.icon_color.into(),
            border: s.border.into(),
            text_color: s.text_color.map(Into::into),
            ring: None,
        }
    }
}

/// The inverse: converting our `Style` to iced's silently drops `ring`. We
/// keep this impl because stock iced widgets that compose a checkbox (e.g.
/// markdown task-lists in `widget/src/markdown/widget.rs`) require `Theme:
/// iced_widget::checkbox::Catalog`, and that Catalog impl needs to return
/// iced's `Style`. New code in our crate should prefer the vendored
/// `widget::checkbox` widget instead — it understands ring.
impl From<Style> for iced_widget::checkbox::Style {
    fn from(s: Style) -> Self {
        iced_widget::checkbox::Style {
            background: s.background.into(),
            icon_color: s.icon_color.into(),
            border: s.border.into(),
            text_color: s.text_color.map(Into::into),
        }
    }
}

/// Bridge so stock iced widgets (markdown task-lists etc.) that compose a
/// checkbox continue to work with our `Theme`. Routes through our OKLCH
/// resolver and converts Style at the boundary. Ring is dropped at the
/// iced::Style boundary — those compositors can't render it anyway.
impl iced_widget::checkbox::Catalog for Theme {
    type Class<'a> = iced_widget::checkbox::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|t, s| default(t, s).into())
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> iced_widget::checkbox::Style {
        class(self, status)
    }
}

fn is_dark(theme: &Theme) -> bool {
    theme.neutral.base.l < 0.5
}

fn is_checked(status: Status) -> bool {
    match status {
        Status::Active { is_checked }
        | Status::Hovered { is_checked }
        | Status::Disabled { is_checked } => is_checked,
    }
}

// --- Variant fns -------------------------------------------------------------

/// Default checkbox variant — nova `size-4` box with `rounded-[4px]` corners.
///
/// Light unchecked bg = transparent (the page background shows through); dark
/// unchecked bg = `palette.input` at α 0.3 (`dark:bg-input/30`). Checked bg =
/// `palette.primary` (`data-checked:bg-primary`); icon color = primary
/// foreground; checked border = primary. Border at rest = `palette.input`
/// (`border border-input`).
pub fn default(theme: &Theme, status: Status) -> Style {
    let p = theme;
    let dark = is_dark(theme);
    let checked = is_checked(status);

    // Unchecked bg — light = transparent, dark = palette.input @ 0.3.
    let unchecked_bg = if dark {
        p.input.alpha(0.3)
    } else {
        Color::TRANSPARENT
    };

    let active = if checked {
        Style {
            background: p.primary.base.into(),
            icon_color: p.primary.foreground,
            border: Border {
                // shadcn nova: `rounded-[4px]` — a literal pixel value, NOT a
                // Tailwind preset. We expose this as `radius::XS` rather than
                // a magic literal so future 4px-radius widgets pick it up
                // consistently.
                radius: radius::XS.into(),
                width: 1.0,
                color: p.primary.base,
            },
            text_color: None,
            ring: None,
        }
    } else {
        Style {
            background: unchecked_bg.into(),
            icon_color: p.primary.foreground,
            border: Border {
                radius: radius::XS.into(),
                width: 1.0,
                color: p.input,
            },
            text_color: None,
            ring: None,
        }
    };

    match status {
        // Nova has no `:hover` rule — Hovered must equal Active.
        Status::Active { .. } | Status::Hovered { .. } => active,
        Status::Disabled { .. } => Style {
            // `disabled:opacity-50` — multiply every channel by 0.5. Use
            // `scale_alpha` so colors keep their *own* base alpha (matters
            // for dark `palette.input` at α 0.15).
            background: active.background.scale_alpha(0.5),
            icon_color: active.icon_color.scale_alpha(0.5),
            border: Border {
                color: active.border.color.scale_alpha(0.5),
                ..active.border
            },
            text_color: Some(p.muted.foreground),
            ring: None,
        },
    }
}

/// Invalid checkbox variant — nova `aria-invalid:*` box.
///
/// Border swaps to `palette.destructive` (`aria-invalid:border-destructive`);
/// a 3px ring is rendered at `palette.destructive` @ 20% alpha
/// (`aria-invalid:ring-3 ring-destructive/20`) — in dark mode the ring drops
/// to 40% alpha (`dark:aria-invalid:ring-destructive/40`). Special case: when
/// invalid AND checked, the border returns to `palette.primary`
/// (`aria-invalid:aria-checked:border-primary`). In dark mode the
/// destructive border drops to alpha 0.5 (`dark:aria-invalid:border-destructive/50`).
pub fn invalid(theme: &Theme, status: Status) -> Style {
    let p = theme;
    let dark = is_dark(theme);
    let checked = is_checked(status);
    let base = default(theme, status);

    // Invalid border color: destructive (or primary if also checked).
    let invalid_border = if checked {
        p.primary.base
    } else if dark {
        p.destructive.base.alpha(0.5)
    } else {
        p.destructive.base
    };

    // shadcn `aria-invalid:ring-3 ring-destructive/20` (`/40` in dark).
    // Always rendered for the invalid variant — shadcn's `aria-invalid:`
    // prefix unconditionally enables the ring, no focus required (different
    // from the `focus-visible:ring-3` ring which gates on focus).
    let invalid_ring = Some(Ring {
        color: p.destructive.base.alpha(if dark { 0.40 } else { 0.20 }),
        width: 3.0,
    });

    match status {
        Status::Active { .. } | Status::Hovered { .. } => Style {
            border: Border {
                color: invalid_border,
                ..base.border
            },
            ring: invalid_ring,
            ..base
        },
        Status::Disabled { .. } => Style {
            border: Border {
                color: invalid_border.scale_alpha(0.5),
                ..base.border
            },
            // Disabled invalid: dim the ring along with everything else
            // (mirrors `disabled:opacity-50`).
            ring: invalid_ring.map(|r| Ring {
                color: r.color.scale_alpha(0.5),
                ..r
            }),
            ..base
        },
    }
}

/// Disabled checkbox variant — explicit disabled styling regardless of the
/// runtime `Status`. Useful as a tag-style variant where the call site
/// declares its disabled intent at construction time. Renders identically to
/// [`default`] under `Status::Disabled`.
pub fn disabled(theme: &Theme, status: Status) -> Style {
    let forced = match status {
        Status::Active { is_checked }
        | Status::Hovered { is_checked }
        | Status::Disabled { is_checked } => Status::Disabled { is_checked },
    };
    default(theme, forced)
}

// --- Variant unit structs ----------------------------------------------------
//
// Tag types matching the variant fns. Each impls `Add<T: Into<Tweak>>` to
// produce a `Spec`, and `From<Variant> for Spec` for implicit promotion.

/// Default checkbox variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Invalid checkbox variant — see [`invalid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invalid;

/// Disabled checkbox variant — see [`disabled`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Disabled;

// --- Spec --------------------------------------------------------------------

/// A checkbox style: a base variant function plus a sequence of tweaks
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
    /// checkbox status. The base variant is computed first, then each tweak
    /// is applied in order.
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

impl From<Invalid> for Spec {
    fn from(_: Invalid) -> Spec {
        Spec::variant(invalid)
    }
}

impl From<Disabled> for Spec {
    fn from(_: Disabled) -> Spec {
        Spec::variant(disabled)
    }
}

impl<T: Into<Tweak>> Add<T> for Default {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Invalid {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Disabled {
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
            style.background = t.resolve(theme).into();
        }
        Tweak::Fg(t) => {
            // For a checkbox the semantic "foreground" is the icon color.
            style.icon_color = t.resolve(theme);
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
            // OKLCH alpha-composite the variant's bg over palette.background
            // and force alpha=1.0. Same shape as button/container.
            if let Background::Color(src) = style.background {
                let dst = theme.neutral.base;
                let composed = src.blend(dst, 1.0 - src.a).alpha(1.0);
                style.background = Background::Color(composed);
            }
        }
        Tweak::Alpha(factor) => {
            style.background = style.background.scale_alpha(*factor);
        }
        Tweak::State(trigger, inner) => {
            if triggered(*trigger, status) {
                for inner_tweak in &inner.0 {
                    apply(inner_tweak, style, theme, status);
                }
            }
        }
        Tweak::Ring {
            color,
            width,
            offset: _,
        } => {
            style.ring = Some(Ring {
                color: color.resolve(theme),
                width: *width,
            });
        }
        // CSS-style cascade: tweaks that don't apply to checkbox styling are
        // silently dropped.
        Tweak::Shadow(_)
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
        (Trigger::Hover, Status::Hovered { .. }) => true,
        (Trigger::Disabled, Status::Disabled { .. }) => true,
        // Hover only fires on Hovered; Disabled only on Disabled. The other
        // (trigger, status) pairs are silently false — exhaustively listed
        // here so a future Status variant doesn't accidentally inherit
        // wildcard semantics.
        (Trigger::Hover, Status::Active { .. } | Status::Disabled { .. })
        | (Trigger::Disabled, Status::Active { .. } | Status::Hovered { .. }) => false,
        // Checkbox has no Pressed; Active is the rest state. Focus tracking
        // happens in the vendored widget — Spec sees Status without focus,
        // so `Trigger::Focus` and `Trigger::Active` are silently dropped.
        (Trigger::Focus | Trigger::Active, _) => false,
    }
}

// --- StyleFn conversions -----------------------------------------------------
//
// Allow variants and Specs to be passed directly to wrapper `.style()` builders
// that take `impl Into<StyleFn<'a, Theme>>`.

impl<'a> From<Default> for StyleFn<'a, Theme> {
    fn from(t: Default) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, default, std::convert::identity)
    }
}

impl<'a> From<Invalid> for StyleFn<'a, Theme> {
    fn from(t: Invalid) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, invalid, std::convert::identity)
    }
}

impl<'a> From<Disabled> for StyleFn<'a, Theme> {
    fn from(t: Disabled) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, disabled, std::convert::identity)
    }
}

impl<'a> From<Spec> for StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |theme, status| spec.resolve(theme, status))
    }
}

// Checkbox carries widget-specific slots (icon_color) the universal Style
// can't represent — per-tag intrinsics produce the full widget Style and
// the user overlay only touches bg / border / text_color / border_radius.

fn empty_impl(_p: Palette, _s: status::Status) -> style::Style {
    style::Style::default()
}

impl From<Default> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Default) -> Self {
        style::StyleFn {
            inner: empty_impl,
            tint: style::Tint::Neutral,
        }
    }
}

impl From<Invalid> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Invalid) -> Self {
        style::StyleFn {
            inner: empty_impl,
            tint: style::Tint::Destructive,
        }
    }
}

impl From<Disabled> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Disabled) -> Self {
        style::StyleFn {
            inner: empty_impl,
            tint: style::Tint::Neutral,
        }
    }
}

/// Project a universal Style overlay onto an existing checkbox `Style`.
/// Universal-Style fields override the widget Style; widget-specific
/// fields (icon_color) come from the intrinsic and are preserved.
fn overlay_universal(univ: style::Style, base: Style) -> Style {
    Style {
        background: univ.fill.map(Into::into).unwrap_or(base.background),
        icon_color: base.icon_color,
        border: Border {
            width: univ.border_width.unwrap_or(base.border.width),
            color: univ.border_color.unwrap_or(base.border.color),
            radius: univ.border_radius.unwrap_or(base.border.radius),
        },
        text_color: univ.text_color.or(base.text_color),
        ring: base.ring,
    }
}

fn bridge_with<F, D, Out: 'static>(
    sf: style::StyleFn<F>,
    defaults: D,
    convert: fn(Style) -> Out,
) -> Box<dyn Fn(&Theme, Status) -> Out>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
    D: Fn(&Theme, Status) -> Style + 'static,
{
    Box::new(move |theme, iced_status| {
        let s = status::from_checkbox(iced_status);
        let palette = theme.tinted(sf.tint);
        let user = (sf.inner)(palette, s);
        let base = defaults(theme, iced_status);
        convert(overlay_universal(user, base))
    })
}

impl<F> From<style::StyleFn<F>> for iced_widget::checkbox::StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
{
    fn from(sf: style::StyleFn<F>) -> Self {
        bridge_with(sf, default, Into::<iced_widget::checkbox::Style>::into)
    }
}

// --- Iced-side StyleFn conversions -------------------------------------------
//
// Stock iced widgets that compose a checkbox (markdown task-lists, etc.) call
// `Theme::style` which returns iced's `checkbox::Style`. Per-tag intrinsics
// supply the variant-specific defaults; the universal overlay is empty in
// these pass-throughs since each tag's `From for style::StyleFn` returns an
// empty inner.

impl<'a> From<Default> for iced_widget::checkbox::StyleFn<'a, Theme> {
    fn from(t: Default) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, default, Into::<iced_widget::checkbox::Style>::into)
    }
}

impl<'a> From<Invalid> for iced_widget::checkbox::StyleFn<'a, Theme> {
    fn from(t: Invalid) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, invalid, Into::<iced_widget::checkbox::Style>::into)
    }
}

impl<'a> From<Disabled> for iced_widget::checkbox::StyleFn<'a, Theme> {
    fn from(t: Disabled) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, disabled, Into::<iced_widget::checkbox::Style>::into)
    }
}

// --- Vendored-widget StyleFn bridges -----------------------------------------
//
// `theme::checkbox::StyleFn<'a, Theme>` is the type our widget builder
// `.style()` accepts. Bridge from `style::StyleFn<F>` and from each tag.

impl<F> From<style::StyleFn<F>> for StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
{
    fn from(sf: style::StyleFn<F>) -> Self {
        bridge_with(sf, default, std::convert::identity)
    }
}

impl<'a> From<Spec> for iced_widget::checkbox::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |theme, status| spec.resolve(theme, status).into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tweak::{self, Bg, Token};
    use super::*;

    fn bg_color(style: &Style) -> Color {
        match style.background {
            Background::Color(c) => c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    fn close(a: Color, b: Color) -> bool {
        (a.l - b.l).abs() < 1e-4 && (a.c - b.c).abs() < 1e-4 && (a.a - b.a).abs() < 1e-4
    }

    /// Checked default = `data-checked:bg-primary` — primary surface fills the
    /// box, primary foreground draws the check, border matches the fill.
    #[test]
    fn checked_default_uses_primary_bg() {
        let theme = Theme::light();
        let resolved = default(&theme, Status::Active { is_checked: true });
        let bg = bg_color(&resolved);
        let p = theme;
        assert!(
            close(bg, p.primary.base),
            "expected bg = primary ({:?}), got {bg:?}",
            p.primary.base,
        );
        assert!(
            close(resolved.icon_color, p.primary.foreground),
            "expected icon_color = primary.foreground",
        );
        assert!(
            close(resolved.border.color, p.primary.base),
            "expected border.color = primary",
        );
    }

    /// Dark unchecked bg = `palette.input` with alpha 0.3 (`dark:bg-input/30`).
    /// Tailwind's `/N` modifier overrides the source alpha (does NOT multiply).
    #[test]
    fn dark_unchecked_bg_uses_input_alpha_0_3() {
        let theme = Theme::dark();
        let resolved = default(&theme, Status::Active { is_checked: false });
        let bg = bg_color(&resolved);
        let expected = theme.input.alpha(0.3);
        assert!(
            close(bg, expected),
            "expected bg ≈ {expected:?}, got {bg:?}",
        );
    }

    /// Nova has no `:hover` rule — Hovered must be field-identical to Active.
    #[test]
    fn hover_equals_active() {
        let theme = Theme::dark();
        for is_checked in [false, true] {
            let active = default(&theme, Status::Active { is_checked });
            let hovered = default(&theme, Status::Hovered { is_checked });
            assert_eq!(active.background, hovered.background);
            assert_eq!(active.border, hovered.border);
            assert_eq!(active.icon_color, hovered.icon_color);
        }
    }

    /// Invalid + unchecked = destructive border. Light-mode full-strength.
    #[test]
    fn invalid_unchecked_border_is_destructive() {
        let theme = Theme::light();
        let resolved = invalid(&theme, Status::Active { is_checked: false });
        let p = theme;
        assert!(
            close(resolved.border.color, p.destructive.base),
            "expected invalid border = destructive ({:?}), got {:?}",
            p.destructive.base,
            resolved.border.color,
        );
    }

    /// `aria-invalid:aria-checked:border-primary` — when invalid AND checked,
    /// the border drops back to primary, not destructive.
    #[test]
    fn invalid_checked_border_returns_to_primary() {
        let theme = Theme::light();
        let resolved = invalid(&theme, Status::Active { is_checked: true });
        let p = theme;
        assert!(
            close(resolved.border.color, p.primary.base),
            "expected invalid+checked border = primary ({:?}), got {:?}",
            p.primary.base,
            resolved.border.color,
        );
    }

    /// `Bg(Token::Primary)` on `Default` overrides bg to `palette.primary.base`.
    /// Sanity that the atom plumbing reaches `apply()`.
    #[test]
    fn bg_atom_overrides_default_variant() {
        let theme = Theme::dark();
        let spec: Spec = (Default + Bg(Token::Primary)).into();
        let resolved = spec.resolve(&theme, Status::Active { is_checked: false });
        let bg = bg_color(&resolved);
        let expected = theme.primary.base;
        assert!(
            close(bg, expected),
            "expected bg = primary ({expected:?}), got {bg:?}",
        );
    }

    /// Bare `Ring` atom writes to `style.ring` regardless of status, with
    /// the token resolving against the active palette and `offset` dropped.
    #[test]
    fn ring_atom_writes_to_style_ring() {
        let theme = Theme::light();
        let spec: Spec = (Default
            + tweak::Ring {
                color: Token::Ring,
                width: 3.0,
                offset: 2.0,
            })
        .into();
        let resolved = spec.resolve(&theme, Status::Active { is_checked: false });
        let ring = resolved.ring.expect("ring should be set");
        assert_eq!(ring.width, 3.0);
        assert!(close(ring.color, theme.neutral.ring));
    }

    /// shadcn `rounded-[4px]` — literal 4 (NOT a Tailwind preset). We expose
    /// it as `radius::XS` so future 4px-radius widgets pick it up consistently.
    #[test]
    fn default_uses_xs_radius() {
        let theme = Theme::light();
        for is_checked in [false, true] {
            let resolved = default(&theme, Status::Active { is_checked });
            let r = resolved.border.radius;
            // Border::Radius has uniform corners when set from a single f32.
            assert!(
                (r.top_left - 4.0).abs() < 1e-4
                    && (r.top_right - 4.0).abs() < 1e-4
                    && (r.bottom_left - 4.0).abs() < 1e-4
                    && (r.bottom_right - 4.0).abs() < 1e-4,
                "expected radius ≈ 4 on every corner, got {r:?}",
            );
        }
    }

    /// shadcn `aria-invalid:ring-3 ring-destructive/20` — invalid checkbox
    /// emits a 3px ring at destructive @ 20% alpha (light) / 40% (dark) by
    /// default, no focus required. Verifies the variant fn writes the ring
    /// directly (not via a tweak).
    #[test]
    fn invalid_emits_destructive_ring_3px_at_20_percent() {
        let theme = Theme::light();
        let resolved = invalid(&theme, Status::Active { is_checked: false });
        let p = theme;
        let ring = resolved.ring.expect("invalid variant must emit a ring");
        assert!(
            (ring.width - 3.0).abs() < 1e-4,
            "expected 3px ring (shadcn `ring-3`), got {}",
            ring.width,
        );
        let expected_color = p.destructive.base.alpha(0.20);
        assert!(
            close(ring.color, expected_color),
            "expected ring.color = destructive @ 0.20 ({expected_color:?}), got {:?}",
            ring.color,
        );
    }

    /// `dark:aria-invalid:ring-destructive/40` — dark mode bumps the
    /// invalid ring alpha from 0.20 to 0.40.
    #[test]
    fn invalid_dark_ring_alpha_is_40_percent() {
        let theme = Theme::dark();
        let resolved = invalid(&theme, Status::Active { is_checked: false });
        let ring = resolved.ring.expect("invalid variant must emit a ring");
        assert!(
            (ring.color.a - 0.40).abs() < 1e-4,
            "expected dark invalid ring alpha = 0.40, got {}",
            ring.color.a,
        );
    }
}
