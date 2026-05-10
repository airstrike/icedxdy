//! Accordion styling — nova style
//!
//! `Style` is OKLCH-native — variants and tweaks operate on our `Color`,
//! `Border` end-to-end. Conversion to iced's sRGB types happens once at the
//! catalog boundary in `widget/src/accordion/element.rs`.
//!
//! Nova classlist (verbatim from `.nova/apps/v4/styles/radix-nova/ui/accordion.tsx`):
//!
//! Item: `not-last:border-b` — every item except the last is bottom-bordered
//! using `--color-border`.
//!
//! Trigger: `flex-1 items-start justify-between rounded-lg border border-transparent
//! py-2.5 text-left text-sm font-medium transition-all outline-none
//! hover:underline focus-visible:border-ring focus-visible:ring-3
//! focus-visible:ring-ring/50 focus-visible:after:border-ring
//! disabled:pointer-events-none disabled:opacity-50`. The chevron icon
//! inherits `text-muted-foreground` and `size-4` (16px).
//!
//! Content: `overflow-hidden text-sm` body with `pt-0 pb-2.5` inner padding
//! and animated open/close (`data-open:animate-accordion-down`,
//! `data-closed:animate-accordion-up`). The body color is whatever the
//! ambient `--color-foreground` resolves to.
//!
//! There is no `Card` variant in shadcn's accordion.tsx itself — the
//! `accordion-card` example wraps the entire `<Accordion>` in a `<Card>`,
//! so it composes externally. We mirror that with the `Card` variant fn
//! here that simply leaves the divider/trigger surfaces alone (the wrapping
//! `card` container provides the chrome). Kept as a named variant so the
//! accordion-card example can declare its style intent at the call site for
//! clarity / future surface tweaks.

use std::ops::Add;

use super::Theme;
use super::tweak::{Trigger, Tweak, TweakSet};
use crate::palette::Palette;
use crate::style;
use crate::style::status;
use crate::{Border, Color};

/// Status of a single accordion trigger row.
///
/// Mirrors the shape of other vendored-widget statuses (`button::Status`,
/// `switch::Status`). The `is_open` flag rides on every variant so the
/// catalog can distinguish open vs. closed visuals if it wants — we
/// currently don't (all variants render the same trigger regardless of
/// open-state — only the body chrome below differs), but the field is
/// kept so future tweaks (e.g. open-state border emphasis) can land
/// without an enum reshape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Resting trigger.
    Active { is_open: bool },
    /// Cursor over the trigger.
    Hovered { is_open: bool },
    /// Trigger has keyboard focus.
    Focused { is_open: bool },
    /// Item is disabled — opacity 50%, no pointer.
    Disabled { is_open: bool },
}

impl Status {
    /// Project the open-flag out of any variant.
    pub fn is_open(self) -> bool {
        match self {
            Status::Active { is_open }
            | Status::Hovered { is_open }
            | Status::Focused { is_open }
            | Status::Disabled { is_open } => is_open,
        }
    }
}

/// OKLCH-native accordion style. One value parametrizes a whole accordion's
/// visuals: divider color (between items), trigger text color, body text
/// color, hover/focus ring, and disabled opacity.
///
/// Field naming follows the shadcn slots: `divider` is the `border-b` line
/// between consecutive items; `trigger_text` and `body_text` are the two
/// distinct text surfaces (the trigger uses `font-medium`, the body uses
/// the regular weight); `ring` is the focus halo painted around the
/// trigger when keyboard-focused.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// Color of the 1px line between consecutive items (`not-last:border-b`).
    /// `Color::TRANSPARENT` to suppress (the `Card` variant uses this).
    pub divider: Color,
    /// Color of the trigger row text. Nova: `--color-foreground`.
    pub trigger_text: Color,
    /// Color of the chevron icon. Nova: `--color-muted-foreground`.
    pub chevron: Color,
    /// Color of the body text. Nova: `--color-foreground` (inherits from
    /// `text-sm` text wrapping).
    pub body_text: Color,
    /// Optional keyboard-focus ring around the trigger row. `None` = no
    /// halo rendered. Mirrors `text_input` / `switch` ring semantics.
    pub ring: Option<Ring>,
    /// Opacity multiplier for disabled items (`disabled:opacity-50`).
    /// Applied once at draw time to all of trigger_text, chevron, body_text.
    pub disabled_alpha: f32,
    /// Border around the entire accordion. `Border::default()` for unbordered
    /// (the `Default` variant). The `Borders` example wraps in a `border` +
    /// `rounded-lg` ring; we surface that as a tweak knob rather than a
    /// dedicated variant — the wrapper container provides the rounding.
    pub border: Border,
    /// Border-radius of the trigger row's focus highlight (`rounded-lg`).
    /// Mirrors shadcn nova's `rounded-lg` on the trigger element.
    pub trigger_radius: crate::Radius,
}

/// Resolved focus-ring fragment painted around the trigger row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    pub color: Color,
    pub width: f32,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            divider: Color::TRANSPARENT,
            trigger_text: Color::TRANSPARENT,
            chevron: Color::TRANSPARENT,
            body_text: Color::TRANSPARENT,
            ring: None,
            disabled_alpha: 0.5,
            border: Border::default(),
            trigger_radius: crate::Radius::from(crate::radius::LG),
        }
    }
}

/// A styling closure that returns our OKLCH-native `Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

// --- Variant fns -------------------------------------------------------------

/// Default accordion variant.
///
/// - `divider` = `palette.border` (the `not-last:border-b` separator).
/// - `trigger_text` = `palette.foreground`.
/// - `chevron` = `palette.muted.foreground`.
/// - `body_text` = `palette.foreground`.
/// - `ring` only on `Focused` status (`focus-visible:ring-3 ring-ring/50`).
/// - `disabled_alpha` = 0.5 (`disabled:opacity-50`).
/// - `border` = transparent / 0px (no outer ring; this is the bare variant).
pub fn default(theme: &Theme, status: Status) -> Style {
    let p = theme;

    let ring = match status {
        Status::Focused { .. } => Some(Ring {
            color: p.neutral.ring.alpha(0.5),
            width: 3.0,
        }),
        Status::Active { .. } | Status::Hovered { .. } | Status::Disabled { .. } => None,
    };

    Style {
        divider: p.neutral.border,
        trigger_text: p.neutral.foreground,
        chevron: p.muted.foreground,
        body_text: p.neutral.foreground,
        ring,
        disabled_alpha: 0.5,
        border: Border::default(),
        trigger_radius: crate::Radius::from(crate::radius::LG),
    }
}

/// Card accordion variant — used by the `accordion-card` shadcn example,
/// which wraps the accordion in a `<Card>`. Same trigger / body surfaces as
/// `default`; suppresses the outer border (the wrapping card already paints
/// one) but keeps the inter-item divider so multiple items still read as
/// distinct rows.
///
/// In practice this is currently identical to `default` — the wrapping
/// `card` container's chrome lives at the call site, so we don't need to
/// duplicate it here. Kept as a named variant for call-site clarity and
/// to provide a stable seam if shadcn adds card-specific accordion tweaks
/// upstream.
pub fn card(theme: &Theme, status: Status) -> Style {
    default(theme, status)
}

// --- Variant unit structs ----------------------------------------------------
//
// Tag types matching the variant fns, mirroring the switch/button shape.

/// Default accordion variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Card accordion variant — see [`card`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card;

// --- Spec --------------------------------------------------------------------

/// An accordion style: a base variant function plus a sequence of tweaks
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
    /// trigger status. The base variant is computed first, then each tweak
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

impl From<Card> for Spec {
    fn from(_: Card) -> Spec {
        Spec::variant(card)
    }
}

impl<T: Into<Tweak>> Add<T> for Default {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Card {
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
// Tweak meanings for accordion:
//   - `Bg`           → the body background (rare; mostly Tweak::BorderColor
//                      moves the divider). Folded into divider for symmetry
//                      with how container handles single-color surfaces.
//   - `Fg`           → trigger_text + body_text together.
//   - `BorderColor`  → divider color.
//   - `BorderWidth`  → outer border width (the `Borders` example uses 1px).
//   - `Radius`       → trigger_radius.
//   - `Ring { ... }` → focus halo override.
//   - `Alpha`        → multiplies `disabled_alpha`. Lets call sites stack a
//                      "softer disabled" on top of the variant default.
//   - `State`        → standard nested-trigger handling.

fn apply(tweak: &Tweak, style: &mut Style, theme: &Theme, status: Status) {
    match tweak {
        Tweak::Bg(_) => {
            // Accordion has no fill of its own — the wrapping card / preview
            // container provides background. Silently dropped to keep the
            // atom binding stable across widgets without surprising callers.
        }
        Tweak::Fg(t) => {
            let c = t.resolve(theme);
            style.trigger_text = c;
            style.body_text = c;
        }
        Tweak::BorderColor(t) => {
            style.divider = t.resolve(theme);
        }
        Tweak::BorderWidth(w) => {
            style.border = Border {
                width: *w,
                ..style.border
            };
        }
        Tweak::Radius(r) => {
            style.trigger_radius = *r;
            style.border = Border {
                radius: *r,
                ..style.border
            };
        }
        Tweak::Alpha(factor) => {
            style.disabled_alpha *= factor;
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
        Tweak::Opaque
        | Tweak::Shadow(_)
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
        (Trigger::Focus, Status::Focused { .. }) => true,
        (Trigger::Disabled, Status::Disabled { .. }) => true,
        (
            Trigger::Hover,
            Status::Active { .. } | Status::Focused { .. } | Status::Disabled { .. },
        )
        | (
            Trigger::Focus,
            Status::Active { .. } | Status::Hovered { .. } | Status::Disabled { .. },
        )
        | (
            Trigger::Disabled,
            Status::Active { .. } | Status::Hovered { .. } | Status::Focused { .. },
        ) => false,
        // Accordion has no Pressed status; Active is the rest state.
        (Trigger::Active, _) => false,
    }
}

// Accordion carries widget-specific slots (divider / trigger_text / chevron
// / body_text / ring / disabled_alpha / trigger_radius). User overlay maps
// border_color → divider, text_color → trigger_text + body_text,
// border_radius → trigger_radius.

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

impl From<Card> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Card) -> Self {
        style::StyleFn {
            inner: empty_impl,
            tint: style::Tint::Neutral,
        }
    }
}

fn overlay_universal(univ: style::Style, base: Style) -> Style {
    Style {
        divider: univ.border_color.unwrap_or(base.divider),
        trigger_text: univ.text_color.unwrap_or(base.trigger_text),
        chevron: base.chevron,
        body_text: univ.text_color.unwrap_or(base.body_text),
        ring: base.ring,
        disabled_alpha: base.disabled_alpha,
        border: Border {
            width: univ.border_width.unwrap_or(base.border.width),
            color: univ.border_color.unwrap_or(base.border.color),
            radius: univ.border_radius.unwrap_or(base.border.radius),
        },
        trigger_radius: univ.border_radius.unwrap_or(base.trigger_radius),
    }
}

fn bridge_with<F, D>(sf: style::StyleFn<F>, defaults: D) -> StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
    D: Fn(&Theme, Status) -> Style + 'static,
{
    Box::new(move |theme, accordion_status| {
        let s = status::from_accordion(accordion_status);
        let palette = theme.tinted(sf.tint);
        let user = (sf.inner)(palette, s);
        let base = defaults(theme, accordion_status);
        overlay_universal(user, base)
    })
}

impl<F> From<style::StyleFn<F>> for StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
{
    fn from(sf: style::StyleFn<F>) -> Self {
        bridge_with(sf, default)
    }
}

/// `.style(<tint>)` shorthand for the vendored accordion widget.
impl From<style::Tint> for StyleFn<'static, Theme> {
    fn from(t: style::Tint) -> Self {
        let sf: style::StyleFn<fn(Palette, status::Status) -> style::Style> = t.into();
        sf.into()
    }
}

// --- StyleFn conversions -----------------------------------------------------
//
// Allow variants and Specs to be passed directly to wrapper `.style()` builders
// that take `impl Into<StyleFn<'a, Theme>>`.

impl<'a> From<Default> for StyleFn<'a, Theme> {
    fn from(t: Default) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, default)
    }
}

impl<'a> From<Card> for StyleFn<'a, Theme> {
    fn from(t: Card) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, card)
    }
}

impl<'a> From<Spec> for StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |theme, status| spec.resolve(theme, status))
    }
}

#[cfg(test)]
mod tests {
    use super::super::tweak;
    use super::super::tweak::Token;
    use super::*;

    fn close(a: Color, b: Color) -> bool {
        (a.l - b.l).abs() < 1e-4 && (a.c - b.c).abs() < 1e-4 && (a.a - b.a).abs() < 1e-4
    }

    /// Default divider = `palette.border`.
    #[test]
    fn default_divider_is_border() {
        let theme = Theme::light();
        let resolved = default(&theme, Status::Active { is_open: false });
        assert!(close(resolved.divider, theme.neutral.border));
    }

    /// Default chevron = `palette.muted.foreground`.
    #[test]
    fn default_chevron_is_muted_foreground() {
        let theme = Theme::light();
        let resolved = default(&theme, Status::Active { is_open: false });
        assert!(close(resolved.chevron, theme.muted.foreground));
    }

    /// Focus-visible status writes a ring at width 3 and `ring/0.5` alpha.
    #[test]
    fn focused_status_writes_ring() {
        let theme = Theme::light();
        let resolved = default(&theme, Status::Focused { is_open: false });
        let ring = resolved.ring.expect("focused trigger should have ring");
        assert_eq!(ring.width, 3.0);
        let expected = theme.neutral.ring.alpha(0.5);
        assert!(close(ring.color, expected));
    }

    /// Non-focused statuses leave ring as None.
    #[test]
    fn unfocused_statuses_have_no_ring() {
        let theme = Theme::light();
        for status in [
            Status::Active { is_open: false },
            Status::Hovered { is_open: false },
            Status::Disabled { is_open: false },
        ] {
            let resolved = default(&theme, status);
            assert!(
                resolved.ring.is_none(),
                "expected ring=None for {status:?}, got {:?}",
                resolved.ring,
            );
        }
    }

    /// `Card` variant currently mirrors `Default` — covered via Spec
    /// resolution to ensure the `From<Card> for Spec` plumbing reaches.
    #[test]
    fn card_variant_resolves_via_spec() {
        let theme = Theme::dark();
        let spec: Spec = Card.into();
        let resolved = spec.resolve(&theme, Status::Active { is_open: false });
        let baseline = card(&theme, Status::Active { is_open: false });
        assert_eq!(resolved.divider, baseline.divider);
        assert_eq!(resolved.trigger_text, baseline.trigger_text);
    }

    /// `border::Color(Token::Foreground)` overrides the divider.
    #[test]
    fn border_color_atom_overrides_divider() {
        let theme = Theme::light();
        let border_color = tweak::border::Color(Token::Foreground);
        let spec: Spec = (Default + border_color).into();
        let resolved = spec.resolve(&theme, Status::Active { is_open: false });
        assert!(close(resolved.divider, theme.neutral.foreground));
    }
}
