//! Alert container styling

use iced_core::Border;

use super::Theme;
use crate::radius;

pub use iced_widget::container::Style;

/// Default alert style: card background, standard border, rounded-lg, card-foreground text
pub fn default(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.card.base.into()),
        text_color: Some(p.card.foreground.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.border.into(),
        },
        ..Default::default()
    }
}

/// Destructive alert style: card background, destructive border, destructive text
pub fn destructive(theme: &Theme) -> Style {
    let p = theme.palette();
    let is_dark = p.background.l < 0.5;
    let border_color = if is_dark {
        p.destructive.base
    } else {
        p.destructive.base.alpha(0.5)
    };
    Style {
        background: Some(p.card.base.into()),
        text_color: Some(p.destructive.base.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: border_color.into(),
        },
        ..Default::default()
    }
}
