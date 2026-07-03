//! Catalog bridge for the vendored slider widget.

use icedxdy_core::Theme;
use icedxdy_core::theme::slider;

use super::widget;

fn from_core(
    s: iced_widget::slider::Style,
    theme: &Theme,
    status: widget::Status,
) -> widget::Style {
    let mut style = widget::Style::from(s);

    style.ring = match status {
        widget::Status::Hovered | widget::Status::Dragged => Some(widget::Ring {
            color: theme.neutral.ring.alpha(0.5).into(),
            width: 3.0,
        }),
        widget::Status::Active => None,
    };

    style
}

fn default(theme: &Theme, status: widget::Status) -> widget::Style {
    let iced_status = match status {
        widget::Status::Active => slider::Status::Active,
        widget::Status::Hovered => slider::Status::Hovered,
        widget::Status::Dragged => slider::Status::Dragged,
    };

    from_core(slider::default(theme, iced_status), theme, status)
}

impl widget::Catalog for Theme {
    type Class<'a> = widget::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: widget::Status) -> widget::Style {
        class(self, status)
    }
}
