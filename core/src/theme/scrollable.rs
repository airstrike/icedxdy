//! Scrollable styling — project-wide thin invisible-rail default.
//!
//! Spec (matches `examples/browser` precedent before vendoring):
//!
//! - Rail (track) background: TRANSPARENT — invisible.
//! - Rail border: `Border::default()` — none.
//! - Scroller width: 2px (set on the [`Scrollbar`] direction itself by the
//!   icedxdy widget's defaults; the style here only paints the scroller).
//! - Scroller color: `palette.muted.foreground` at alpha 0.4 (Active),
//!   0.7 (Hovered), 0.9 (Dragged). Reads as a faint hairline in both
//!   light and dark mode while still being draggable.
//! - Scroller border radius: rounded (1px) so the 2px-wide scroller
//!   reads as a pill, not a rectangle.
//! - AutoScroll overlay: kept as the OKLCH-native styling parity with
//!   the previous default (used only when `auto_scroll(true)` is set).
//!
//! Iced's stock default is a chunky 10px rail with `palette.background.weak`
//! background and `palette.background.strongest` scroller — the visual shape
//! used by `iced::Theme` consumers. The `Theme` Catalog impl below replaces
//! that with the thin spec for icedxdy users.
//!
//! Variants:
//! - `Default` — the same thin spec, exposed as a unit struct so callers can
//!   re-attach it explicitly (`.class(scrollable::Default)`).
//! - `Hidden` — drops the scroller entirely, useful when the scroll affordance
//!   should be implicit (e.g., a clipped row that scrolls via wheel only).

use iced_core::{Background, Border, Color, Shadow, Vector};
use iced_widget::container;
use iced_widget::scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// Variant: project-wide thin invisible-rail default.
///
/// Re-exposed as a unit struct so callers can re-attach it explicitly via
/// `.class(scrollable::Default)` after switching off another variant.
#[derive(Debug, Clone, Copy, Default)]
pub struct Default;

impl From<Default> for StyleFn<'static, Theme> {
    fn from(_: Default) -> Self {
        Box::new(default)
    }
}

/// Variant: zero-alpha scroller — content scrolls but no scrollbar paint.
#[derive(Debug, Clone, Copy, Default)]
pub struct Hidden;

impl From<Hidden> for StyleFn<'static, Theme> {
    fn from(_: Hidden) -> Self {
        Box::new(hidden)
    }
}

/// The project-wide default scrollable style: thin scroller, invisible rail.
pub fn default(theme: &Theme, status: Status) -> Style {
    let p = theme;

    let scroller_alpha = match status {
        Status::Dragged { .. } => 0.9,
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } if is_horizontal_scrollbar_hovered || is_vertical_scrollbar_hovered => 0.7,
        _ => 0.4,
    };

    let rail = Rail {
        // Invisible rail — track is transparent, no border.
        background: None,
        border: Border::default(),
        scroller: Scroller {
            background: Background::Color(p.muted.foreground.alpha(scroller_alpha).into()),
            border: Border {
                radius: 1.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
    };

    Style {
        container: container::Style::default(),
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
        auto_scroll: auto_scroll_overlay(theme),
    }
}

/// Variant style: no scroller paint at all (rail and scroller transparent).
pub fn hidden(theme: &Theme, _status: Status) -> Style {
    let rail = Rail {
        background: None,
        border: Border::default(),
        scroller: Scroller {
            background: Color::TRANSPARENT.into(),
            border: Border::default(),
        },
    };

    Style {
        container: container::Style::default(),
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
        auto_scroll: auto_scroll_overlay(theme),
    }
}

/// Shared autoscroll overlay styling — OKLCH-native parity with the
/// previous default. Only painted when `Scrollable::auto_scroll(true)`
/// is set and middle-click triggers the autoscroll mode.
fn auto_scroll_overlay(theme: &Theme) -> AutoScroll {
    let p = theme;
    AutoScroll {
        background: Background::Color(p.neutral.base.alpha(0.9).into()),
        border: Border {
            radius: (u32::MAX as f32).into(),
            width: 1.0,
            color: p.neutral.border.into(),
        },
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(0.3),
            offset: Vector::ZERO,
            blur_radius: 2.0,
        },
        icon: p.neutral.foreground.alpha(0.6).into(),
    }
}
