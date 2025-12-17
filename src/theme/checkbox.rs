//! Checkbox styling

pub use iced_widget::checkbox::{Catalog, Status, Style, StyleFn};

use iced_core::Border;

use super::Theme;
use icedxdy_core::radius;

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

    let base = Style {
        background: p.background.into(),
        icon_color: p.primary.foreground.into(),
        border: Border {
            radius: radius::SM.into(),
            width: 1.0,
            color: p.border.into(),
        },
        text_color: None,
    };

    match status {
        Status::Active { is_checked } => {
            if is_checked {
                Style {
                    background: p.primary.base.into(),
                    border: Border {
                        color: p.primary.base.into(),
                        ..base.border
                    },
                    ..base
                }
            } else {
                base
            }
        }
        Status::Hovered { is_checked } => {
            if is_checked {
                Style {
                    background: p.primary.base.alpha(0.9).into(),
                    border: Border {
                        color: p.primary.base.alpha(0.9).into(),
                        ..base.border
                    },
                    ..base
                }
            } else {
                Style {
                    background: p.accent.base.into(),
                    ..base
                }
            }
        }
        Status::Disabled { is_checked } => Style {
            background: if is_checked {
                p.primary.base.alpha(0.5).into()
            } else {
                p.muted.base.into()
            },
            icon_color: p.primary.foreground.alpha(0.5).into(),
            border: Border {
                color: p.border.alpha(0.5).into(),
                ..base.border
            },
            text_color: Some(p.muted.foreground.into()),
        },
    }
}
