//! Radio styling - nova style
//!
//! Stock iced's radio widget has the right geometry for shadcn's
//! `RadioGroupItem` (16px circle, 8px indicator dot). This catalog bridge
//! maps the nova tokens onto iced's `radio::Style`.
//!
//! Nova classlist (verbatim from `.nova/apps/v4/styles/radix-nova/ui/radio-group.tsx`):
//!
//! ```text
//! group/radio-group-item peer relative flex aspect-square size-4 shrink-0
//! rounded-full border border-input outline-none after:absolute after:-inset-x-3
//! after:-inset-y-2 focus-visible:border-ring focus-visible:ring-3
//! focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50
//! aria-invalid:border-destructive aria-invalid:ring-3
//! aria-invalid:ring-destructive/20 aria-invalid:aria-checked:border-primary
//! dark:bg-input/30 dark:aria-invalid:border-destructive/50
//! dark:aria-invalid:ring-destructive/40 data-checked:border-primary
//! data-checked:bg-primary data-checked:text-primary-foreground
//! dark:data-checked:bg-primary
//! ```

pub use iced_widget::radio::Status;

use super::Theme;
use crate::Color;

impl iced_widget::radio::Catalog for Theme {
    type Class<'a> = iced_widget::radio::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> iced_widget::radio::Style {
        class(self, status)
    }

    fn selection(&self) -> iced_core::Color {
        self.selection()
    }
}

fn is_dark(theme: &Theme) -> bool {
    theme.neutral.base.l < 0.5
}

fn is_selected(status: Status) -> bool {
    match status {
        Status::Active { is_selected } | Status::Hovered { is_selected } => is_selected,
    }
}

/// Default radio variant: nova `size-4` control with a primary selected fill.
///
/// Light unchecked bg = transparent; dark unchecked bg = `palette.input` at
/// alpha 0.3 (`dark:bg-input/30`). Checked bg = primary; checked dot =
/// primary foreground; border at rest = input, selected = primary.
pub fn default(theme: &Theme, status: Status) -> iced_widget::radio::Style {
    let p = theme;
    let selected = is_selected(status);
    let background = if selected {
        p.primary.base
    } else if is_dark(theme) {
        p.input.alpha(0.3)
    } else {
        Color::TRANSPARENT
    };
    let border = if selected { p.primary.base } else { p.input };

    iced_widget::radio::Style {
        background: background.into(),
        dot_color: p.primary.foreground.into(),
        border_width: 1.0,
        border_color: border.into(),
        text_color: None,
    }
}
