//! Field card-mode container styling
//!
//! Provides two container variants for `Field`'s optional card wrapper:
//!
//! - [`Card`] — inactive border (`palette.border`) + transparent background.
//!   Matches shadcn FieldLabel's `has-[>[data-slot=field]]:border` rule.
//! - [`CardActive`] — primary-tinted border (`palette.primary` @ 30% alpha)
//!   + primary-tinted background (`palette.primary` @ 5% alpha).
//!   Matches `has-data-checked:border-primary/30` and `has-data-checked:bg-primary/5`.
//!
//! Both carry 8px corner radius (`rounded-lg` per nova) and 1px border width.
//! The caller selects the variant based on the inner widget's checked/active
//! state — iced has no CSS-style `data-checked` cascade.

use super::Theme;
use super::container::Style;
use crate::{Border, radius};

// --- Variant fns -------------------------------------------------------------

/// Inactive card: 1px `border-border`, transparent bg, `rounded-lg`.
///
/// Matches shadcn's `has-[>[data-slot=field]]:rounded-lg has-[>[data-slot=field]]:border`
/// (inactive branch, no `has-data-checked` overrides).
pub fn card(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: None,
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: p.border,
        },
        ..Style::default()
    }
}

/// Active card: 1px `border-primary/30`, `bg-primary/5`, `rounded-lg`.
///
/// Matches shadcn's `has-data-checked:border-primary/30 has-data-checked:bg-primary/5`.
pub fn card_active(theme: &Theme) -> Style {
    use crate::Background;
    let p = theme.palette();
    Style {
        background: Some(Background::Color(p.primary.base.alpha(0.05))),
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: p.primary.base.alpha(0.30),
        },
        ..Style::default()
    }
}

// --- Variant unit structs ----------------------------------------------------

/// Inactive field card container variant.
///
/// Renders a 1px `palette.border`-colored border with transparent background
/// and `rounded-lg` (`radius::MD`) corners. Use when the inner control is
/// unchecked / inactive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card;

/// Active field card container variant.
///
/// Renders `border-primary/30` + `bg-primary/5` chrome. Use when the inner
/// control is checked / active. Caller is responsible for selecting between
/// `Card` and `CardActive` — iced has no `data-checked` cascade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardActive;

// --- Bridge to iced::container::StyleFn --------------------------------------

impl<'a> From<Card> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: Card) -> Self {
        Box::new(|t| card(t).into())
    }
}

impl<'a> From<CardActive> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: CardActive) -> Self {
        Box::new(|t| card_active(t).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Inactive card: transparent bg, 1px border using `palette.border`.
    #[test]
    fn card_uses_palette_border() {
        let theme = Theme::dark();
        let p = theme.palette();
        let style = card(&theme);
        assert!(
            style.background.is_none(),
            "inactive card must have no background",
        );
        assert!(
            (style.border.width - 1.0).abs() < 1e-4,
            "card must have a 1px border",
        );
        let bc = style.border.color;
        let expected = p.border;
        assert!(
            (bc.l - expected.l).abs() < 1e-3
                && (bc.c - expected.c).abs() < 1e-3
                && (bc.a - expected.a).abs() < 1e-3,
            "inactive card border must match palette.border. got {bc:?}, expected {expected:?}",
        );
    }

    /// Active card: primary-tinted bg and border.
    #[test]
    fn card_active_uses_palette_primary() {
        use crate::Background;
        let theme = Theme::dark();
        let p = theme.palette();
        let style = card_active(&theme);
        // Border: primary @ 30%
        let bc = style.border.color;
        let ep = p.primary.base;
        assert!(
            (bc.l - ep.l).abs() < 1e-3 && (bc.c - ep.c).abs() < 1e-3,
            "active card border must share primary hue. got {bc:?}",
        );
        assert!(
            (bc.a - 0.30).abs() < 1e-3,
            "active card border alpha must be 0.30, got {}",
            bc.a,
        );
        // Background: primary @ 5%
        match style.background {
            Some(Background::Color(bg)) => {
                assert!(
                    (bg.l - ep.l).abs() < 1e-3 && (bg.c - ep.c).abs() < 1e-3,
                    "active card bg must share primary hue. got {bg:?}",
                );
                assert!(
                    (bg.a - 0.05).abs() < 1e-3,
                    "active card bg alpha must be 0.05, got {}",
                    bg.a,
                );
            }
            other => panic!("expected Background::Color for active card, got {other:?}"),
        }
    }
}
