//! Text input styling — nova style
//!
//! `Style` is OKLCH-native — variants and tweaks operate on our `Color`,
//! `Border`, `Background` end-to-end. Conversion to iced's sRGB types
//! happens once at the catalog boundary (`From<Style> for
//! iced::text_input::Style`).
//!
//! Nova classlist (verbatim from `.nova/apps/v4/styles/radix-nova/ui/input.tsx`):
//!
//! ```text
//! h-8 w-full min-w-0 rounded-lg border border-input bg-transparent
//! px-2.5 py-1 text-base transition-colors outline-none
//! placeholder:text-muted-foreground
//! focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50
//! disabled:pointer-events-none disabled:cursor-not-allowed
//! disabled:bg-input/50 disabled:opacity-50
//! md:text-sm
//! dark:bg-input/30 dark:disabled:bg-input/80
//! ```

use std::ops::Add;

/// Interaction state of a text input. Mirrors `iced_widget::text_input::Status`
/// field-for-field, but it's our type so we're not bound to iced's choices when
/// adding new states (e.g. `Invalid` for `aria-invalid`).
///
/// Convert from iced's stock `Status` via `From` if you have one in hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The input can be interacted with.
    Active,
    /// The input is being hovered.
    Hovered,
    /// The input is focused.
    Focused {
        /// Whether the input is hovered, while focused.
        is_hovered: bool,
    },
    /// The input cannot be interacted with.
    Disabled,
}

impl From<iced_widget::text_input::Status> for Status {
    fn from(s: iced_widget::text_input::Status) -> Self {
        // Local `use` shadows the outer `Status` so we can match on iced's
        // variants by their bare name; `Self` covers the construction side.
        use iced_widget::text_input::Status;
        match s {
            Status::Active => Self::Active,
            Status::Hovered => Self::Hovered,
            Status::Focused { is_hovered } => Self::Focused { is_hovered },
            Status::Disabled => Self::Disabled,
        }
    }
}

use super::Theme;
use super::tweak::{Trigger, Tweak, TweakSet};
use crate::{Background, Border, Color, Palette, radius};

/// OKLCH-native text input style. Mirrors `iced_widget::text_input::Style`
/// field-for-field, plus a `ring` field with no iced equivalent — rendered
/// by our vendored `widget::text_input` as a halo behind the main quad.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The background of the text input.
    pub background: Background,
    /// The border of the text input.
    pub border: Border,
    /// The color of any leading icon.
    pub icon: Color,
    /// The placeholder color (shown when value is empty).
    pub placeholder: Color,
    /// The typed-value color.
    pub value: Color,
    /// The selection-highlight color.
    pub selection: Color,
    /// Optional focus ring (Tailwind `ring-*`). `None` = no halo rendered.
    pub ring: Option<Ring>,
}

/// Resolved focus-ring fragment: a solid colored halo painted at `width`
/// pixels around the input's bounds, behind the main fill. Equivalent to
/// CSS `box-shadow: 0 0 0 {width}px {color}`. Tailwind's `ring-offset` is
/// dropped — iced has no `outline-offset` analog.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    pub color: Color,
    pub width: f32,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            background: Background::Color(Color::TRANSPARENT),
            border: Border::default(),
            // `Color::default()` is fully transparent — useless as a fallback
            // for value/placeholder/icon. Use BLACK so the field renders
            // *something* if a caller skips the catalog default.
            icon: Color::BLACK,
            placeholder: Color::BLACK,
            value: Color::BLACK,
            selection: Color::BLACK,
            ring: None,
        }
    }
}

/// A styling closure that returns our OKLCH-native `Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

/// Convert from iced's stock `text_input::Style` into ours. Lossless in this
/// direction (iced has no `ring` field, so we set `ring: None`).
impl From<iced_widget::text_input::Style> for Style {
    fn from(s: iced_widget::text_input::Style) -> Self {
        Style {
            background: s.background.into(),
            border: s.border.into(),
            icon: s.icon.into(),
            placeholder: s.placeholder.into(),
            value: s.value.into(),
            selection: s.selection.into(),
            ring: None,
        }
    }
}

/// The inverse: converting our `Style` to iced's silently drops `ring`. We
/// keep this impl because stock iced widgets that compose a text input (e.g.
/// `iced_widget::combo_box`) require `Theme: iced_widget::text_input::Catalog`,
/// and that Catalog impl needs to return iced's `Style`. New code in our
/// crate should prefer the vendored `widget::text_input` widget instead — it
/// understands ring.
impl From<Style> for iced_widget::text_input::Style {
    fn from(s: Style) -> Self {
        iced_widget::text_input::Style {
            background: s.background.into(),
            border: s.border.into(),
            icon: s.icon.into(),
            placeholder: s.placeholder.into(),
            value: s.value.into(),
            selection: s.selection.into(),
        }
    }
}

/// Bridge so stock iced widgets (combo_box etc.) that compose a text input
/// continue to work with our `Theme`. Routes through our OKLCH resolver,
/// converts both Status and Style at the boundary. Ring is dropped at the
/// iced::Style boundary — combo_box can't render it anyway.
impl iced_widget::text_input::Catalog for Theme {
    type Class<'a> = iced_widget::text_input::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|t, s| default(t, s.into()).into())
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced_widget::text_input::Status,
    ) -> iced_widget::text_input::Style {
        class(self, status)
    }
}

fn is_dark(theme: &Theme) -> bool {
    theme.palette().background.l < 0.5
}

// --- Variant fns -------------------------------------------------------------

/// Default text input variant — nova `h-8` field.
///
/// Light bg = `palette.background` (the classlist says `bg-transparent` but
/// we have no parent fill in the preview, so the field's own surface is the
/// page background). Dark bg = `palette.input` at α `input.a × 0.3`. Border
/// = `palette.input`, radius `radius::LG` (10px). Focused border swaps to
/// `palette.ring`. Hover has NO state change (nova has no `:hover` rule).
pub fn default(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();
    let dark = is_dark(theme);

    let bg = if dark {
        // Tailwind `/30` modifier sets alpha to 0.3 (it does NOT multiply
        // with the source's existing alpha). Spec: `dark:bg-input/30`.
        p.input.alpha(0.3)
    } else {
        p.background
    };

    let active = Style {
        background: bg.into(),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.input,
        },
        icon: p.foreground,
        placeholder: p.muted.foreground,
        value: p.foreground,
        selection: p.primary.base.alpha(0.3),
        ring: None,
    };

    match status {
        // Nova has no hover state — Hovered must equal Active.
        Status::Active | Status::Hovered => active,
        Status::Focused { .. } => Style {
            border: Border {
                color: p.ring,
                ..active.border
            },
            // Nova `focus-visible:ring-3 ring-ring/50` — 3px halo at
            // palette.ring with 50% alpha. Rendered by the vendored widget
            // as an expanded fill_quad behind the main fill.
            ring: Some(Ring {
                color: p.ring.alpha(0.5),
                width: 3.0,
            }),
            ..active
        },
        Status::Disabled => {
            // Tailwind `/N` overrides alpha; doesn't multiply with the
            // source. Spec: `disabled:bg-input/50` (light), `dark:disabled:bg-input/80`.
            let dis_bg = if dark {
                p.input.alpha(0.8)
            } else {
                p.input.alpha(0.5)
            };
            // The element-wide `disabled:opacity-50` on top multiplies every
            // pixel by 0.5 — we apply that to non-bg colors with `scale_alpha`
            // so they preserve their *own* base alpha (matters for dark
            // `palette.input` which is α 0.15, not 1.0).
            Style {
                background: dis_bg.scale_alpha(0.5).into(),
                border: Border {
                    color: p.input.scale_alpha(0.5),
                    ..active.border
                },
                value: p.foreground.scale_alpha(0.5),
                placeholder: p.muted.foreground.scale_alpha(0.5),
                icon: p.foreground.scale_alpha(0.5),
                ..active
            }
        }
    }
}

/// Invalid text input variant — nova `aria-invalid:*` field.
///
/// Same layout as [`default`] but with the destructive token swapped in for
/// border (rest), border-on-focus, and ring color. Mirrors the nova class
/// triplet `aria-invalid:border-destructive`,
/// `aria-invalid:focus-visible:ring-destructive/20`,
/// `aria-invalid:focus-visible:border-destructive`.
///
/// Note: shadcn ships `ring-destructive/20` (20% alpha). We currently render
/// the ring at full alpha — proper alpha tracking through the Ring atom is a
/// separate follow-up (needs `Token` to carry an alpha override or a new
/// alpha-aware Ring tweak).
pub fn invalid(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();
    let base = default(theme, status);

    match status {
        Status::Active | Status::Hovered => Style {
            border: Border {
                color: p.destructive.base,
                ..base.border
            },
            ..base
        },
        Status::Focused { .. } => Style {
            border: Border {
                color: p.destructive.base,
                ..base.border
            },
            ring: Some(Ring {
                // FIXME: shadcn `ring-destructive/20` = 20% alpha; full alpha
                // here pending an alpha-aware Ring tweak (see module docs).
                color: p.destructive.base,
                width: 3.0,
            }),
            ..base
        },
        // Disabled propagates the destructive border through the same
        // `scale_alpha(0.5)` dimming applied by `default`.
        Status::Disabled => Style {
            border: Border {
                color: p.destructive.base.scale_alpha(0.5),
                ..base.border
            },
            ..base
        },
    }
}

// --- Variant unit structs ----------------------------------------------------
//
// Tag types matching the variant fns. Each impls `Add<T: Into<Tweak>>` to
// produce a `Spec`, and `From<Variant> for Spec` for implicit promotion.

/// Default text input variant (nova `h-8` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Invalid text input variant (nova `aria-invalid:*` field). See [`invalid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invalid;

// --- Spec --------------------------------------------------------------------

/// A text input style: a base variant function plus a sequence of tweaks
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
    /// input status. The base variant is computed first, then each tweak
    /// is applied in order.
    pub fn resolve(&self, theme: &Theme, status: Status) -> Style {
        let mut style = (self.variant)(theme, status);
        let palette = theme.palette();
        for t in &self.tweaks.0 {
            apply(t, &mut style, palette, status);
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

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        self.push(rhs.into())
    }
}

// --- Resolver ----------------------------------------------------------------

fn apply(tweak: &Tweak, style: &mut Style, palette: &Palette, status: Status) {
    match tweak {
        Tweak::Bg(t) => {
            style.background = t.resolve(palette).into();
        }
        Tweak::Fg(t) => {
            // For an input the semantic "foreground" is the typed value.
            // Placeholder is its own slot and is not adjusted here.
            style.value = t.resolve(palette);
        }
        Tweak::BorderColor(t) => {
            style.border.color = t.resolve(palette);
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
                let dst = palette.background;
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
                    apply(inner_tweak, style, palette, status);
                }
            }
        }
        Tweak::Ring {
            color,
            width,
            offset: _,
        } => {
            // Ring renders as a halo at `width` px around the bounds, behind
            // the main fill. `offset` is dropped (no iced equivalent for
            // Tailwind's `ring-offset` / CSS `outline-offset`).
            style.ring = Some(Ring {
                color: color.resolve(palette),
                width: *width,
            });
        }
        // CSS-style cascade: tweaks that don't apply to text_input styling
        // are silently dropped.
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
        (Trigger::Hover, Status::Hovered) => true,
        (Trigger::Focus, Status::Focused { .. }) => true,
        (Trigger::Disabled, Status::Disabled) => true,
        // text_input has no Pressed; Active is the rest state.
        (Trigger::Active, _) => false,
        _ => false,
    }
}

// --- StyleFn conversions -----------------------------------------------------
//
// Allow variants and Specs to be passed directly to wrapper `.style()` builders
// that take `impl Into<StyleFn<'a, Theme>>`.

impl<'a> From<Default> for StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(default)
    }
}

impl<'a> From<Invalid> for StyleFn<'a, Theme> {
    fn from(_: Invalid) -> Self {
        Box::new(invalid)
    }
}

impl<'a> From<Spec> for StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |theme, status| spec.resolve(theme, status))
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

    /// Dark theme rest bg = `palette.input` with alpha *set* to 0.3.
    /// Regression guard for the `dark:bg-input/30` class — Tailwind's `/N`
    /// modifier overrides the source alpha; it does NOT multiply.
    #[test]
    fn default_dark_bg_uses_input_alpha_0_3() {
        let theme = Theme::dark();
        let resolved = default(&theme, Status::Active);
        let bg = bg_color(&resolved);
        let p = theme.palette();
        let expected = p.input.alpha(0.3);
        assert!(
            close(bg, expected),
            "expected bg ≈ {expected:?}, got {bg:?}",
        );
    }

    /// Focused border = `palette.ring`. Confirms `focus-visible:border-ring`.
    #[test]
    fn focused_border_uses_ring() {
        let theme = Theme::light();
        let resolved = default(&theme, Status::Focused { is_hovered: false });
        let actual = resolved.border.color;
        let expected = theme.palette().ring;
        assert!(
            close(actual, expected),
            "expected focused border = ring ({expected:?}), got {actual:?}",
        );
    }

    /// Nova has no hover state — Hovered must be field-identical to Active.
    /// Preserves the invariant against future drift.
    #[test]
    fn hover_equals_active() {
        let theme = Theme::dark();
        let active = default(&theme, Status::Active);
        let hovered = default(&theme, Status::Hovered);
        assert_eq!(active.background, hovered.background);
        assert_eq!(active.border, hovered.border);
        assert_eq!(active.value, hovered.value);
        assert_eq!(active.placeholder, hovered.placeholder);
        assert_eq!(active.icon, hovered.icon);
        assert_eq!(active.selection, hovered.selection);
    }

    /// `Bg(Token::Primary)` on `Default` overrides bg to `palette.primary.base`.
    /// Sanity that the atom plumbing reaches `apply()`.
    #[test]
    fn bg_atom_overrides_default_variant() {
        let theme = Theme::dark();
        let spec: Spec = (Default + Bg(Token::Primary)).into();
        let resolved = spec.resolve(&theme, Status::Active);
        let actual = bg_color(&resolved);
        let expected = theme.palette().primary.base;
        assert!(
            close(actual, expected),
            "expected bg = primary ({expected:?}), got {actual:?}",
        );
    }

    /// Bare `Ring` atom (no state trigger) writes to `style.ring` regardless
    /// of status. Confirms `apply()` resolves `Token → Color` and drops `offset`.
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
        let resolved = spec.resolve(&theme, Status::Active);
        let ring = resolved.ring.expect("ring should be set");
        assert_eq!(ring.width, 3.0);
        assert!(close(ring.color, theme.palette().ring));
    }

    /// `focus(Ring(...))` writes ring only when status = Focused.
    /// Mirrors the nova classlist `focus-visible:ring-3 ring-ring/50`.
    #[test]
    fn focus_trigger_gates_ring() {
        let theme = Theme::light();
        let spec: Spec = (Default
            + tweak::focus(tweak::Ring {
                color: Token::Ring,
                width: 3.0,
                offset: 0.0,
            }))
        .into();
        for status in [Status::Active, Status::Hovered, Status::Disabled] {
            let resolved = spec.resolve(&theme, status);
            assert!(
                resolved.ring.is_none(),
                "expected ring=None for {status:?}, got {:?}",
                resolved.ring,
            );
        }
        let focused = spec.resolve(&theme, Status::Focused { is_hovered: false });
        assert!(focused.ring.is_some(), "expected ring=Some on Focused");
    }
}
