use iced_widget::overlay::menu::{self, Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(|theme| menu::default(theme.iced_theme()))
    }

    fn style(&self, class: &StyleFn<'_, Self>) -> Style {
        class(self)
    }
}
