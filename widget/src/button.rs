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
//! button::outline!("Outline").on_press(Message::Nav)
//! button::outline!["Outline ", icon::arrow_right()].on_press(Message::Nav)
//! button::icon![arrow_up].rounded()
//! ```

mod element;
pub mod group;
mod helpers;
pub mod widget;

pub use element::{Button, Link, button};
pub use group::ButtonGroup;
pub use helpers::*;

/// Create a button with one or more children using the default variant.
///
/// Two forms:
/// - `button!("Submit")` — single child, defers to [`button`].
/// - `button![child1, child2, ...]` — multiple children wrapped in a row
///   and passed to [`button`]. Spacing comes from size propagation, not
///   from the macro itself.
///
/// For non-default variants, use the per-variant macros at the module
/// path: [`button::primary!`], [`button::secondary!`], [`button::outline!`],
/// [`button::ghost!`], [`button::destructive!`], [`button::icon!`]. They
/// share the same single-arg / multi-arg arms as `button!`.
///
/// ```ignore
/// button!["Submit"]
/// button![icon::mail(), "Login with Email"]
/// button::outline![icon::git_branch(), "Branch"]
/// button::destructive!("Delete")
/// button::icon![arrow_up].rounded()
/// button![arrow_up, "Submit"]                  // leading-ident sugar
/// button::outline![git_branch, "Branch"]       // leading-ident sugar on variant
/// ```
#[macro_export]
macro_rules! button {
    // Default-variant single-arg arm
    ($single:expr $(,)?) => {
        $crate::button::button($single)
    };
    // Leading-ident icon sugar: the first arg as a bare ident is rewritten
    // to `icon::$first()`. Requires `use crate::icon;` (or equivalent path
    // to an `icon` module exposing `fn $first() -> Text`) in scope at the
    // call site. Macro hygiene: `icon` here resolves at the call site, not
    // the def site — matches `button_icon!`'s existing single-arg arm.
    //
    // `button![arrow_up, "Submit"]` => button(row![icon::arrow_up(), "Submit"])
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::button::button(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
            )
        )
    };
    // Default-variant multi-arg arm — wraps children in a row.
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

/// Primary-variant button macro. See [`button!`] for the arm shapes.
#[macro_export]
macro_rules! button_primary {
    ($single:expr $(,)?) => {
        $crate::button::primary($single)
    };
    // Leading-ident icon sugar — see [`button!`] for details.
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::button::primary(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
            )
        )
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

/// Secondary-variant button macro. See [`button!`] for the arm shapes.
#[macro_export]
macro_rules! button_secondary {
    ($single:expr $(,)?) => {
        $crate::button::secondary($single)
    };
    // Leading-ident icon sugar — see [`button!`] for details.
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::button::secondary(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
            )
        )
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

/// Outline-variant button macro. See [`button!`] for the arm shapes.
#[macro_export]
macro_rules! button_outline {
    ($single:expr $(,)?) => {
        $crate::button::outline($single)
    };
    // Leading-ident icon sugar — see [`button!`] for details.
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::button::outline(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
            )
        )
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

/// Ghost-variant button macro. See [`button!`] for the arm shapes.
#[macro_export]
macro_rules! button_ghost {
    ($single:expr $(,)?) => {
        $crate::button::ghost($single)
    };
    // Leading-ident icon sugar — see [`button!`] for details.
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::button::ghost(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
            )
        )
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

/// Destructive-variant button macro. See [`button!`] for the arm shapes.
#[macro_export]
macro_rules! button_destructive {
    ($single:expr $(,)?) => {
        $crate::button::destructive($single)
    };
    // Leading-ident icon sugar — see [`button!`] for details.
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::button::destructive(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
            )
        )
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

/// Icon-only button macro.
///
/// Three forms (in match order):
/// - `button::icon![arrow_up]` — bare ident. Calls `icon::$name()` at the
///   call site (so `use crate::icon;` must be in scope), then passes the
///   resulting widget to [`icon`]. The convention every example file
///   already follows.
/// - `button::icon![icon::arrow_up]` — explicit path. Invokes `$f()` and
///   passes the result to [`icon`]. Useful when `icon` is not in scope or
///   when the user wants to reach for a custom icon module.
/// - `button::icon![child1, child2, ...]` — multi-child fallback: wraps
///   the children in a row and passes them to [`icon`].
#[macro_export]
macro_rules! button_icon {
    // Bare ident — assumes `icon::$name` is in scope at the call site.
    // Placed first so a single ident matches here before the path arm.
    ($name:ident $(,)?) => {
        $crate::button::icon(icon::$name())
    };
    // Explicit path — module-qualified function reference.
    ($f:path $(,)?) => {
        $crate::button::icon($f())
    };
    // Multi-child fallback.
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

// `#[macro_export]` always lands at the consuming crate's root, so two
// widgets can't both claim a name like `secondary!` there. The fix: the
// real macros above are exported with a `button_` prefix, then we re-alias
// each to its idiomatic name at the module path (`button::secondary!`,
// `button::outline!`, ...) right here. This is the carved exception in
// CLAUDE.md hard rule #3 — the file-based exemption in the pre-commit
// hook permits these two aliased re-exports (and matching ones in
// `widget/src/badge.rs`). iced uses the same trick for `column!` /
// `column()` coexisting in the `column` module.
#[doc(inline)]
pub use crate::button_destructive as destructive;
#[doc(inline)]
pub use crate::button_ghost as ghost;
#[doc(inline)]
pub use crate::button_group as group;
#[doc(inline)]
pub use crate::button_icon as icon;
#[doc(inline)]
pub use crate::button_outline as outline;
#[doc(inline)]
pub use crate::button_primary as primary;
#[doc(inline)]
pub use crate::button_secondary as secondary;

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

        // `button::icon!` multi-child fallback.
        let _: Button<'_, Msg> = crate::button::icon!["i1", "i2"];
    }

    /// Path-arm + bare-ident smoke test: `button::icon![path::Foo]`
    /// expands to `button::icon(path::Foo())`; `button::icon![bare]`
    /// expands to `button::icon(icon::bare())` (ident arm), provided
    /// `icon` is in scope at the call site.
    #[test]
    fn icon_macro_path_arm() {
        // Local `icon` module mirroring the real `icon::*` helper
        // signature in `examples/browser/src/icon.rs`. The bare-ident
        // arm of `button::icon!` looks up `icon::$name()` at the call
        // site, so this module name is load-bearing.
        mod icon {
            pub fn arrow_up<'a, T>() -> iced_widget::Text<'a, T>
            where
                T: iced_widget::text::Catalog,
            {
                iced_widget::Text::new("")
            }
        }

        // Bare ident — resolves to `icon::arrow_up()` via the in-scope
        // `icon` module above.
        let _: Button<'_, Msg> = crate::button::icon![arrow_up];

        // Module-qualified path also works (path arm).
        mod inner {
            pub fn ic<'a, T>() -> iced_widget::Text<'a, T>
            where
                T: iced_widget::text::Catalog,
            {
                iced_widget::Text::new("")
            }
        }
        let _: Button<'_, Msg> = crate::button::icon![inner::ic];
    }

    /// Leading-ident sugar: `button![my_icon, "Submit"]` rewrites
    /// the bare ident to `icon::my_icon()` at the call site, then
    /// prepends it to the row of remaining children. Same arm exists
    /// on every variant macro except `button::icon!`.
    #[test]
    fn leading_ident_sugar_arm() {
        // Local `icon` module — name load-bearing for the same reason
        // as `icon_macro_path_arm` above.
        mod icon {
            pub fn my_icon<'a, T>() -> iced_widget::Text<'a, T>
            where
                T: iced_widget::text::Catalog,
            {
                iced_widget::Text::new("")
            }
        }

        let _: Button<'_, Msg> = crate::button![my_icon, "Submit"];
        let _: Button<'_, Msg> = crate::button::primary![my_icon, "Click"];
        let _: Button<'_, Msg> = crate::button::secondary![my_icon, "Click"];
        let _: Button<'_, Msg> = crate::button::outline![my_icon, "Click"];
        let _: Button<'_, Msg> = crate::button::ghost![my_icon, "Click"];
        let _: Button<'_, Msg> = crate::button::destructive![my_icon, "Click"];
    }
}
