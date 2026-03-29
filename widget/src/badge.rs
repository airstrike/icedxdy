//! Badge component — small inline pill label
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::badge;
//!
//! badge("Badge")
//! badge("Destructive").variant(badge::Variant::Destructive)
//! ```

use crate::component::{Component, IntoElement};
use crate::Weight;
use iced_widget::Renderer;
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
}

/// A small inline pill label.
pub struct Badge<'a, Message> {
    content: Component<'a, Message>,
    variant: Variant,
}

/// Create a [`Badge`] with default variant.
pub fn badge<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Badge<'a, Message> {
    Badge {
        content: content.into(),
        variant: Variant::Default,
    }
}

impl<'a, Message: 'a> Badge<'a, Message> {
    /// Set the badge variant.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
}

impl<'a, Message: 'a> From<Badge<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(badge: Badge<'a, Message>) -> Self {
        Box::new(badge).into_element()
    }
}

impl<'a, Message: 'a> From<Badge<'a, Message>> for Component<'a, Message> {
    fn from(b: Badge<'a, Message>) -> Self {
        Component::new(b)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Badge<'a, Message> {
    fn size(&mut self, size: crate::Size) {
        self.content.size(size);
    }

    fn weight(&mut self, weight: Weight) {
        self.content.weight(weight);
    }

    fn into_element(mut self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        // Push text-xs (12px) and semibold down to content
        self.content.size(crate::Size::Xs);
        self.content.weight(Weight::Semibold);

        let style_fn: fn(&Theme) -> iced_widget::container::Style = match self.variant {
            Variant::Default => theme::badge::default,
            Variant::Secondary => theme::badge::secondary,
            Variant::Destructive => theme::badge::destructive,
            Variant::Outline => theme::badge::outline,
        };

        // py-0.5 = 2px, px-2.5 = 10px
        iced_widget::container(self.content.into_iced())
            .padding([2, 10])
            .center_y(iced_core::Length::Shrink)
            .style(style_fn)
            .into()
    }
}
