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
