//! Passthrough styling

pub use iced_widget::float::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme| {
            let iced_theme = theme.iced_theme();
            <iced_core::Theme as Catalog>::style(
                iced_theme,
                &<iced_core::Theme as Catalog>::default(),
            )
        })
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}
