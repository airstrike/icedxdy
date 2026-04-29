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
//! ```

use crate::component::{Component, IntoElement};
use crate::Weight;
use iced_core::mouse::Interaction;
use iced_widget::{mouse_area, Renderer};
use icedxdy_core::theme;
use icedxdy_core::Theme;

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
pub fn default<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Badge<'a, Message> {
    badge(content).variant(Variant::Default)
}

/// Create a [`Badge`] with the secondary variant.
pub fn secondary<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Badge<'a, Message> {
    badge(content).variant(Variant::Secondary)
}

/// Create a [`Badge`] with the outline variant.
pub fn outline<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Badge<'a, Message> {
    badge(content).variant(Variant::Outline)
}

/// Create a [`Badge`] with the destructive variant.
pub fn destructive<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Badge<'a, Message> {
    badge(content).variant(Variant::Destructive)
}

/// Create a [`Badge`] with the ghost variant.
pub fn ghost<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Badge<'a, Message> {
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
