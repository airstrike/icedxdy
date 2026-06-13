use iced_widget::pick_list::{Catalog, Status, Style, StyleFn};

use crate::{Border, Color, Theme, radius};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn default(theme: &Theme, status: Status) -> Style {
    let background = match status {
        Status::Hovered | Status::Opened { .. } => theme.muted.base.alpha(0.5),
        Status::Active | Status::Disabled => Color::TRANSPARENT,
    };

    Style {
        text_color: theme.neutral.foreground.into(),
        placeholder_color: theme.muted.foreground.into(),
        handle_color: theme.muted.foreground.into(),
        background: background.into(),
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: theme.input,
        }
        .into(),
    }
}
