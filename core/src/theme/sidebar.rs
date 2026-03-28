//! Sidebar-specific styles
//!
//! Uses the sidebar sub-palette for menu buttons, separators, labels, and containers.

use iced_core::{Border, Shadow, Vector};
use iced_widget::button::Status;
use iced_widget::{button, container, rule, text};

use super::Theme;
use crate::radius;

// ---------------------------------------------------------------------------
// Menu button styles (for iced_widget::button)
// ---------------------------------------------------------------------------

/// Default sidebar menu button: no bg at rest, accent on hover
pub fn menu_button(theme: &Theme, status: Status) -> button::Style {
    let sb = &theme.palette().sidebar;

    let base = button::Style {
        background: None,
        text_color: sb.foreground.into(),
        border: Border {
            radius: radius::SM.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered | Status::Pressed => button::Style {
            background: Some(sb.accent.base.into()),
            text_color: sb.accent.foreground.into(),
            ..base
        },
        Status::Disabled => button::Style {
            text_color: sb.foreground.alpha(0.5).into(),
            ..base
        },
    }
}

/// Active sidebar menu button: accent bg always
pub fn menu_button_active(theme: &Theme, status: Status) -> button::Style {
    let sb = &theme.palette().sidebar;

    let base = button::Style {
        background: Some(sb.accent.base.into()),
        text_color: sb.accent.foreground.into(),
        border: Border {
            radius: radius::SM.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        Status::Active | Status::Hovered | Status::Pressed => base,
        Status::Disabled => button::Style {
            background: Some(sb.accent.base.alpha(0.5).into()),
            text_color: sb.accent.foreground.alpha(0.5).into(),
            ..base
        },
    }
}

/// Outline sidebar menu button: bordered, accent on hover
pub fn menu_button_outline(theme: &Theme, status: Status) -> button::Style {
    let sb = &theme.palette().sidebar;

    let base = button::Style {
        background: Some(sb.background.into()),
        text_color: sb.foreground.into(),
        border: Border {
            radius: radius::SM.into(),
            width: 1.0,
            color: sb.border.into(),
        },
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered | Status::Pressed => button::Style {
            background: Some(sb.accent.base.into()),
            text_color: sb.accent.foreground.into(),
            ..base
        },
        Status::Disabled => button::Style {
            background: Some(sb.background.alpha(0.5).into()),
            text_color: sb.foreground.alpha(0.5).into(),
            border: Border {
                color: sb.border.alpha(0.5).into(),
                ..base.border
            },
            ..base
        },
    }
}

/// Outline sidebar menu button (active): bordered + accent bg always
pub fn menu_button_outline_active(theme: &Theme, status: Status) -> button::Style {
    let sb = &theme.palette().sidebar;

    let base = button::Style {
        background: Some(sb.accent.base.into()),
        text_color: sb.accent.foreground.into(),
        border: Border {
            radius: radius::SM.into(),
            width: 1.0,
            color: sb.border.into(),
        },
        ..Default::default()
    };

    match status {
        Status::Active | Status::Hovered | Status::Pressed => base,
        Status::Disabled => button::Style {
            background: Some(sb.accent.base.alpha(0.5).into()),
            text_color: sb.accent.foreground.alpha(0.5).into(),
            border: Border {
                color: sb.border.alpha(0.5).into(),
                ..base.border
            },
            ..base
        },
    }
}

// ---------------------------------------------------------------------------
// Rule (separator)
// ---------------------------------------------------------------------------

/// Sidebar separator using sidebar-border color
pub fn separator(theme: &Theme) -> rule::Style {
    rule::Style {
        color: theme.palette().sidebar.border.into(),
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    }
}

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

/// Sidebar group label: foreground at 70% opacity
pub fn group_label(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.palette().sidebar.foreground.alpha(0.7).into()),
    }
}

// ---------------------------------------------------------------------------
// Container
// ---------------------------------------------------------------------------

/// Sidebar background container
pub fn sidebar_background(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(theme.palette().sidebar.background.into()),
        ..Default::default()
    }
}

/// Sidebar floating container: background, border, shadow, rounded corners
pub fn sidebar_floating(theme: &Theme) -> container::Style {
    let sb = &theme.palette().sidebar;

    container::Style {
        background: Some(sb.background.into()),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: sb.border.into(),
        },
        shadow: Shadow {
            color: iced_core::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}
