//! Widget-agnostic focus operations.
//!
//! This module provides sweetened versions of focus operations that return
//! the [`widget::Id`] of the newly focused widget.
//!
//! # Example
//! ```no_run
//! # use iced_core::widget;
//! use icedxdy_widget::operation;
//!
//! # #[derive(Debug, Clone)]
//! # enum Message { FocusedId(widget::Id), FocusNext }
//! # fn update(message: Message) -> iced_runtime::Task<Message> {
//! // Focus the next widget and get its ID:
//! operation::focus_next().map(Message::FocusedId)
//! # }
//! ```

pub mod position;
pub mod scroll;

use crate::core::widget;
use crate::core::widget::operation::focusable;
use iced_runtime::Task;
use iced_runtime::task;
use iced_runtime::widget::operation;

/// Produces a [`Task`] that focuses the next focusable widget
/// and returns the [`widget::Id`] of the newly focused widget.
///
/// This is a sweetened version of iced_runtime's `focus_next` that tells
/// you which widget received focus.
///
/// Use `.discard()` if you don't need the ID, or `.map(|id| ...)` to use it.
pub fn focus_next() -> Task<widget::Id> {
    operation::focus_next().chain(task::widget(focusable::find_focused()))
}

/// Produces a [`Task`] that focuses the previous focusable widget
/// and returns the [`widget::Id`] of the newly focused widget.
///
/// This is a sweetened version of iced_runtime's `focus_previous` that
/// tells you which widget received focus.
///
/// Use `.discard()` if you don't need the ID, or `.map(|id| ...)` to use it.
pub fn focus_previous() -> Task<widget::Id> {
    operation::focus_previous().chain(task::widget(focusable::find_focused()))
}

/// Produces a [`Task`] that focuses the widget with the given [`widget::Id`].
///
/// Re-exported from iced_runtime.
pub use operation::focus;
