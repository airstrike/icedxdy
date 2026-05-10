//! Table styling

pub use iced_widget::table::{Catalog, Style, StyleFn};

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

pub fn default(theme: &Theme) -> Style {
    let p = theme;

    Style {
        separator_x: p.neutral.border.into(),
        separator_y: p.neutral.border.into(),
    }
}

/// Catalog implementation for sweeten's enhanced table widget. Sweeten's
/// `Style` adds `border`, `sticky_background`, and `header_underline` on
/// top of the stock iced fields — this default mirrors the nova table
/// look (separator + border share `palette.border`, sticky header sits
/// on `palette.background`, no underline).
#[cfg(feature = "sweeten")]
pub mod sweeten_table {
    use sweeten::widget::table::{Catalog, Style, StyleFn};

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

    pub fn default(theme: &Theme) -> Style {
        let p = theme;
        let separator = p.neutral.border.into();

        Style {
            separator_x: separator,
            separator_y: separator,
            border: separator,
            sticky_background: p.neutral.base.into(),
            header_underline: None,
        }
    }
}
