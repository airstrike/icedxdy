//! Container styling
//!
//! `Style` is OKLCH-native — variants and tweaks operate on our `Color`,
//! `Border`, `Shadow`, `Background` end-to-end. Conversion to iced's sRGB
//! types happens once at the catalog boundary.

use std::ops::Add;

use iced_core::Vector;

use super::Theme;
use super::tweak::{Token, Trigger, Tweak, TweakSet};
use crate::palette::Palette;
use crate::style;
use crate::style::status;
use crate::{Background, Border, Color, Shadow, radius};

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
        background: Some(theme.neutral.base.into()),
        ..Style::default()
    }
}

/// Card surface — bg-card + foreground/10 ring + rounded-xl.
pub fn card(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.card.base.into()),
        border: Border {
            radius: radius::XL.into(),
            width: 1.0,
            color: p.neutral.foreground.alpha(0.10),
        },
        ..Style::default()
    }
}

/// Preview half of an example card — bg-background, only top corners rounded.
pub fn card_preview(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.neutral.base.into()),
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
    let p = theme;
    Style {
        background: Some(p.muted.base.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.neutral.border,
        },
        ..Style::default()
    }
}

/// Standalone fenced code block — bg-code, all four corners rounded
/// (`rounded-md`). Used by the markdown viewer for ``` blocks in
/// page-level prose (Usage, API Reference, Cursor, etc.).
pub fn code_block(theme: &Theme) -> Style {
    Style {
        background: Some(theme.code.base.into()),
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
        background: Some(theme.code.base.into()),
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

/// Inset-variant sibling of `super::sidebar::inset` — the content area
/// when paired with an Inset sidebar. Per shadcn nova
/// (`sidebar.tsx::SidebarInset`, the `peer-data-[variant=inset]` branch):
/// `bg-background rounded-xl shadow-sm`. We approximate `rounded-xl` with
/// `radius::LG` (closest available) and add a 1px border for legibility
/// in flat themes.
pub fn sidebar_inset(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.neutral.base.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.neutral.border,
        },
        shadow: Shadow {
            color: Color::BLACK.alpha(0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
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
    let code_bg: iced_core::Color = theme.code.base.into();
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

/// Sidebar-inset content-area variant — paired with the Inset sidebar
/// variant; carries `bg-background rounded-xl shadow-sm` chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SidebarInset;

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
        for t in &self.tweaks.0 {
            apply(t, &mut style, theme);
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

impl From<SidebarInset> for Spec {
    fn from(_: SidebarInset) -> Spec {
        Spec::variant(sidebar_inset)
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
    SidebarInset,
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
            // Same OKLCH alpha-compositing as button — bake any translucent
            // variant bg into a solid color over palette.background.
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

impl<'a> From<SidebarInset> for StyleFn<'a, Theme> {
    fn from(_: SidebarInset) -> Self {
        Box::new(sidebar_inset)
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

// Container is stateless — Status is synthesized as Active by the bridge,
// and state atoms (hover/focus/etc.) no-op against that.

type BoxedFn = Box<dyn Fn(Palette, status::Status) -> style::Style>;

/// Container's bare default in the universal Style shape: empty (transparent).
pub fn intrinsic_default(_theme: &Theme) -> style::Style {
    style::Style::default()
}

/// Project a universal Style onto iced's container::Style.
///
/// Reads each universal slot and writes the matching iced field; widget-
/// specific defaults (snap=false here) ride along.
pub fn project_container(univ: style::Style) -> iced_widget::container::Style {
    let border = match (univ.border_width, univ.border_color, univ.border_radius) {
        (None, None, None) => iced_core::Border::default(),
        (w, c, r) => Border {
            width: w.unwrap_or(0.0),
            color: c.unwrap_or(Color::TRANSPARENT),
            radius: r.unwrap_or_else(|| 0.0.into()),
        }
        .into(),
    };
    iced_widget::container::Style {
        text_color: univ.text_color.map(Into::into),
        background: univ.fill.map(Into::into),
        border,
        shadow: univ.shadow.unwrap_or_default().into(),
        snap: false,
    }
}

fn lift(inner: fn(Palette) -> style::Style, tint: style::Tint) -> style::StyleFn<BoxedFn> {
    style::StyleFn {
        inner: Box::new(move |p, _s| inner(p)),
        tint,
    }
}

fn transparent_impl(_p: Palette) -> style::Style {
    style::Style::default()
}

fn background_impl(p: Palette) -> style::Style {
    style::Style {
        fill: Some(p.base.into()),
        ..style::Style::default()
    }
}

fn card_preview_impl(p: Palette) -> style::Style {
    style::Style {
        fill: Some(p.base.into()),
        border_radius: Some(crate::Radius {
            top_left: radius::LG,
            top_right: radius::LG,
            bottom_right: 0.0,
            bottom_left: 0.0,
        }),
        ..style::Style::default()
    }
}

fn empty_impl(_p: Palette, _s: status::Status) -> style::Style {
    style::Style::default()
}

// Surface- / mixed-palette tags need theme access the Tint mechanism can't
// supply; each carries its own intrinsic that the bridge calls instead of
// the widget-wide one.

pub fn intrinsic_card(theme: &Theme) -> style::Style {
    style::Style {
        fill: Some(theme.card.base.into()),
        border_width: Some(1.0),
        border_color: Some(theme.neutral.border),
        border_radius: Some(radius::LG.into()),
        ..style::Style::default()
    }
}

pub fn intrinsic_card_muted(theme: &Theme) -> style::Style {
    style::Style {
        fill: Some(theme.muted.base.into()),
        border_width: Some(1.0),
        border_color: Some(theme.neutral.border),
        border_radius: Some(radius::LG.into()),
        ..style::Style::default()
    }
}

pub fn intrinsic_code_block(theme: &Theme) -> style::Style {
    style::Style {
        fill: Some(theme.code.base.into()),
        border_radius: Some(radius::MD.into()),
        ..style::Style::default()
    }
}

pub fn intrinsic_sidebar_inset(theme: &Theme) -> style::Style {
    style::Style {
        fill: Some(theme.neutral.base.into()),
        border_width: Some(1.0),
        border_color: Some(theme.neutral.border),
        border_radius: Some(radius::LG.into()),
        shadow: Some(Shadow {
            color: Color::BLACK.alpha(0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        }),
        ..style::Style::default()
    }
}

pub fn intrinsic_code_fade_overlay(theme: &Theme) -> style::Style {
    use iced_core::gradient::Linear;
    use std::f32::consts::PI;

    let code_bg: iced_core::Color = theme.code.base.into();
    style::Style {
        fill: Some(Background::Gradient(
            Linear::new(PI)
                .add_stop(0.0, iced_core::Color::TRANSPARENT)
                .add_stop(0.5, iced_core::Color { a: 0.6, ..code_bg })
                .add_stop(1.0, code_bg)
                .into(),
        )),
        border_radius: Some(crate::Radius {
            top_left: 0.0,
            top_right: 0.0,
            bottom_left: radius::LG,
            bottom_right: radius::LG,
        }),
        ..style::Style::default()
    }
}

impl From<Transparent> for style::StyleFn<BoxedFn> {
    fn from(_: Transparent) -> Self {
        lift(transparent_impl, style::Tint::Neutral)
    }
}

impl From<PageBackground> for style::StyleFn<BoxedFn> {
    fn from(_: PageBackground) -> Self {
        lift(background_impl, style::Tint::Neutral)
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

impl From<CardPreview> for style::StyleFn<BoxedFn> {
    fn from(_: CardPreview) -> Self {
        lift(card_preview_impl, style::Tint::Neutral)
    }
}

impl From<CardMuted> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: CardMuted) -> Self {
        style::StyleFn {
            inner: empty_impl,
            tint: style::Tint::Neutral,
        }
    }
}

impl From<CodeBlock> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: CodeBlock) -> Self {
        style::StyleFn {
            inner: empty_impl,
            tint: style::Tint::Neutral,
        }
    }
}

impl From<SidebarInset> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: SidebarInset) -> Self {
        style::StyleFn {
            inner: empty_impl,
            tint: style::Tint::Neutral,
        }
    }
}

impl From<CodeFadeOverlay> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: CodeFadeOverlay) -> Self {
        style::StyleFn {
            inner: empty_impl,
            tint: style::Tint::Neutral,
        }
    }
}

impl<F> From<style::StyleFn<F>> for iced_widget::container::StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
{
    fn from(sf: style::StyleFn<F>) -> Self {
        Box::new(move |theme| {
            let palette = theme.tinted(sf.tint);
            let user = (sf.inner)(palette, status::Status::Active);
            let default = intrinsic_default(theme);
            let merged = default + user;
            project_container(merged)
        })
    }
}

/// `.style(<tint>)` shorthand — routes through `style::StyleFn` with an
/// empty inner; container's intrinsic default carries the look.
impl From<style::Tint> for iced_widget::container::StyleFn<'static, Theme> {
    fn from(t: style::Tint) -> Self {
        let sf: style::StyleFn<fn(Palette, status::Status) -> style::Style> = t.into();
        sf.into()
    }
}

pub fn bridge_with<F, D>(
    sf: style::StyleFn<F>,
    defaults: D,
) -> iced_widget::container::StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
    D: Fn(&Theme) -> style::Style + 'static,
{
    Box::new(move |theme| {
        let palette = theme.tinted(sf.tint);
        let user = (sf.inner)(palette, status::Status::Active);
        let default = defaults(theme);
        let merged = default + user;
        project_container(merged)
    })
}

// --- Bridge to iced::container::StyleFn --------------------------------------

impl<'a> From<Transparent> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: Transparent) -> Self {
        let sf: style::StyleFn<_> = t.into();
        sf.into()
    }
}

impl<'a> From<PageBackground> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: PageBackground) -> Self {
        let sf: style::StyleFn<_> = t.into();
        sf.into()
    }
}

impl<'a> From<Card> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: Card) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, intrinsic_card)
    }
}

impl<'a> From<CardPreview> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: CardPreview) -> Self {
        let sf: style::StyleFn<_> = t.into();
        sf.into()
    }
}

impl<'a> From<CardMuted> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: CardMuted) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, intrinsic_card_muted)
    }
}

impl<'a> From<CodeBlock> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: CodeBlock) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, intrinsic_code_block)
    }
}

impl<'a> From<SidebarInset> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: SidebarInset) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, intrinsic_sidebar_inset)
    }
}

impl<'a> From<CodeFadeOverlay> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: CodeFadeOverlay) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge_with(sf, intrinsic_code_fade_overlay)
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
    use crate::style::atoms::{bg, hover};

    fn bg_color(style: &Style) -> Color {
        match style.background {
            Some(Background::Color(c)) => c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    /// `.style(bg(c))` on a container resolves to a container::Style whose
    /// background is `c`. Covers the universal-pipeline → iced::container::Style
    /// projection path.
    #[test]
    fn bg_atom_resolves_to_container_background() {
        let theme = Theme::light();
        let pink = Color::new(0.7, 0.15, 0.0);
        let composed = bg(Background::Color(pink));
        let iced_sf: iced_widget::container::StyleFn<'_, Theme> = composed.into();
        let resolved = iced_sf(&theme);
        match resolved.background {
            Some(iced_core::Background::Color(c)) => {
                let expected: iced_core::Color = pink.into();
                assert!(
                    (c.r - expected.r).abs() < 1e-3 && (c.g - expected.g).abs() < 1e-3,
                    "expected pink, got {c:?}",
                );
            }
            other => panic!("expected solid color background, got {other:?}"),
        }
    }

    /// `.style(hover(bg(c)))` on a container resolves to bare default — the
    /// hover combinator gates on Status::Hovered, but the container bridge
    /// synthesizes Status::Active, so the hover atom no-ops.
    #[test]
    fn hover_combinator_noops_on_container() {
        let theme = Theme::light();
        let pink = Color::new(0.7, 0.15, 0.0);
        let composed = hover(bg(Background::Color(pink)));
        let iced_sf: iced_widget::container::StyleFn<'_, Theme> = composed.into();
        let resolved = iced_sf(&theme);
        // container's intrinsic_default is empty → no background.
        assert!(
            resolved.background.is_none(),
            "expected no background (hover no-ops on stateless container), got {:?}",
            resolved.background,
        );
    }

    /// `Alpha(0.5)` on `Card` (which has an opaque card surface) must
    /// scale the resolved bg alpha by half.
    #[test]
    fn alpha_scales_card_background_alpha() {
        let theme = Theme::dark();
        let spec: Spec = (Card + Alpha(0.5)).into();
        let resolved = spec.resolve(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.card.base.a * 0.5;
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

    /// `SidebarInset` content area carries the inset chrome: bg-background,
    /// LG-rounded corners, 1px border, and a small shadow. Pairs with the
    /// `theme::sidebar::Inset` (no-chrome) sidebar variant.
    #[test]
    fn sidebar_inset_has_chrome() {
        use crate::radius;
        let theme = Theme::dark();
        let resolved = sidebar_inset(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.neutral.base;
        assert!(
            (bg.l - expected.l).abs() < 1e-4 && (bg.c - expected.c).abs() < 1e-4,
            "SidebarInset bg must be palette.background ({expected:?}), got {bg:?}",
        );
        assert_eq!(
            resolved.border.radius,
            radius::LG.into(),
            "SidebarInset must use rounded-lg corners",
        );
        assert!(
            resolved.border.width > 0.0,
            "SidebarInset must carry a non-zero border",
        );
        assert!(
            resolved.shadow.blur_radius > 0.0,
            "SidebarInset must carry a non-zero shadow",
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
