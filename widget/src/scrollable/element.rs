//! Catalog impl bridging our `Theme` into the vendored scrollable widget.
//! Mirrors the role of `widget/src/button/element.rs` and `widget/src/container/element.rs`.
//!
//! The cross-crate flow for `.style(...)`:
//!
//! 1. User calls `.style(theme::scrollable::Default)` (or any
//!    `From<X> for iced_widget::scrollable::StyleFn` impl in core).
//! 2. The wrapper in `widget/src/scrollable.rs` stores that as an
//!    `iced_widget::scrollable::StyleFn` so the existing core `From` impls
//!    flow through unchanged.
//! 3. At conversion time the wrapper composes a vendored `widget::StyleFn`
//!    that calls the iced StyleFn and runs the field-by-field
//!    `From<iced::scrollable::Style> for widget::Style` impl from
//!    `super::widget` to land it locally.
//!
//! This module owns the default Catalog impl used when the user calls no
//! `.style(...)` at all — it routes through the `default` fn in
//! `core::theme::scrollable` so the project-wide thin invisible-rail
//! default is the resolved style.

use icedxdy_core::Theme;
use icedxdy_core::theme::scrollable;

use super::widget;

fn default(theme: &Theme, status: widget::Status) -> widget::Style {
    // The `default` fn (from core's theme::scrollable) returns iced's
    // `Style`; convert to our vendored `widget::Style` via the
    // field-by-field `From` impl.
    let iced_status: iced_widget::scrollable::Status = status.into();
    let iced: iced_widget::scrollable::Style = scrollable::default(theme, iced_status);
    iced.into()
}

impl widget::Catalog for Theme {
    type Class<'a> = widget::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: widget::Status) -> widget::Style {
        class(self, status)
    }
}
