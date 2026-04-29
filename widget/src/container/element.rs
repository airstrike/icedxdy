//! Catalog impl bridging our `Theme` into the vendored container widget.
//! Mirrors `widget/src/button/element.rs` — pulls the OKLCH-native default
//! styling from `core::theme::container`, runs it through the existing
//! `iced::container::Style` boundary, then converts to the vendored
//! widget's `Style` via the `From` impl in `super::widget`.

use icedxdy_core::Theme;

use super::widget;

fn default(_theme: &Theme) -> widget::Style {
    // Iced's stock default is "transparent"; our container default is the
    // same shape. Users override per call site via `.class(...)`, so this
    // default is rarely hit.
    widget::Style::default()
}

impl widget::Catalog for Theme {
    type Class<'a> = widget::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> widget::Style {
        class(self)
    }
}
