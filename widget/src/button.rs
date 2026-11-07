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
//! button("Outline").style(button::Outline).on_press(Message::Nav)
//! button!["Outline ", icon::arrow_right()].style(button::Outline)
//! button(icon::arrow_up()).square().rounded().style(button::Outline)
//! ```
//!
//! # Variants
//!
//! shadcn's `variant` prop is a style: `.style(button::Outline)`. The
//! variant unit structs below are re-exported from the theme and take
//! tweaks (`button::Outline + Bg(...)`) like any other style.

mod element;
pub mod group;
pub mod widget;

pub use element::{Button, button};
pub use group::ButtonGroup;
pub use icedxdy_core::theme::button::{Default, Destructive, Ghost, Link, Outline, Secondary};

/// Create a button with one or more children.
///
/// Two forms:
/// - `button!("Submit")` — single child, defers to [`button`].
/// - `button![child1, child2, ...]` — multiple children wrapped in a row
///   and passed to [`button`]. Spacing comes from size propagation, not
///   from the macro itself.
///
/// The variant is a style, set after the macro:
///
/// ```ignore
/// button!["Submit"]
/// button![icon::mail(), "Login with Email"]
/// button![icon::git_branch(), "Branch"].style(button::Outline)
/// button!("Delete").style(button::Destructive)
/// button![arrow_up, "Submit"]                  // leading-ident sugar
/// ```
#[macro_export]
macro_rules! button {
    // Single-arg arm
    ($single:expr $(,)?) => {
        $crate::button::button($single)
    };
    // Leading-ident icon sugar: the first arg as a bare ident is rewritten
    // to `icon::$first()`. Requires `use crate::icon;` (or equivalent path
    // to an `icon` module exposing `fn $first() -> Text`) in scope at the
    // call site. Macro hygiene: `icon` here resolves at the call site, not
    // the def site.
    //
    // `button![arrow_up, "Submit"]` => button(row![icon::arrow_up(), "Submit"])
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::button::button(
            $crate::compose::Slot::new(
                $crate::row::Row::with_children([
                    Into::<$crate::compose::Slot<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::compose::Slot<'_, _>>::into($rest)),+
                ])
            )
        )
    };
    // Multi-arg arm — wraps children in a row.
    ($($child:expr),+ $(,)?) => {
        $crate::button::button(
            $crate::compose::Slot::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::compose::Slot<'_, _>>::into($child)),+
                ])
            )
        )
    };
}

#[cfg(test)]
mod macro_smoke {
    //! Compile-only smoke tests: invoke the macro single-arg, multi-arg and
    //! with the leading-ident sugar, and verify the result is a `Button`.

    use crate::button::{self, Button};

    #[derive(Clone)]
    enum Msg {}

    #[test]
    fn macro_arms_compile() {
        let _: Button<'_, Msg> = crate::button!("default");
        let _: Button<'_, Msg> = crate::button!["a", "b"].style(button::Outline);
        let _: Button<'_, Msg> = crate::button!("d").style(button::Destructive);
    }

    /// Leading-ident sugar: `button![my_icon, "Submit"]` rewrites the bare
    /// ident to `icon::my_icon()` at the call site, then prepends it to the
    /// row of remaining children.
    #[test]
    fn leading_ident_sugar_arm() {
        // Local `icon` module — the sugar arm expands to `icon::$first()`
        // resolved at the call site, so this module name is load-bearing.
        mod icon {
            pub fn my_icon<'a, T>() -> iced_widget::Text<'a, T>
            where
                T: iced_widget::text::Catalog,
            {
                iced_widget::Text::new("")
            }
        }

        let _: Button<'_, Msg> = crate::button![my_icon, "Submit"];
        let _: Button<'_, Msg> = crate::button![my_icon, "Click"].style(button::Ghost);
    }
}
