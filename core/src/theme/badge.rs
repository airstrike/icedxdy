//! Badge styling
//!
//! Style functions for badge variants, returning container styles.

use iced_core::Border;
use iced_widget::container::Style;

use super::Theme;

const PILL_RADIUS: f32 = u32::MAX as f32;

/// Default: bg-primary text-primary-foreground
pub fn default(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.primary.base.into()),
        text_color: Some(p.primary.foreground.into()),
        border: Border {
            radius: PILL_RADIUS.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Secondary: bg-secondary text-secondary-foreground
pub fn secondary(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.secondary.base.into()),
        text_color: Some(p.secondary.foreground.into()),
        border: Border {
            radius: PILL_RADIUS.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Destructive: bg-destructive text-destructive-foreground
pub fn destructive(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: Some(p.destructive.base.into()),
        text_color: Some(p.destructive.foreground.into()),
        border: Border {
            radius: PILL_RADIUS.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Outline: bg-transparent text-foreground border-1px
pub fn outline(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: None,
        text_color: Some(p.foreground.into()),
        border: Border {
            radius: PILL_RADIUS.into(),
            width: 1.0,
            color: p.border.into(),
        },
        ..Default::default()
    }
}
