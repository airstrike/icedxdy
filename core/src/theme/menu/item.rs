//! Menu item styling: nova style.
//!
//! `rounded-md px-1.5 py-1 text-sm focus:bg-accent
//! focus:text-accent-foreground data-disabled:opacity-50`, and the
//! destructive variant's `text-destructive focus:bg-destructive/10`
//! (`dark:focus:bg-destructive/20`). Shared by dropdown, context and select
//! menus.

use iced_widget::button::{Status, Style};

use crate::theme::Theme;
use crate::{Border, radius};

/// The look of a menu item: `.style(menu::Destructive)` on an item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// A plain item.
    #[default]
    Default,
    /// A destructive action, in the destructive color.
    Destructive,
}

/// A destructive menu item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Destructive;

impl From<Destructive> for Variant {
    fn from(_: Destructive) -> Self {
        Variant::Destructive
    }
}

/// A menu item's surface for its status.
pub fn style(theme: &Theme, status: Status, variant: Variant) -> Style {
    let (text, focused_bg, focused_text) = match variant {
        Variant::Default => (
            theme.popover.foreground,
            theme.accent.base,
            theme.accent.foreground,
        ),
        Variant::Destructive => (
            theme.destructive.base,
            theme
                .destructive
                .base
                .alpha(if theme.neutral.is_dark { 0.2 } else { 0.1 }),
            theme.destructive.base,
        ),
    };

    let base = Style {
        background: None,
        text_color: text.into(),
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        }
        .into(),
        ..Style::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered | Status::Pressed => Style {
            background: Some(focused_bg.into()),
            text_color: focused_text.into(),
            ..base
        },
        Status::Disabled => Style {
            text_color: text.alpha(0.5).into(),
            ..base
        },
    }
}
