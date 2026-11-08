//! Toggle styling, shared by `Toggle` and `ToggleGroup` items.
//!
//! Nova `toggleVariants`: a transparent `rounded-lg` box with
//! `hover:bg-muted hover:text-foreground`, `disabled:opacity-50` and
//! `data-[state=on]:bg-muted`; `outline` adds `border border-input`. A toggle
//! is a button, so its styles are button [`Spec`]s and take the same tweaks.

use std::ops::Add;

use super::Theme;
use super::button::{Spec, Status, Style};
use super::tweak::Tweak;
use crate::{Border, Color, radius};

/// Default toggle: no fill at rest, muted on hover.
pub fn default(theme: &Theme, status: Status) -> Style {
    let base = Style {
        background: None,
        text_color: theme.neutral.foreground,
        border: Border {
            radius: radius::MD.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        ..Style::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered | Status::Pressed => Style {
            background: Some(theme.muted.base.into()),
            ..base
        },
        Status::Disabled => Style {
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}

/// Outline toggle: the default toggle inside a `border-input` stroke.
pub fn outline(theme: &Theme, status: Status) -> Style {
    let mut style = default(theme, status);
    let border = if status == Status::Disabled {
        theme.input.scale_alpha(0.5)
    } else {
        theme.input
    };

    style.border.width = 1.0;
    style.border.color = border;
    style
}

/// Paint `style` as pressed on (`data-[state=on]:bg-muted`).
pub fn on(theme: &Theme, style: Style) -> Style {
    Style {
        background: Some(theme.muted.base.into()),
        text_color: theme.neutral.foreground,
        ..style
    }
}

/// Default toggle variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Outline toggle variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outline;

impl From<Default> for Spec {
    fn from(_: Default) -> Spec {
        Spec::variant(default)
    }
}

impl From<Outline> for Spec {
    fn from(_: Outline) -> Spec {
        Spec::variant(outline)
    }
}

impl<T: Into<Tweak>> Add<T> for Default {
    type Output = Spec;

    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Outline {
    type Output = Spec;

    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}
