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
//! button::outline!["Outline ", icon::arrow_right()].on_press(Message::Nav)
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
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
            )
        )
    };
}

// Variant macros — each mirrors the `button!` two-arm shape but calls the
// matching variant fn from `helpers.rs` instead of `button()`. They're
// declared at crate root via `#[macro_export]` (Rust requires it for
// cross-crate visibility) and re-exported under the `button` module via
// alias-free `pub use crate::<name>` so users invoke them as
// `button::outline!(...)` etc. Macros and the variant fns coexist at the
// same path because they live in different namespaces.

/// Variant macro: `button::primary!(...)`. Mirrors [`button!`] for the
/// primary variant.
#[macro_export]
macro_rules! primary {
    ($single:expr $(,)?) => {
        $crate::button::primary($single)
    };
    ($($child:expr),+ $(,)?) => {
        $crate::button::primary(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
            )
        )
    };
}

/// Variant macro: `button::secondary!(...)`. Mirrors [`button!`] for the
/// secondary variant.
#[macro_export]
macro_rules! secondary {
    ($single:expr $(,)?) => {
        $crate::button::secondary($single)
    };
    ($($child:expr),+ $(,)?) => {
        $crate::button::secondary(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
            )
        )
    };
}

/// Variant macro: `button::outline!(...)`. Mirrors [`button!`] for the
/// outline variant.
#[macro_export]
macro_rules! outline {
    ($single:expr $(,)?) => {
        $crate::button::outline($single)
    };
    ($($child:expr),+ $(,)?) => {
        $crate::button::outline(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
            )
        )
    };
}

/// Variant macro: `button::ghost!(...)`. Mirrors [`button!`] for the ghost
/// variant.
#[macro_export]
macro_rules! ghost {
    ($single:expr $(,)?) => {
        $crate::button::ghost($single)
    };
    ($($child:expr),+ $(,)?) => {
        $crate::button::ghost(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
            )
        )
    };
}

/// Variant macro: `button::destructive!(...)`. Mirrors [`button!`] for the
/// destructive variant.
#[macro_export]
macro_rules! destructive {
    ($single:expr $(,)?) => {
        $crate::button::destructive($single)
    };
    ($($child:expr),+ $(,)?) => {
        $crate::button::destructive(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
            )
        )
    };
}

/// Variant macro: `button::icon!(...)`. Mirrors [`button!`] for the
/// icon-only square variant.
#[macro_export]
macro_rules! icon {
    ($single:expr $(,)?) => {
        $crate::button::icon($single)
    };
    ($($child:expr),+ $(,)?) => {
        $crate::button::icon(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
            )
        )
    };
}

#[doc(inline)]
pub use crate::primary;
#[doc(inline)]
pub use crate::secondary;
#[doc(inline)]
pub use crate::outline;
#[doc(inline)]
pub use crate::ghost;
#[doc(inline)]
pub use crate::destructive;
#[doc(inline)]
pub use crate::icon;

#[cfg(test)]
mod macro_smoke {
    //! Compile-only smoke tests: invoke each variant macro both single-arg
    //! and multi-arg, and verify the result is a `Button`.

    use crate::button::Button;

    #[derive(Clone)]
    enum Msg {}

    #[allow(dead_code)]
    fn assert_button<M: Clone>(_: &Button<'_, M>) {}

    #[test]
    fn variant_macros_compile() {
        let _: Button<'_, Msg> = crate::button!("default");
        let _: Button<'_, Msg> = crate::button!["a", "b"];

        let _: Button<'_, Msg> = crate::button::primary!("p");
        let _: Button<'_, Msg> = crate::button::primary!["p1", "p2"];

        let _: Button<'_, Msg> = crate::button::secondary!("s");
        let _: Button<'_, Msg> = crate::button::secondary!["s1", "s2"];

        let _: Button<'_, Msg> = crate::button::outline!("o");
        let _: Button<'_, Msg> = crate::button::outline!["o1", "o2"];

        let _: Button<'_, Msg> = crate::button::ghost!("g");
        let _: Button<'_, Msg> = crate::button::ghost!["g1", "g2"];

        let _: Button<'_, Msg> = crate::button::destructive!("d");
        let _: Button<'_, Msg> = crate::button::destructive!["d1", "d2"];

        let _: Button<'_, Msg> = crate::button::icon!("i");
        let _: Button<'_, Msg> = crate::button::icon!["i1", "i2"];
    }
}
