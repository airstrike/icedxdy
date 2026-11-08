//! NativeSelect styling - nova style.
//!
//! `rounded-lg border border-input bg-transparent text-sm`, a muted
//! chevron, `dark:bg-input/30 dark:hover:bg-input/50`;
//! `aria-invalid:border-destructive` (`/50` in dark);
//! `disabled:cursor-not-allowed` with `opacity-50` on the wrapper.

use iced_widget::pick_list::{Status, Style, StyleFn};

use super::Theme;
use crate::{Border, Color, radius};

fn is_dark(theme: &Theme) -> bool {
    theme.neutral.base.l < 0.5
}

/// The default select field.
pub fn default(theme: &Theme, status: Status) -> Style {
    let input_a = theme.input.a;
    let background = match (is_dark(theme), status) {
        (true, Status::Hovered | Status::Opened { .. }) => theme.input.alpha(input_a * 0.5),
        (true, Status::Active | Status::Disabled) => theme.input.alpha(input_a * 0.3),
        (false, _) => Color::TRANSPARENT,
    };

    let style = Style {
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
    };

    match status {
        Status::Disabled => Style {
            text_color: style.text_color.scale_alpha(0.5),
            placeholder_color: style.placeholder_color.scale_alpha(0.5),
            handle_color: style.handle_color.scale_alpha(0.5),
            background: style.background.scale_alpha(0.5),
            border: iced_core::Border {
                color: style.border.color.scale_alpha(0.5),
                ..style.border
            },
        },
        Status::Active | Status::Hovered | Status::Opened { .. } => style,
    }
}

/// An invalid select field: a destructive border.
pub fn invalid(theme: &Theme, status: Status) -> Style {
    let style = default(theme, status);
    let border = if is_dark(theme) {
        theme.destructive.base.alpha(0.5)
    } else {
        theme.destructive.base
    };

    Style {
        border: iced_core::Border {
            color: border.into(),
            ..style.border
        },
        ..style
    }
}

/// Default select style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Invalid select style (`aria-invalid`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invalid;

impl<'a> From<Default> for StyleFn<'a, Theme> {
    fn from(_: Default) -> Self {
        Box::new(default)
    }
}

impl<'a> From<Invalid> for StyleFn<'a, Theme> {
    fn from(_: Invalid) -> Self {
        Box::new(invalid)
    }
}
