//! Scrollable styling

use iced_widget::container;
use iced_widget::scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style, StyleFn};
use iced_core::{Background, Border, Color, Shadow, Vector};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    let rail = Rail {
        background: None,
        border: Border::default(),
        scroller: Scroller {
            background: p.muted.base.into(),
            border: Border {
                radius: 8.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
    };

    let auto_scroll = AutoScroll {
        background: Background::Color(p.background.alpha(0.9).into()),
        border: Border {
            radius: (u32::MAX as f32).into(),
            width: 1.0,
            color: p.border.into(),
        },
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(0.3),
            offset: Vector::ZERO,
            blur_radius: 2.0,
        },
        icon: p.foreground.alpha(0.6).into(),
    };

    match status {
        Status::Active { .. } | Status::Hovered { .. } | Status::Dragged { .. } => Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
            auto_scroll,
        },
    }
}

pub fn hidden(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    let rail = Rail {
        background: None,
        border: Border::default(),
        scroller: Scroller {
            background: Color::TRANSPARENT.into(),
            border: Border::default(),
        },
    };

    let auto_scroll = AutoScroll {
        background: Background::Color(p.background.alpha(0.9).into()),
        border: Border {
            radius: (u32::MAX as f32).into(),
            width: 1.0,
            color: p.border.into(),
        },
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(0.3),
            offset: Vector::ZERO,
            blur_radius: 2.0,
        },
        icon: p.foreground.alpha(0.6).into(),
    };

    match status {
        Status::Active { .. } | Status::Hovered { .. } | Status::Dragged { .. } => Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
            auto_scroll,
        },
    }
}
