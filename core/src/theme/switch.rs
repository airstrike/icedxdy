//! Switch styling — nova style
//!
//! `Style` is OKLCH-native — variants and tweaks operate on our `Color`,
//! `Border`, `Background` end-to-end. Conversion to iced's sRGB types happens
//! once at the catalog boundary (`From<Style> for iced::toggler::Style`).
//!
//! Nova classlist (verbatim from `.nova/apps/v4/styles/radix-nova/ui/switch.tsx`):
//!
//! ```text
//! peer group/switch relative inline-flex shrink-0 items-center
//! rounded-full border border-transparent transition-all outline-none
//! after:absolute after:-inset-x-3 after:-inset-y-2
//! focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50
//! aria-invalid:border-destructive aria-invalid:ring-3
//! aria-invalid:ring-destructive/20
//! data-[size=default]:h-[18.4px] data-[size=default]:w-[32px]
//! dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40
//! data-checked:bg-primary data-unchecked:bg-input
//! dark:data-unchecked:bg-input/80
//! data-disabled:cursor-not-allowed data-disabled:opacity-50
//! ```
//!
//! Thumb classlist:
//!
//! ```text
//! pointer-events-none block rounded-full bg-background ring-0
//! transition-transform group-data-[size=default]/switch:size-4
//! group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)]
//! dark:data-checked:bg-primary-foreground
//! group-data-[size=default]/switch:data-unchecked:translate-x-0
//! dark:data-unchecked:bg-foreground
//! ```

pub use iced_widget::toggler::Status;

use std::ops::Add;

use super::Theme;
use super::tweak::{Trigger, Tweak, TweakSet};
use crate::palette::Palette;
use crate::style;
use crate::style::status;
use crate::{Background, Color};

/// OKLCH-native switch style. Mirrors `iced_widget::toggler::Style`
/// field-for-field, plus a `ring` field with no iced equivalent — rendered
/// by our vendored `widget::switch` as a halo behind the track.
///
/// Field naming (`background` / `foreground`) is preserved verbatim from
/// iced's stock toggler `Style`; `background` is the track (the bigger
/// pill) and `foreground` is the thumb (the smaller circle that slides).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The track of the switch (the bigger pill).
    pub background: Background,
    /// Width of the track border.
    pub background_border_width: f32,
    /// Color of the track border.
    pub background_border_color: Color,
    /// The thumb of the switch (the smaller circle that slides).
    pub foreground: Background,
    /// Width of the thumb border.
    pub foreground_border_width: f32,
    /// Color of the thumb border.
    pub foreground_border_color: Color,
    /// Optional inline label text color.
    pub text_color: Option<Color>,
    /// The border radius of the track.
    ///
    /// `None` → painted as a perfect pill (radius = height / 2).
    pub border_radius: Option<crate::Radius>,
    /// The ratio of separation between the track and the thumb, as a
    /// fraction of track height.
    pub padding_ratio: f32,
    /// Optional focus ring (Tailwind `ring-*`). `None` = no halo rendered.
    pub ring: Option<Ring>,
}

/// Resolved focus-ring fragment: a solid colored halo painted at `width`
/// pixels around the track bounds, behind the main fill. Equivalent to
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
            background_border_width: 0.0,
            background_border_color: Color::TRANSPARENT,
            foreground: Background::Color(Color::TRANSPARENT),
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
            text_color: None,
            border_radius: None,
            // Nova: thumb is `size-4` (16 × 16 px), track height is 18.4 px.
            // Padding = (18.4 − 16) / 2 = 1.2 px → ratio = 1.2 / 18.4 ≈ 0.065.
            // (Sweeten's upstream baseline was 0.06; adjusted for exact spec.)
            padding_ratio: 0.065,
            ring: None,
        }
    }
}

/// A styling closure that returns our OKLCH-native `Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

/// Convert from iced's stock `toggler::Style` into ours. Lossless in this
/// direction (iced has no `ring` field, so we set `ring: None`).
impl From<iced_widget::toggler::Style> for Style {
    fn from(s: iced_widget::toggler::Style) -> Self {
        Style {
            background: s.background.into(),
            background_border_width: s.background_border_width,
            background_border_color: s.background_border_color.into(),
            foreground: s.foreground.into(),
            foreground_border_width: s.foreground_border_width,
            foreground_border_color: s.foreground_border_color.into(),
            text_color: s.text_color.map(Into::into),
            border_radius: s.border_radius.map(Into::into),
            padding_ratio: s.padding_ratio,
            ring: None,
        }
    }
}

/// The inverse: converting our `Style` to iced's silently drops `ring`. We
/// keep this impl so stock iced code paths that consume an
/// `iced::toggler::Style` still work.
impl From<Style> for iced_widget::toggler::Style {
    fn from(s: Style) -> Self {
        iced_widget::toggler::Style {
            background: s.background.into(),
            background_border_width: s.background_border_width,
            background_border_color: s.background_border_color.into(),
            foreground: s.foreground.into(),
            foreground_border_width: s.foreground_border_width,
            foreground_border_color: s.foreground_border_color.into(),
            text_color: s.text_color.map(Into::into),
            border_radius: s.border_radius.map(Into::into),
            padding_ratio: s.padding_ratio,
        }
    }
}

/// Bridge so stock iced widgets that compose a toggler continue to work
/// with our `Theme`. Routes through our OKLCH resolver and converts Style
/// at the boundary. Ring is dropped at the iced::Style boundary — those
/// compositors can't render it anyway.
impl iced_widget::toggler::Catalog for Theme {
    type Class<'a> = iced_widget::toggler::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|t, s| default(t, s).into())
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> iced_widget::toggler::Style {
        class(self, status)
    }
}

fn is_dark(theme: &Theme) -> bool {
    theme.neutral.base.l < 0.5
}

fn is_toggled(status: Status) -> bool {
    match status {
        Status::Active { is_toggled }
        | Status::Hovered { is_toggled }
        | Status::Disabled { is_toggled } => is_toggled,
    }
}

// --- Variant fns -------------------------------------------------------------

/// Default switch variant — nova default size (32px wide, 18.4px tall).
///
/// Track-checked = `bg-primary`. Track-unchecked light = `bg-input`; dark =
/// `bg-input/80`. Thumb light = `bg-background`; thumb dark when checked =
/// `bg-primary-foreground`; thumb dark when unchecked = `bg-foreground`.
pub fn default(theme: &Theme, status: Status) -> Style {
    let p = theme;
    let dark = is_dark(theme);
    let toggled = is_toggled(status);

    let track = if toggled {
        // `data-checked:bg-primary` — same in both themes.
        p.primary.base
    } else if dark {
        // FIXME(renderer): nova ships `dark:data-unchecked:bg-input/80` —
        // i.e. `oklch(1 0 0 / 12%)` (input is `oklch(1 0 0 / 15%)`, scaled
        // by 0.8). Composited over `--background` (`oklch(0.145 0 0)`) in
        // a faithful renderer that yields a muted dark gray. iced's
        // current alpha-compositing path renders that translucent white
        // close to opaque white, making the unchecked track read as
        // bright on the dark page bg. Workaround: substitute the
        // perceptually-similar opaque `muted.base` token (also a flat
        // dark gray in the dark palette). When iced's compositor is
        // fixed upstream, restore `p.input.alpha(0.8)` here.
        p.muted.base
    } else {
        // `data-unchecked:bg-input`.
        p.input
    };

    let thumb = if dark {
        if toggled {
            // `dark:data-checked:bg-primary-foreground`.
            p.primary.foreground
        } else {
            // `dark:data-unchecked:bg-foreground`.
            p.neutral.foreground
        }
    } else {
        // `bg-background` (default thumb in light mode for both states).
        p.neutral.base
    };

    let active = Style {
        background: track.into(),
        foreground: thumb.into(),
        ..Style::default()
    };

    match status {
        // Nova has no `:hover` rule on switch — Hovered must equal Active.
        Status::Active { .. } | Status::Hovered { .. } => active,
        Status::Disabled { .. } => Style {
            // `data-disabled:opacity-50` — multiply each region by 0.5.
            background: active.background.scale_alpha(0.5),
            foreground: active.foreground.scale_alpha(0.5),
            text_color: Some(p.muted.foreground),
            ..active
        },
    }
}

/// Invalid switch variant — nova `aria-invalid:*` track.
///
/// Border swaps to `palette.destructive` (`aria-invalid:border-destructive`);
/// the focus ring uses `palette.destructive` at low alpha
/// (`aria-invalid:ring-destructive/20`). In dark mode the destructive border
/// drops to alpha 0.5 (`dark:aria-invalid:border-destructive/50`).
pub fn invalid(theme: &Theme, status: Status) -> Style {
    let p = theme;
    let dark = is_dark(theme);
    let base = default(theme, status);

    let invalid_border = if dark {
        p.destructive.base.alpha(0.5)
    } else {
        p.destructive.base
    };

    match status {
        Status::Active { .. } | Status::Hovered { .. } => Style {
            background_border_color: invalid_border,
            background_border_width: 1.0,
            ..base
        },
        Status::Disabled { .. } => Style {
            background_border_color: invalid_border.scale_alpha(0.5),
            background_border_width: 1.0,
            ..base
        },
    }
}

/// Disabled switch variant — explicit disabled styling regardless of the
/// runtime `Status`. Useful as a tag-style variant where the call site
/// declares its disabled intent at construction time. Renders identically to
/// [`default`] under `Status::Disabled`.
pub fn disabled(theme: &Theme, status: Status) -> Style {
    let forced = match status {
        Status::Active { is_toggled }
        | Status::Hovered { is_toggled }
        | Status::Disabled { is_toggled } => Status::Disabled { is_toggled },
    };
    default(theme, forced)
}

// --- Variant unit structs ----------------------------------------------------
//
// Tag types matching the variant fns. Each impls `Add<T: Into<Tweak>>` to
// produce a `Spec`, and `From<Variant> for Spec` for implicit promotion.

/// Default switch variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Invalid switch variant — see [`invalid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invalid;

/// Disabled switch variant — see [`disabled`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Disabled;

// --- Spec --------------------------------------------------------------------

/// A switch style: a base variant function plus a sequence of tweaks
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
    /// switch status. The base variant is computed first, then each tweak
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
//
// `Tweak::Bg` targets the track (the visible "main" region of the switch);
// `Tweak::Fg` targets the thumb. Same figure-region atom binding the
// progress_bar catalog established (see `core/src/theme/progress_bar.rs`
// lines 158-167: `Bg` → ground/track, `Fg` → figure/active region).
// `Tweak::BorderColor` / `Tweak::BorderWidth` target the track border so
// invalid-style overrides via atoms work.

fn apply(tweak: &Tweak, style: &mut Style, theme: &Theme, status: Status) {
    match tweak {
        Tweak::Bg(t) => {
            style.background = t.resolve(theme).into();
        }
        Tweak::Fg(t) => {
            // For a switch the "foreground" region is the thumb.
            style.foreground = t.resolve(theme).into();
        }
        Tweak::BorderColor(t) => {
            style.background_border_color = t.resolve(theme);
        }
        Tweak::BorderWidth(w) => {
            style.background_border_width = *w;
        }
        Tweak::Radius(r) => {
            style.border_radius = Some(*r);
        }
        Tweak::Opaque => {
            // OKLCH alpha-composite the variant's track bg over palette.background
            // and force alpha=1.0. Same shape as button/container/checkbox.
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
        // CSS-style cascade: tweaks that don't apply to switch styling are
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
        // Switch has no Pressed; Active is the rest state. Focus tracking
        // happens in the vendored widget — Spec sees Status without focus,
        // so `Trigger::Focus` and `Trigger::Active` are silently dropped.
        (Trigger::Focus | Trigger::Active, _) => false,
    }
}

// Switch carries widget-specific slots (thumb, padding_ratio) the universal
// Style can't represent — per-tag intrinsics produce the full widget Style;
// user overlay maps onto track bg / border / radius only.

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

fn overlay_universal(univ: style::Style, base: Style) -> Style {
    Style {
        background: univ.fill.map(Into::into).unwrap_or(base.background),
        background_border_width: univ.border_width.unwrap_or(base.background_border_width),
        background_border_color: univ.border_color.unwrap_or(base.background_border_color),
        foreground: base.foreground,
        foreground_border_width: base.foreground_border_width,
        foreground_border_color: base.foreground_border_color,
        text_color: univ.text_color.or(base.text_color),
        border_radius: univ.border_radius.or(base.border_radius),
        padding_ratio: base.padding_ratio,
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
        let s = status::from_toggler(iced_status);
        let palette = theme.tinted(sf.tint);
        let user = (sf.inner)(palette, s);
        let base = defaults(theme, iced_status);
        convert(overlay_universal(user, base))
    })
}

impl<F> From<style::StyleFn<F>> for StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
{
    fn from(sf: style::StyleFn<F>) -> Self {
        bridge_with(sf, default, std::convert::identity)
    }
}

impl From<style::Tint> for StyleFn<'static, Theme> {
    fn from(t: style::Tint) -> Self {
        let sf: style::StyleFn<fn(Palette, status::Status) -> style::Style> = t.into();
        sf.into()
    }
}

impl<F> From<style::StyleFn<F>> for iced_widget::toggler::StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
{
    fn from(sf: style::StyleFn<F>) -> Self {
        bridge_with(sf, default, Into::<iced_widget::toggler::Style>::into)
    }
}

impl From<style::Tint> for iced_widget::toggler::StyleFn<'static, Theme> {
    fn from(t: style::Tint) -> Self {
        let sf: style::StyleFn<fn(Palette, status::Status) -> style::Style> = t.into();
        sf.into()
    }
}

// --- StyleFn conversions (vendored widget) -----------------------------------

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

// --- Iced-side StyleFn conversions -------------------------------------------
//
// Stock iced widgets that compose a toggler call `Theme::style` which returns
// iced's `toggler::Style`. Same routing as above; convert maps to iced's
// shape at the boundary (drops `ring`).

impl<'a> From<Default> for iced_widget::toggler::StyleFn<'a, Theme> {
    fn from(t: Default) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, default, Into::<iced_widget::toggler::Style>::into)
    }
}

impl<'a> From<Invalid> for iced_widget::toggler::StyleFn<'a, Theme> {
    fn from(t: Invalid) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, invalid, Into::<iced_widget::toggler::Style>::into)
    }
}

impl<'a> From<Disabled> for iced_widget::toggler::StyleFn<'a, Theme> {
    fn from(t: Disabled) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, disabled, Into::<iced_widget::toggler::Style>::into)
    }
}

impl<'a> From<Spec> for iced_widget::toggler::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |theme, status| spec.resolve(theme, status).into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tweak::{self, Bg, Token};
    use super::*;

    fn bg_color(bg: Background) -> Color {
        match bg {
            Background::Color(c) => c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    fn close(a: Color, b: Color) -> bool {
        (a.l - b.l).abs() < 1e-4 && (a.c - b.c).abs() < 1e-4 && (a.a - b.a).abs() < 1e-4
    }

    /// Toggled default = `data-checked:bg-primary` — primary surface fills the
    /// track in both light and dark themes.
    #[test]
    fn toggled_default_track_uses_primary() {
        let theme = Theme::light();
        let resolved = default(&theme, Status::Active { is_toggled: true });
        let track = bg_color(resolved.background);
        let p = theme;
        assert!(
            close(track, p.primary.base),
            "expected track = primary ({:?}), got {track:?}",
            p.primary.base,
        );
    }

    /// Light unchecked track = `data-unchecked:bg-input`.
    #[test]
    fn light_unchecked_track_is_input() {
        let theme = Theme::light();
        let resolved = default(&theme, Status::Active { is_toggled: false });
        let track = bg_color(resolved.background);
        let p = theme;
        assert!(
            close(track, p.input),
            "expected track = input ({:?}), got {track:?}",
            p.input,
        );
    }

    /// Dark unchecked track is the `muted.base` workaround for the renderer
    /// alpha-compositing issue (see FIXME in [`default`]). Nova's CSS source
    /// is `dark:data-unchecked:bg-input/80` (`oklch(1 0 0 / 12%)`); the
    /// faithful render is a muted gray that iced's current compositor
    /// can't reproduce from the translucent value, so we substitute the
    /// opaque `muted.base` token. Restore the alpha-multiplied assertion
    /// when the compositor is fixed.
    #[test]
    fn dark_unchecked_track_is_muted_base_workaround() {
        let theme = Theme::dark();
        let resolved = default(&theme, Status::Active { is_toggled: false });
        let track = bg_color(resolved.background);
        let expected = theme.muted.base;
        assert!(
            close(track, expected),
            "expected track = muted.base ({expected:?}), got {track:?}",
        );
    }

    /// Dark + checked thumb = `dark:data-checked:bg-primary-foreground`.
    #[test]
    fn dark_checked_thumb_is_primary_foreground() {
        let theme = Theme::dark();
        let resolved = default(&theme, Status::Active { is_toggled: true });
        let thumb = bg_color(resolved.foreground);
        let expected = theme.primary.foreground;
        assert!(
            close(thumb, expected),
            "expected thumb = primary.foreground ({expected:?}), got {thumb:?}",
        );
    }

    /// Light thumb = `bg-background` regardless of toggled state.
    #[test]
    fn light_thumb_is_background_regardless_of_toggle() {
        let theme = Theme::light();
        let p = &theme;
        for is_toggled in [false, true] {
            let resolved = default(&theme, Status::Active { is_toggled });
            let thumb = bg_color(resolved.foreground);
            assert!(
                close(thumb, p.neutral.base),
                "is_toggled={is_toggled}: expected thumb = background ({:?}), got {thumb:?}",
                p.neutral.base,
            );
        }
    }

    /// Nova has no `:hover` rule on switch — Hovered must equal Active.
    #[test]
    fn switch_hover_equals_active() {
        let theme = Theme::dark();
        for is_toggled in [false, true] {
            let active = default(&theme, Status::Active { is_toggled });
            let hovered = default(&theme, Status::Hovered { is_toggled });
            assert_eq!(active.background, hovered.background);
            assert_eq!(active.foreground, hovered.foreground);
        }
    }

    /// Invalid + unchecked = destructive track border in light mode.
    #[test]
    fn invalid_unchecked_border_is_destructive() {
        let theme = Theme::light();
        let resolved = invalid(&theme, Status::Active { is_toggled: false });
        let p = theme;
        assert!(
            close(resolved.background_border_color, p.destructive.base),
            "expected invalid border = destructive ({:?}), got {:?}",
            p.destructive.base,
            resolved.background_border_color,
        );
    }

    /// `Bg(Token::Primary)` on `Default` overrides the track to
    /// `palette.primary.base`. Sanity that the atom plumbing reaches `apply()`
    /// and that the `Bg → background` (track) convention from progress_bar
    /// holds for switch.
    #[test]
    fn bg_atom_overrides_track() {
        let theme = Theme::dark();
        let spec: Spec = (Default + Bg(Token::Primary)).into();
        let resolved = spec.resolve(&theme, Status::Active { is_toggled: false });
        let track = bg_color(resolved.background);
        let expected = theme.primary.base;
        assert!(
            close(track, expected),
            "expected track = primary ({expected:?}), got {track:?}",
        );
    }

    /// `Fg(Token::Primary)` on `Default` overrides the thumb to
    /// `palette.primary.base`. Sanity that the `Fg → foreground` (thumb)
    /// convention from progress_bar holds for switch.
    #[test]
    fn fg_atom_overrides_thumb() {
        let theme = Theme::dark();
        let spec: Spec = (Default + tweak::Fg(Token::Primary)).into();
        let resolved = spec.resolve(&theme, Status::Active { is_toggled: false });
        let thumb = bg_color(resolved.foreground);
        let expected = theme.primary.base;
        assert!(
            close(thumb, expected),
            "expected thumb = primary ({expected:?}), got {thumb:?}",
        );
    }

    /// Bare `Ring` atom writes to `style.ring` regardless of status, with
    /// the token resolving against the active palette and `offset` dropped.
    #[test]
    fn switch_ring_atom_writes_to_style_ring() {
        let theme = Theme::light();
        let spec: Spec = (Default
            + tweak::Ring {
                color: Token::Ring,
                width: 3.0,
                offset: 2.0,
            })
        .into();
        let resolved = spec.resolve(&theme, Status::Active { is_toggled: false });
        let ring = resolved.ring.expect("ring should be set");
        assert_eq!(ring.width, 3.0);
        assert!(close(ring.color, theme.neutral.ring));
    }
}
