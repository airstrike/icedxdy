//! Button components
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::button;
//!
//! button("Submit").on_press(Message::Submit)
//! button("Go").size(Size::Lg).on_press(Message::Go)
//! button![icon::git_branch(), "New Branch"].on_press(Message::Go)
//! button::outline("Outline").on_press(Message::Nav)
//! ```

mod element;
mod helpers;
pub mod widget;

pub use element::{button, Button, Link};
pub use helpers::*;

/// Create a button with one or more children.
///
/// With a single argument, behaves like `button(content)`.
/// With multiple arguments, lays them out in a centered row with spacing.
///
/// ```ignore
/// button!["Submit"]
/// button![icon::mail(), "Login with Email"]
/// button![icon::loader_circle(), "Generating"].on_press(Message::Go)
/// ```
#[macro_export]
macro_rules! button {
    ($single:expr $(,)?) => {
        $crate::button::button($single)
    };
    ($($child:expr),+ $(,)?) => {
        $crate::button::button(
            $crate::element::Element::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::element::Element<'_, _>>::into($child)),+
                ])
            )
        )
    };
}
