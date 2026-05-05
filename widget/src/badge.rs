//! Badge component — small inline pill label
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::badge;
//!
//! badge("Badge")
//! badge::secondary("Secondary")
//! badge::destructive("Destructive")
//! badge::outline("Outline")
//! badge::ghost("Ghost")
//! badge::secondary("Open Link").on_press(Message::Open)
//!
//! // Macro form: single-arg defers to the default variant fn; multi-arg
//! // wraps children in a `row::Row::with_children([...]).spacing(4)` to
//! // land the nova `gap-1` (4px) icon-text gap automatically.
//! badge!["Verified", icon::badge_check()]
//! badge::secondary!["Verified", icon::badge_check()]
//! badge::outline!["Bookmark", icon::bookmark()]
//! ```

/// Create a badge with one or more children using the default variant.
///
/// Two forms:
/// - `badge!("Badge")` — single child, defers to [`badge`].
/// - `badge![child1, child2, ...]` — multiple children, wrapped in a
///   row with `spacing(4)` (nova `gap-1`) and passed to [`badge`].
///
/// For non-default variants, use the per-variant macros at the module
/// path: [`badge::secondary!`], [`badge::outline!`], [`badge::destructive!`],
/// [`badge::ghost!`]. They share the same single-arg / multi-arg arms.
///
/// ```ignore
/// badge!["Verified", icon::badge_check()]
/// badge::secondary!["Verified", icon::badge_check()]
/// badge::outline![icon::bookmark(), "Bookmark"]
/// badge::destructive![spinner(), "Deleting"]
/// badge![badge_check, "Verified"]                    // leading-ident sugar
/// badge::secondary![badge_check, "Verified"]         // leading-ident sugar on variant
/// ```
#[macro_export]
macro_rules! badge {
    // Default-variant single-arg arm
    ($single:expr $(,)?) => {
        $crate::badge::badge($single)
    };
    // Leading-ident icon sugar: the first arg as a bare ident is rewritten
    // to `icon::$first()`. Requires `use crate::icon;` (or equivalent path
    // to an `icon` module exposing `fn $first() -> Text`) in scope at the
    // call site. Macro hygiene: `icon` here resolves at the call site, not
    // the def site — matches `button_icon!`'s existing single-arg arm.
    //
    // `badge![badge_check, "Verified"]` => badge(row![icon::badge_check(), "Verified"].spacing(4))
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::badge::badge(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
                .spacing(4)
            )
        )
    };
    // Default-variant multi-arg arm — wraps children in a row with 4px gap.
    ($($child:expr),+ $(,)?) => {
        $crate::badge::badge(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
                .spacing(4)
            )
        )
    };
}

/// Secondary-variant badge macro. See [`badge!`] for the arm shapes.
#[macro_export]
macro_rules! badge_secondary {
    ($single:expr $(,)?) => {
        $crate::badge::secondary($single)
    };
    // Leading-ident icon sugar — see [`badge!`] for details.
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::badge::secondary(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
                .spacing(4)
            )
        )
    };
    ($($child:expr),+ $(,)?) => {
        $crate::badge::secondary(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
                .spacing(4)
            )
        )
    };
}

/// Outline-variant badge macro. See [`badge!`] for the arm shapes.
#[macro_export]
macro_rules! badge_outline {
    ($single:expr $(,)?) => {
        $crate::badge::outline($single)
    };
    // Leading-ident icon sugar — see [`badge!`] for details.
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::badge::outline(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
                .spacing(4)
            )
        )
    };
    ($($child:expr),+ $(,)?) => {
        $crate::badge::outline(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
                .spacing(4)
            )
        )
    };
}

/// Destructive-variant badge macro. See [`badge!`] for the arm shapes.
#[macro_export]
macro_rules! badge_destructive {
    ($single:expr $(,)?) => {
        $crate::badge::destructive($single)
    };
    // Leading-ident icon sugar — see [`badge!`] for details.
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::badge::destructive(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
                .spacing(4)
            )
        )
    };
    ($($child:expr),+ $(,)?) => {
        $crate::badge::destructive(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
                .spacing(4)
            )
        )
    };
}

/// Ghost-variant badge macro. See [`badge!`] for the arm shapes.
#[macro_export]
macro_rules! badge_ghost {
    ($single:expr $(,)?) => {
        $crate::badge::ghost($single)
    };
    // Leading-ident icon sugar — see [`badge!`] for details.
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::badge::ghost(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    Into::<$crate::component::Component<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::component::Component<'_, _>>::into($rest)),+
                ])
                .spacing(4)
            )
        )
    };
    ($($child:expr),+ $(,)?) => {
        $crate::badge::ghost(
            $crate::component::Component::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::component::Component<'_, _>>::into($child)),+
                ])
                .spacing(4)
            )
        )
    };
}

// `#[macro_export]` always lands at the consuming crate's root, so two
// widgets can't both claim a name like `secondary!` there. The fix: the
// real macros above are exported with a `badge_` prefix, then we re-alias
// each to its idiomatic name at the module path (`badge::secondary!`,
// `badge::outline!`, ...) right here. This is the carved exception in
// CLAUDE.md hard rule #3 — the file-based exemption in the pre-commit
// hook permits these aliased re-exports (and matching ones in
// `widget/src/button.rs`). iced uses the same trick for `column!` /
// `column()` coexisting in the `column` module.
#[doc(inline)]
pub use crate::badge_destructive as destructive;
#[doc(inline)]
pub use crate::badge_ghost as ghost;
#[doc(inline)]
pub use crate::badge_outline as outline;
#[doc(inline)]
pub use crate::badge_secondary as secondary;

use crate::Weight;
use crate::component::{Component, IntoElement};
use iced_core::mouse::Interaction;
use iced_widget::{Renderer, mouse_area};
use icedxdy_core::Theme;
use icedxdy_core::theme;

/// Badge variant matching shadcn/ui
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
}

/// A small inline pill label.
pub struct Badge<'a, Message> {
    content: Component<'a, Message>,
    variant: Variant,
    on_press: Option<Message>,
}

/// Create a [`Badge`] with default variant.
pub fn badge<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Badge<'a, Message> {
    Badge {
        content: content.into(),
        variant: Variant::Default,
        on_press: None,
    }
}

/// Create a [`Badge`] with the default variant.
///
/// Equivalent to [`badge`]; provided for symmetry with the other variant
/// helpers.
pub fn default<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Badge<'a, Message> {
    badge(content).variant(Variant::Default)
}

/// Create a [`Badge`] with the secondary variant.
pub fn secondary<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Badge<'a, Message> {
    badge(content).variant(Variant::Secondary)
}

/// Create a [`Badge`] with the outline variant.
pub fn outline<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Badge<'a, Message> {
    badge(content).variant(Variant::Outline)
}

/// Create a [`Badge`] with the destructive variant.
pub fn destructive<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Badge<'a, Message> {
    badge(content).variant(Variant::Destructive)
}

/// Create a [`Badge`] with the ghost variant.
pub fn ghost<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Badge<'a, Message> {
    badge(content).variant(Variant::Ghost)
}

impl<'a, Message: 'a> Badge<'a, Message> {
    /// Set the badge variant.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
}

impl<'a, Message: Clone + 'a> Badge<'a, Message> {
    /// Make the badge clickable, emitting `message` on left-button press.
    ///
    /// Mirrors shadcn's `<Badge asChild><a href=...>` pattern: the badge
    /// keeps its visual styling and gains pointer-cursor + click semantics.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<'a, Message: Clone + 'a> From<Badge<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(badge: Badge<'a, Message>) -> Self {
        Box::new(badge).into_element()
    }
}

impl<'a, Message: Clone + 'a> From<Badge<'a, Message>> for Component<'a, Message> {
    fn from(b: Badge<'a, Message>) -> Self {
        Component::new(b)
    }
}

impl<'a, Message: Clone + 'a> IntoElement<'a, Message> for Badge<'a, Message> {
    fn size(&mut self, size: crate::Size) {
        self.content.size(size);
    }

    fn weight(&mut self, weight: Weight) {
        self.content.weight(weight);
    }

    fn into_element(mut self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        // Push text-xs (12px) and medium down to content (nova `font-medium`)
        self.content.size(crate::Size::Xs);
        self.content.weight(Weight::Medium);

        let style_fn: iced_widget::container::StyleFn<'_, Theme> = match self.variant {
            Variant::Default => theme::badge::Default.into(),
            Variant::Secondary => theme::badge::Secondary.into(),
            Variant::Destructive => theme::badge::Destructive.into(),
            Variant::Outline => theme::badge::Outline.into(),
            Variant::Ghost => theme::badge::Ghost.into(),
        };

        // py-0.5 = 2px, px-2 = 8px (nova `px-2 py-0.5`).
        //
        // NOTE: nova additionally tightens the icon-side padding via
        // `has-data-[icon=inline-end]:pr-1.5` and
        // `has-data-[icon=inline-start]:pl-1.5` (6px instead of 8px). We
        // don't yet have an icon-side hint on the builder so this remains
        // a uniform 8px on both sides — minor visual drift when an icon
        // sits flush on either edge. Acceptable for this audit pass.
        let pill = iced_widget::container(self.content.into_iced())
            .padding([2, 8])
            .center_y(iced_core::Length::Shrink)
            .style(style_fn);

        match self.on_press {
            Some(message) => mouse_area(pill)
                .interaction(Interaction::Pointer)
                .on_press(message)
                .into(),
            None => pill.into(),
        }
    }
}

#[cfg(test)]
mod macro_smoke {
    //! Compile-only smoke tests: invoke each variant macro both single-arg
    //! and multi-arg, and verify the result is a [`Badge`].

    use crate::badge::Badge;

    #[derive(Clone)]
    enum Msg {}

    #[allow(dead_code)]
    fn assert_badge<M: Clone>(_: &Badge<'_, M>) {}

    #[test]
    fn macro_arms_compile() {
        // Default variant — single + multi.
        let _: Badge<'_, Msg> = crate::badge!("default");
        let _: Badge<'_, Msg> = crate::badge!["a", "b"];

        // Variant macros at module path — single-arg arm.
        let _: Badge<'_, Msg> = crate::badge::secondary!("s");
        let _: Badge<'_, Msg> = crate::badge::destructive!("d");
        let _: Badge<'_, Msg> = crate::badge::outline!("o");
        let _: Badge<'_, Msg> = crate::badge::ghost!("g");

        // Variant macros at module path — multi-arg arm.
        let _: Badge<'_, Msg> = crate::badge::secondary!["s1", "s2"];
        let _: Badge<'_, Msg> = crate::badge::destructive!["d1", "d2"];
        let _: Badge<'_, Msg> = crate::badge::outline!["o1", "o2"];
        let _: Badge<'_, Msg> = crate::badge::ghost!["g1", "g2"];
    }

    /// Leading-ident sugar: `badge![my_icon, "Tag"]` rewrites the bare
    /// ident to `icon::my_icon()` at the call site, then prepends it
    /// to the row of remaining children. Same arm exists on every
    /// variant macro.
    #[test]
    fn leading_ident_sugar_arm() {
        // Local `icon` module — name load-bearing because the sugar arm
        // expands to `icon::$first()` resolved at the call site.
        mod icon {
            pub fn my_icon<'a, T>() -> iced_widget::Text<'a, T>
            where
                T: iced_widget::text::Catalog,
            {
                iced_widget::Text::new("")
            }
        }

        let _: Badge<'_, Msg> = crate::badge![my_icon, "Tag"];
        let _: Badge<'_, Msg> = crate::badge::secondary![my_icon, "Verified"];
        let _: Badge<'_, Msg> = crate::badge::destructive![my_icon, "Deleting"];
        let _: Badge<'_, Msg> = crate::badge::outline![my_icon, "Bookmark"];
        let _: Badge<'_, Msg> = crate::badge::ghost![my_icon, "Ghost"];
    }
}
