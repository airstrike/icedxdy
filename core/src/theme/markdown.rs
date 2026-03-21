//! Markdown styling

use iced_widget::container;
use iced_widget::markdown::Catalog;

use super::Theme;

impl Catalog for Theme {
    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a> {
        Box::new(super::container::card)
    }
}
