//! Text input styling — shadcn/ui faithful port
//!
//! CSS: flex h-9 w-full rounded-md border border-input bg-transparent
//!      px-3 py-1 text-base shadow-sm transition-colors
//!      placeholder:text-muted-foreground
//!      focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring
//!      disabled:cursor-not-allowed disabled:opacity-50
//!      md:text-sm

pub use iced_widget::text_input::{Catalog, Status, Style, StyleFn};

use iced_core::Border;

use super::Theme;
use crate::radius;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// Default text input style matching shadcn's input component.
pub fn default(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    let active = Style {
        background: p.background.into(),
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: p.input.into(),
        },
        icon: p.foreground.into(),
        placeholder: p.muted.foreground.into(),
        value: p.foreground.into(),
        selection: p.primary.base.alpha(0.3).into(),
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: Border {
                color: p.ring.alpha(0.5).into(),
                ..active.border
            },
            ..active
        },
        Status::Focused { .. } => Style {
            border: Border {
                color: p.ring.into(),
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            background: p.muted.base.alpha(0.5).into(),
            border: Border {
                color: p.input.alpha(0.5).into(),
                ..active.border
            },
            value: p.foreground.alpha(0.5).into(),
            placeholder: p.muted.foreground.alpha(0.5).into(),
            icon: p.foreground.alpha(0.5).into(),
            ..active
        },
    }
}
