//! Markdown styling

use iced_widget::container;
use iced_widget::markdown::Catalog;

use super::Theme;

impl Catalog for Theme {
    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a> {
        // theme::container::code_block returns our OKLCH-native Style;
        // wrap to convert to iced::container::Style for the Catalog Class.
        Box::new(|t| super::container::code_block(t).into())
    }
}
