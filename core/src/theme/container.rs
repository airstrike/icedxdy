//! Container styling

pub use iced_widget::container::{Catalog, Style, StyleFn};

use iced_core::Border;

use super::Theme;
use crate::radius;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn transparent(_theme: &Theme) -> Style {
    Style::default()
}

pub fn background(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette().background.into()),
        ..Default::default()
    }
}

pub fn card(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.card.base.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.border.into(),
        },
        ..Default::default()
    }
}

pub fn card_preview(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.background.into()),
        border: Border {
            radius: iced_core::border::Radius {
                top_left: radius::LG,
                top_right: radius::LG,
                bottom_right: 0.0,
                bottom_left: 0.0,
            },
            width: 0.0,
            color: iced_core::Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

pub fn card_muted(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.muted.base.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.border.into(),
        },
        ..Default::default()
    }
}

pub fn code_block(theme: &Theme) -> Style {
    // shadcn `--color-code` bg + bottom-rounded corners. Sits inside the outer
    // `card` container; the card provides bg-background (page color) on top,
    // and the code area gets this distinct `code` palette bg as a separate
    // band — matching shadcn's bg-card outer + bg-code code structure.
    Style {
        background: Some(theme.palette().code.base.into()),
        border: Border {
            radius: iced_core::border::Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_left: radius::LG,
                bottom_right: radius::LG,
            },
            width: 0.0,
            color: iced_core::Color::TRANSPARENT,
        },
        ..Default::default()
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

    let code_bg = theme.palette().code.base;
    Style {
        background: Some(
            Linear::new(PI)
                .add_stop(0.0, iced_core::Color::TRANSPARENT)
                .add_stop(0.5, code_bg.alpha(0.6).into())
                .add_stop(1.0, code_bg.into())
                .into(),
        ),
        border: Border {
            radius: iced_core::border::Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_left: radius::LG,
                bottom_right: radius::LG,
            },
            width: 0.0,
            color: iced_core::Color::TRANSPARENT,
        },
        ..Default::default()
    }
}
