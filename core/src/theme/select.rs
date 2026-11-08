//! Select trigger styling: nova style.
//!
//! `SelectTrigger`: `rounded-lg border border-input bg-transparent text-sm`,
//! `dark:bg-input/30 dark:hover:bg-input/50`, `disabled:opacity-50`, and
//! `aria-invalid:border-destructive` (`dark:aria-invalid:border-destructive/50`).
//! Its menu uses [`popover::content`](super::popover::content) and the
//! [`menu::item`](super::menu::item) rows.

use iced_widget::button::{Status, Style};

use super::Theme;
use crate::{Border, Color, radius};

/// The look of a select trigger: `.style(select::Invalid)` on a select.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// A plain trigger.
    #[default]
    Default,
    /// An invalid field (`aria-invalid`): a destructive border.
    Invalid,
}

/// An invalid select.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Invalid;

impl From<Invalid> for Variant {
    fn from(_: Invalid) -> Self {
        Variant::Invalid
    }
}

/// A select trigger's surface for its status.
pub fn trigger(theme: &Theme, status: Status, variant: Variant) -> Style {
    let is_dark = theme.neutral.base.l < 0.5;
    let input_a = theme.input.a;

    let background = match (is_dark, status) {
        (true, Status::Hovered | Status::Pressed) => theme.input.alpha(input_a * 0.5),
        (true, Status::Active | Status::Disabled) => theme.input.alpha(input_a * 0.3),
        (false, _) => Color::TRANSPARENT,
    };

    let border = match (variant, is_dark) {
        (Variant::Default, _) => theme.input,
        (Variant::Invalid, false) => theme.destructive.base,
        (Variant::Invalid, true) => theme.destructive.base.alpha(0.5),
    };

    let style = Style {
        background: Some(background.into()),
        text_color: theme.neutral.foreground.into(),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: border,
        }
        .into(),
        ..Style::default()
    };

    match status {
        Status::Disabled => Style {
            text_color: style.text_color.scale_alpha(0.5),
            border: iced_core::Border {
                color: style.border.color.scale_alpha(0.5),
                ..style.border
            },
            ..style
        },
        Status::Active | Status::Hovered | Status::Pressed => style,
    }
}
