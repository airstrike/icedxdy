//! Textarea styling - nova style.
//!
//! A textarea paints like an Input (`rounded-lg border border-input`,
//! `focus-visible:border-ring`, `disabled:opacity-50`, `aria-invalid`), so
//! its styles are the Input's, projected onto iced's `text_editor::Style`.
//! The Input's focus halo has no counterpart in `text_editor::Style`.

use iced_widget::text_editor::{Catalog, Status, Style, StyleFn};

use super::Theme;
use super::text_input;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn input_status(status: Status) -> text_input::Status {
    match status {
        Status::Active => text_input::Status::Active,
        Status::Hovered => text_input::Status::Hovered,
        Status::Focused { is_hovered } => text_input::Status::Focused { is_hovered },
        Status::Disabled => text_input::Status::Disabled,
    }
}

fn project(style: text_input::Style) -> Style {
    Style {
        background: style.background.into(),
        border: style.border.into(),
        placeholder: style.placeholder.into(),
        value: style.value.into(),
        selection: style.selection.into(),
    }
}

/// The default textarea.
pub fn default(theme: &Theme, status: Status) -> Style {
    project(text_input::default(theme, input_status(status)))
}

/// An invalid textarea (`aria-invalid`): a destructive border.
pub fn invalid(theme: &Theme, status: Status) -> Style {
    project(text_input::invalid(theme, input_status(status)))
}

/// Default textarea style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Invalid textarea style (`aria-invalid`).
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
