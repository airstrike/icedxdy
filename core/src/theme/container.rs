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
            width: 1.0,
            color: p.border.into(),
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

pub fn code_block(_theme: &Theme) -> Style {
    Style::default()
}
