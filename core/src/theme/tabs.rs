//! Tabs styling — nova style
//!
//! - [`list`]: the `TabsList` pill, `bg-muted rounded-lg p-[3px]`.
//! - [`trigger`]: a `TabsTrigger`. Inactive triggers are transparent with
//!   `text-foreground/60` (dark: `text-muted-foreground`) and
//!   `hover:text-foreground`; the active one takes `bg-background
//!   text-foreground shadow-sm` (dark: `border-input bg-input/30`).

use iced_core::Vector;

use super::Theme;
use super::button;
use super::container;
use crate::{Border, Color, Shadow, radius};

/// The `TabsList` pill behind the triggers.
pub fn list(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(theme.muted.base.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..container::Style::default()
    }
}

/// A `TabsTrigger`, active when its value is the selected one.
pub fn trigger(theme: &Theme, status: button::Status, is_active: bool) -> button::Style {
    let p = theme;
    let dark = p.neutral.is_dark;

    let idle_text = if dark {
        p.muted.foreground
    } else {
        p.neutral.foreground.alpha(0.6)
    };

    let text_color = match status {
        _ if is_active => p.neutral.foreground,
        button::Status::Hovered | button::Status::Pressed => p.neutral.foreground,
        button::Status::Active => idle_text,
        button::Status::Disabled => idle_text.alpha(0.5),
    };

    if !is_active {
        return button::Style {
            background: None,
            text_color,
            border: Border {
                radius: radius::MD.into(),
                width: 1.0,
                color: Color::TRANSPARENT,
            },
            ..button::Style::default()
        };
    }

    let (background, border_color) = if dark {
        (p.input.alpha(p.input.a * 0.3), p.input)
    } else {
        (p.neutral.base, Color::TRANSPARENT)
    };

    button::Style {
        background: Some(background.into()),
        text_color,
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: border_color,
        },
        shadow: Shadow {
            color: Color::BLACK.alpha(0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
        ..button::Style::default()
    }
}
