//! Tooltip - shadcn/ui port over iced's tooltip.
//!
//! A `text-xs` hint on a `bg-foreground` panel, shown while the pointer is
//! over its trigger, on the trigger's [`Side`] (top by default).
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::{button, tooltip};
//!
//! tooltip(button("Hover").style(button::Outline), "Add to library")
//!
//! tooltip(button("Left").style(button::Outline), "Add to library")
//!     .side(tooltip::Side::Left)
//! ```

use iced_widget::tooltip::Position;
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;
use icedxdy_core::theme::tooltip::content;

use crate::Size;
use crate::compose::{Compose, Inherited, Slot};

/// The side of its trigger a tooltip appears on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Side {
    /// Above the trigger.
    #[default]
    Top,
    /// To the right of the trigger.
    Right,
    /// Below the trigger.
    Bottom,
    /// To the left of the trigger.
    Left,
}

/// A trigger that shows a hint while hovered.
pub struct Tooltip<'a, Message> {
    trigger: Slot<'a, Message>,
    content: Slot<'a, Message>,
    side: Side,
}

/// Create a [`Tooltip`] showing `content` while `trigger` is hovered.
pub fn tooltip<'a, Message: 'a>(
    trigger: impl Into<Slot<'a, Message>>,
    content: impl Into<Slot<'a, Message>>,
) -> Tooltip<'a, Message> {
    Tooltip {
        trigger: trigger.into(),
        content: content.into(),
        side: Side::Top,
    }
}

impl<'a, Message: 'a> Tooltip<'a, Message> {
    /// Set the side of the trigger the tooltip appears on.
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Tooltip<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.trigger.inherit(from);
    }

    fn compose<'b>(mut self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        self.content
            .inherit(&Inherited::default().with_size(Size::Xs));

        // `px-3 py-1.5 max-w-xs`.
        let panel = crate::container(self.content)
            .padding([6, 12])
            .max_width(320);

        let position = match self.side {
            Side::Top => Position::Top,
            Side::Right => Position::Right,
            Side::Bottom => Position::Bottom,
            Side::Left => Position::Left,
        };

        iced_widget::tooltip(
            self.trigger.compose(),
            iced_core::Element::from(panel),
            position,
        )
        .gap(4)
        .style(|theme: &Theme| content(theme).into())
        .boxed()
    }
}

impl<'a, Message: 'a> From<Tooltip<'a, Message>> for Slot<'a, Message> {
    fn from(tooltip: Tooltip<'a, Message>) -> Self {
        Slot::new(tooltip)
    }
}

impl<'a, Message: 'a> From<Tooltip<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(tooltip: Tooltip<'a, Message>) -> Self {
        Box::new(tooltip).compose()
    }
}
