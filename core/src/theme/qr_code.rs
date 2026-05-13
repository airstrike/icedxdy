//! QR-code styling — passthrough from iced's stock qr_code Catalog,
//! re-pointed at our palette so the QR cells/background follow the
//! current Theme.

pub use iced_widget::qr_code::{Catalog, Style, StyleFn};

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
    Style {
        cell: theme.neutral.foreground.into(),
        background: theme.neutral.base.into(),
    }
}
