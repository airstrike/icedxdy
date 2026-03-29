//! Text styling

pub use iced_widget::text::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(_theme: &Theme) -> Style {
    Style { color: None }
}

pub fn foreground(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().foreground.into()),
    }
}

pub fn muted(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().muted.foreground.into()),
    }
}

pub fn link(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().primary.base.into()),
    }
}

pub fn destructive(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().destructive.base.into()),
    }
}
