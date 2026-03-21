use iced_widget::text_input::{self, Catalog, Status, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme, status| text_input::default(theme.iced_theme(), status))
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}
