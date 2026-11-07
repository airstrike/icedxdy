//! Badge component — small inline pill label
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::badge;
//!
//! badge("Badge")
//! badge("Secondary").style(badge::Secondary)
//! badge("Destructive").style(badge::Destructive)
//! badge("Outline").style(badge::Outline)
//! badge("Ghost").style(badge::Ghost)
//! badge("Open Link").style(badge::Secondary).on_press(Message::Open)
//!
//! // Macro form: single-arg defers to [`badge`]; multi-arg wraps children
//! // in a `row::Row::with_children([...]).spacing(4)` to land the nova
//! // `gap-1` (4px) icon-text gap automatically.
//! badge!["Verified", icon::badge_check()]
//! badge!["Verified", icon::badge_check()].style(badge::Secondary)
//! badge!["Bookmark", icon::bookmark()].style(badge::Outline)
//! ```
//!
//! # Variants
//!
//! shadcn's `variant` prop is a style: `.style(badge::Outline)`.

use iced_widget::Widget;
/// Create a badge with one or more children.
///
/// Two forms:
/// - `badge!("Badge")` — single child, defers to [`badge`].
/// - `badge![child1, child2, ...]` — multiple children, wrapped in a
///   row with `spacing(4)` (nova `gap-1`) and passed to [`badge`].
///
/// The variant is a style, set after the macro:
///
/// ```ignore
/// badge!["Verified", icon::badge_check()]
/// badge!["Verified", icon::badge_check()].style(badge::Secondary)
/// badge![spinner(), "Deleting"].style(badge::Destructive)
/// badge![badge_check, "Verified"]                    // leading-ident sugar
/// ```
#[macro_export]
macro_rules! badge {
    // Single-arg arm
    ($single:expr $(,)?) => {
        $crate::badge::badge($single)
    };
    // Leading-ident icon sugar: the first arg as a bare ident is rewritten
    // to `icon::$first()`. Requires `use crate::icon;` (or equivalent path
    // to an `icon` module exposing `fn $first() -> Text`) in scope at the
    // call site. Macro hygiene: `icon` here resolves at the call site, not
    // the def site, as in `button!`.
    //
    // `badge![badge_check, "Verified"]` => badge(row![icon::badge_check(), "Verified"].spacing(4))
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::badge::badge(
            $crate::compose::Slot::new(
                $crate::row::Row::with_children([
                    Into::<$crate::compose::Slot<'_, _>>::into(icon::$first()),
                    $(Into::<$crate::compose::Slot<'_, _>>::into($rest)),+
                ])
                .spacing(4)
            )
        )
    };
    // Multi-arg arm — wraps children in a row with 4px gap.
    ($($child:expr),+ $(,)?) => {
        $crate::badge::badge(
            $crate::compose::Slot::new(
                $crate::row::Row::with_children([
                    $(Into::<$crate::compose::Slot<'_, _>>::into($child)),+
                ])
                .spacing(4)
            )
        )
    };
}

use crate::Weight;
use crate::compose::{Compose, Inherited, Slot};
use iced_core::mouse::Interaction;
use iced_widget::{Renderer, mouse_area};
use icedxdy_core::Theme;

pub use icedxdy_core::theme::badge::{Default, Destructive, Ghost, Outline, Secondary};

/// A small inline pill label.
pub struct Badge<'a, Message> {
    content: Slot<'a, Message>,
    style: iced_widget::container::StyleFn<'a, Theme>,
    on_press: Option<Message>,
}

/// Create a [`Badge`] with the default variant.
pub fn badge<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Badge<'a, Message> {
    Badge {
        content: content.into(),
        style: Default.into(),
        on_press: None,
    }
}

impl<'a, Message: 'a> Badge<'a, Message> {
    /// Set the style: a variant (`badge::Secondary`, `badge::Outline`, ...)
    /// or a tweaked `Spec`.
    pub fn style(mut self, style: impl Into<iced_widget::container::StyleFn<'a, Theme>>) -> Self {
        self.style = style.into();
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
        Box::new(badge).compose()
    }
}

impl<'a, Message: Clone + 'a> From<Badge<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Badge<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Badge<'a, Message> {
    fn compose<'b>(mut self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        // Push text-xs (12px), medium, and `leading-4` (16px) down to content
        // (nova `font-medium text-xs leading-none` — the badge wrapper supplies
        // the explicit `leading-4` parent so the rendered pill height matches
        // shadcn's 22px vertical extent rather than iced's default 1.3 ratio).
        self.content.inherit(
            &Inherited::default()
                .with_size(crate::Size::Xs)
                .with_weight(Weight::Medium)
                .with_line_height(16.0),
        );

        let style_fn = self.style;

        // py-0.5 = 2px, px-2 = 8px (nova `px-2 py-0.5`).
        //
        // NOTE: nova additionally tightens the icon-side padding via
        // `has-data-[icon=inline-end]:pr-1.5` and
        // `has-data-[icon=inline-start]:pl-1.5` (6px instead of 8px). We
        // don't yet have an icon-side hint on the builder so this remains
        // a uniform 8px on both sides — minor visual drift when an icon
        // sits flush on either edge. Acceptable for this audit pass.
        let pill = iced_widget::container(self.content.compose())
            .padding([2, 8])
            .center_y(iced_core::Length::Shrink)
            .style(style_fn);

        match self.on_press {
            Some(message) => mouse_area(pill)
                .interaction(Interaction::Pointer)
                .on_press(message)
                .boxed(),
            None => pill.boxed(),
        }
    }
}

#[cfg(test)]
mod macro_smoke {
    //! Compile-only smoke tests: invoke the macro single-arg, multi-arg and
    //! with the leading-ident sugar, and verify the result is a [`Badge`].

    use crate::badge::{self, Badge};

    #[derive(Clone)]
    enum Msg {}

    #[test]
    fn macro_arms_compile() {
        let _: Badge<'_, Msg> = crate::badge!("default");
        let _: Badge<'_, Msg> = crate::badge!["a", "b"].style(badge::Secondary);
        let _: Badge<'_, Msg> = crate::badge!("o").style(badge::Outline);
    }

    /// Leading-ident sugar: `badge![my_icon, "Tag"]` rewrites the bare
    /// ident to `icon::my_icon()` at the call site, then prepends it to the
    /// row of remaining children.
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
        let _: Badge<'_, Msg> = crate::badge![my_icon, "Deleting"].style(badge::Destructive);
    }
}
