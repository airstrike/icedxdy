//! Slider styling — nova style

pub use iced_widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style, StyleFn};

use super::Theme;
use iced_core::{Background, Border, Color};

const RAIL_WIDTH: f32 = 4.0;
const THUMB_RADIUS: f32 = 6.0;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// Nova default — 4px muted rail, primary range, 12px white thumb with ring border.
pub fn default(theme: &Theme, _status: Status) -> Style {
    Style {
        rail: Rail {
            backgrounds: (
                Background::Color(theme.primary.base.into()),
                Background::Color(theme.muted.base.into()),
            ),
            width: RAIL_WIDTH,
            border: Border {
                radius: 999.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
        handle: Handle {
            shape: HandleShape::Circle {
                radius: THUMB_RADIUS,
            },
            background: Background::Color(Color::WHITE),
            border_width: 1.0,
            border_color: theme.neutral.ring.into(),
        },
    }
}
