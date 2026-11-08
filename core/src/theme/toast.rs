//! Toast (Sonner) styling - nova style.
//!
//! Sonner's toast with shadcn's variables: `--normal-bg: var(--popover)`,
//! `--normal-text: var(--popover-foreground)`, `--normal-border:
//! var(--border)`, nova's `rounded-2xl` and Sonner's
//! `box-shadow: 0 4px 12px #0000001a`. The action button inverts the
//! toast's colors.

use iced_core::Vector;

use super::Theme;
use super::button;
use super::container::Style;
use crate::{Border, Color, Shadow, radius};

/// A toast card.
pub fn card(theme: &Theme) -> Style {
    Style {
        background: Some(theme.popover.base.into()),
        text_color: Some(theme.popover.foreground),
        border: Border {
            radius: radius::XL2.into(),
            width: 1.0,
            color: theme.neutral.border,
        },
        shadow: Shadow {
            color: Color::BLACK.alpha(0.1),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        ..Style::default()
    }
}

/// A toast's action button: the toast's text color as its fill.
pub fn action(theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered | button::Status::Pressed => theme.popover.foreground.alpha(0.9),
        button::Status::Active | button::Status::Disabled => theme.popover.foreground,
    };

    button::Style {
        background: Some(background.into()),
        text_color: theme.popover.base,
        border: Border {
            radius: radius::XS.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}
