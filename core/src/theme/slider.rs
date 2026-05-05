//! Passthrough styling

pub use iced_widget::slider::{Catalog, Status, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme, status| {
            let iced = theme.iced_theme();
            <iced_core::Theme as Catalog>::style(
                iced,
                &<iced_core::Theme as Catalog>::default(),
                status,
            )
        })
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}
