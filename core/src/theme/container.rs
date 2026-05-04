//! Container styling
//!
//! `Style` is OKLCH-native — variants and tweaks operate on our `Color`,
//! `Border`, `Shadow`, `Background` end-to-end. Conversion to iced's sRGB
//! types happens once at the catalog boundary.

use std::ops::Add;

use super::Theme;
use super::tweak::{Token, Trigger, Tweak, TweakSet};
use crate::{Background, Border, Color, Palette, Shadow, radius};

/// OKLCH-native container style. Mirrors `iced_widget::container::Style`
/// field-for-field, but the color-bearing fields use our types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// Optional text color cascaded into descendants.
    pub text_color: Option<Color>,
    /// The container's background.
    pub background: Option<Background>,
    /// The container's border.
    pub border: Border,
    /// The container's drop shadow.
    pub shadow: Shadow,
    /// Whether the container should be snapped to the pixel grid.
    pub snap: bool,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            text_color: None,
            background: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        }
    }
}

/// A styling closure that returns our OKLCH-native `Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl From<Style> for iced_widget::container::Style {
    fn from(s: Style) -> Self {
        iced_widget::container::Style {
            text_color: s.text_color.map(Into::into),
            background: s.background.map(Into::into),
            border: s.border.into(),
            shadow: s.shadow.into(),
            snap: s.snap,
        }
    }
}

impl From<iced_widget::container::Style> for Style {
    fn from(s: iced_widget::container::Style) -> Self {
        Style {
            text_color: s.text_color.map(Into::into),
            background: s.background.map(Into::into),
            border: s.border.into(),
            shadow: s.shadow.into(),
            snap: s.snap,
        }
    }
}

impl iced_widget::container::Catalog for Theme {
    type Class<'a> = iced_widget::container::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|t| transparent(t).into())
    }

    fn style(&self, class: &Self::Class<'_>) -> iced_widget::container::Style {
        class(self)
    }
}

// --- Variant fns -------------------------------------------------------------

pub fn transparent(_theme: &Theme) -> Style {
    Style::default()
}

/// Page background — solid `palette.background`.
pub fn background(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette().background.into()),
        ..Style::default()
    }
}

/// Card surface — bg-card + 1px border-border + rounded-lg.
pub fn card(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.card.base.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.border,
        },
        ..Style::default()
    }
}

/// Preview half of an example card — bg-background, only top corners rounded.
pub fn card_preview(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.background.into()),
        border: Border {
            radius: crate::Radius {
                top_left: radius::LG,
                top_right: radius::LG,
                bottom_right: 0.0,
                bottom_left: 0.0,
            },
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Style::default()
    }
}

/// Muted card surface — bg-muted + 1px border-border + rounded-lg.
pub fn card_muted(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.muted.base.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.border,
        },
        ..Style::default()
    }
}

/// Standalone fenced code block — bg-code, all four corners rounded
/// (`rounded-md`). Used by the markdown viewer for ``` blocks in
/// page-level prose (Usage, API Reference, Cursor, etc.).
pub fn code_block(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette().code.base.into()),
        border: Border {
            radius: radius::MD.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Style::default()
    }
}

/// Code panel attached to a preview above — bg-code, only bottom corners
/// rounded so it sits flush below the preview half of an example card.
/// The example-card outer container's `clip(true)` + `card` style handles
/// the rounded outer chrome; the panel's flat top corners meet the divider
/// rule cleanly.
pub fn code_block_panel(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette().code.base.into()),
        border: Border {
            radius: crate::Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_left: radius::LG,
                bottom_right: radius::LG,
            },
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Style::default()
    }
}

/// Vertical "to top" gradient fade for the View Code overlay — matches
/// shadcn's `linear-gradient(to top, var(--color-code),
/// color-mix(in oklab, var(--color-code) 60%, transparent), transparent)`.
/// Stop 0.0 (top) = transparent, 0.5 (middle) = 60% alpha, 1.0 (bottom) =
/// solid code bg.
pub fn code_fade_overlay(theme: &Theme) -> Style {
    use iced_core::gradient::Linear;
    use std::f32::consts::PI;

    // Gradient stops use iced's Color directly — gradients live in iced's
    // type system since we don't vendor the gradient module.
    let code_bg: iced_core::Color = theme.palette().code.base.into();
    Style {
        background: Some(Background::Gradient(
            Linear::new(PI)
                .add_stop(0.0, iced_core::Color::TRANSPARENT)
                .add_stop(0.5, iced_core::Color { a: 0.6, ..code_bg })
                .add_stop(1.0, code_bg)
                .into(),
        )),
        border: Border {
            radius: crate::Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_left: radius::LG,
                bottom_right: radius::LG,
            },
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Style::default()
    }
}

// --- Variant unit structs ----------------------------------------------------
//
// Tag types matching the variant fn names. Each impls `Add<T: Into<Tweak>>`
// to produce a `Spec`, and `From<Variant> for Spec` for implicit promotion.

/// Page-background variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageBackground;

/// Card surface variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card;

/// Preview-card variant (top-rounded only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardPreview;

/// Muted-card variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardMuted;

/// Code-panel variant (bottom-rounded only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlock;

/// View-Code gradient overlay variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeFadeOverlay;

/// Transparent default variant — useful as the base when only tweaks matter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transparent;

// --- Spec --------------------------------------------------------------------

/// A container style: a base variant function plus a sequence of tweaks
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

impl From<Transparent> for Spec {
    fn from(_: Transparent) -> Spec {
        Spec::variant(transparent)
    }
}

impl From<PageBackground> for Spec {
    fn from(_: PageBackground) -> Spec {
        Spec::variant(background)
    }
}

impl From<Card> for Spec {
    fn from(_: Card) -> Spec {
        Spec::variant(card)
    }
}

impl From<CardPreview> for Spec {
    fn from(_: CardPreview) -> Spec {
        Spec::variant(card_preview)
    }
}

impl From<CardMuted> for Spec {
    fn from(_: CardMuted) -> Spec {
        Spec::variant(card_muted)
    }
}

impl From<CodeBlock> for Spec {
    fn from(_: CodeBlock) -> Spec {
        Spec::variant(code_block)
    }
}

impl From<CodeFadeOverlay> for Spec {
    fn from(_: CodeFadeOverlay) -> Spec {
        Spec::variant(code_fade_overlay)
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

variant_add!(
    Transparent,
    PageBackground,
    Card,
    CardPreview,
    CardMuted,
    CodeBlock,
    CodeFadeOverlay
);

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        self.push(rhs.into())
    }
}

// --- Resolver ----------------------------------------------------------------
//
// Container has no status, so `Tweak::State(...)` silently drops — there's no
// hover/pressed/disabled to react to.

fn apply(tweak: &Tweak, style: &mut Style, palette: &Palette) {
    match tweak {
        Tweak::Bg(t) => {
            style.background = Some(t.resolve(palette).into());
        }
        Tweak::Fg(t) => {
            style.text_color = Some(t.resolve(palette));
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
        Tweak::Shadow(s) => {
            style.shadow = (*s).into();
        }
        Tweak::Opaque => {
            // Same OKLCH alpha-compositing as button — bake any translucent
            // variant bg into a solid color over palette.background.
            if let Some(Background::Color(src)) = style.background {
                let dst = palette.background;
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
        | Tweak::Align(_) => {
            let _ = (Token::Background, Trigger::Hover); // suppress unused
        }
    }
}

// --- StyleFn conversions -----------------------------------------------------

impl<'a> From<Transparent> for StyleFn<'a, Theme> {
    fn from(_: Transparent) -> Self {
        Box::new(transparent)
    }
}

impl<'a> From<PageBackground> for StyleFn<'a, Theme> {
    fn from(_: PageBackground) -> Self {
        Box::new(background)
    }
}

impl<'a> From<Card> for StyleFn<'a, Theme> {
    fn from(_: Card) -> Self {
        Box::new(card)
    }
}

impl<'a> From<CardPreview> for StyleFn<'a, Theme> {
    fn from(_: CardPreview) -> Self {
        Box::new(card_preview)
    }
}

impl<'a> From<CardMuted> for StyleFn<'a, Theme> {
    fn from(_: CardMuted) -> Self {
        Box::new(card_muted)
    }
}

impl<'a> From<CodeBlock> for StyleFn<'a, Theme> {
    fn from(_: CodeBlock) -> Self {
        Box::new(code_block)
    }
}

impl<'a> From<CodeFadeOverlay> for StyleFn<'a, Theme> {
    fn from(_: CodeFadeOverlay) -> Self {
        Box::new(code_fade_overlay)
    }
}

impl<'a> From<Spec> for StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |theme| spec.resolve(theme))
    }
}

// --- Bridge to iced::container::StyleFn --------------------------------------

impl<'a> From<Transparent> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: Transparent) -> Self {
        Box::new(|t| transparent(t).into())
    }
}

impl<'a> From<PageBackground> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: PageBackground) -> Self {
        Box::new(|t| background(t).into())
    }
}

impl<'a> From<Card> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: Card) -> Self {
        Box::new(|t| card(t).into())
    }
}

impl<'a> From<CardPreview> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: CardPreview) -> Self {
        Box::new(|t| card_preview(t).into())
    }
}

impl<'a> From<CardMuted> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: CardMuted) -> Self {
        Box::new(|t| card_muted(t).into())
    }
}

impl<'a> From<CodeBlock> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: CodeBlock) -> Self {
        Box::new(|t| code_block(t).into())
    }
}

impl<'a> From<CodeFadeOverlay> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: CodeFadeOverlay) -> Self {
        Box::new(|t| code_fade_overlay(t).into())
    }
}

impl<'a> From<Spec> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |t| spec.resolve(t).into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tweak::Alpha;
    use super::*;

    fn bg_color(style: &Style) -> Color {
        match style.background {
            Some(Background::Color(c)) => c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    /// `Alpha(0.5)` on `Card` (which has an opaque card surface) must
    /// scale the resolved bg alpha by half.
    #[test]
    fn alpha_scales_card_background_alpha() {
        let theme = Theme::dark();
        let spec: Spec = (Card + Alpha(0.5)).into();
        let resolved = spec.resolve(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.palette().card.base.a * 0.5;
        assert!(
            (bg.a - expected).abs() < 1e-4,
            "expected a={expected}, got a={}",
            bg.a,
        );
    }

    /// `Alpha` on a backgroundless variant is a no-op.
    #[test]
    fn alpha_on_transparent_is_noop() {
        let theme = Theme::dark();
        let spec: Spec = (Transparent + Alpha(0.5)).into();
        let resolved = spec.resolve(&theme);
        assert!(
            resolved.background.is_none(),
            "transparent has no bg; Alpha must not synthesize one",
        );
    }

    /// `Opaque` on `Card`'s already-opaque base is a no-op.
    /// Sanity-check: Card sets `palette.card.base` (alpha=1.0) so OKLCH
    /// alpha-compositing leaves it unchanged.
    #[test]
    fn opaque_on_card_no_change() {
        use super::super::tweak::Opaque;
        let theme = Theme::dark();
        let bare = bg_color(&card(&theme));
        let composed = bg_color(&(Spec::from(Card) + Opaque).resolve(&theme));
        assert!(
            (bare.l - composed.l).abs() < 1e-4
                && (bare.c - composed.c).abs() < 1e-4
                && (bare.a - composed.a).abs() < 1e-4,
            "Card already opaque; Opaque must be a no-op. bare={bare:?}, composed={composed:?}",
        );
    }
}
