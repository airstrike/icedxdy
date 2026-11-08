//! Tooltip: shadcn/ui faithful port (Radix `Tooltip`).
//!
//! A `text-xs` hint on a `bg-foreground` panel with an arrow pointing at its
//! trigger, shown while the pointer rests on the trigger, on the trigger's
//! [`Side`] (top by default). Built on iced's
//! [`anchored`](iced_widget::anchored) overlay in hover mode, so it flips
//! and shifts to stay in the window, and `Escape` hides it.
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

use std::time::Duration;

use iced_core::Pixels;
use iced_widget::{Renderer, Widget, anchored};
use icedxdy_core::Theme;
use icedxdy_core::theme::tooltip::content;

use crate::Size;
use crate::compose::{Compose, Inherited, Slot};

pub use iced_widget::anchored::{Align, Side};

/// The arrow's size: nova's `size-2.5` square, rotated, shows a 10×5
/// triangle.
const ARROW: [f32; 2] = [10.0, 5.0];

/// A trigger that shows a hint while hovered.
pub struct Tooltip<'a, Message> {
    trigger: Slot<'a, Message>,
    content: Slot<'a, Message>,
    position: anchored::Position,
    delay: Duration,
    open: Option<bool>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

/// Create a [`Tooltip`] showing `content` while `trigger` is hovered.
pub fn tooltip<'a, Message: 'a>(
    trigger: impl Into<Slot<'a, Message>>,
    content: impl Into<Slot<'a, Message>>,
) -> Tooltip<'a, Message> {
    Tooltip {
        trigger: trigger.into(),
        content: content.into(),
        // Radix places the content `sideOffset + arrow height` away; the
        // arrow fills that gap.
        position: anchored::Position::new(Side::Top).offset(10),
        // shadcn's `TooltipProvider delayDuration={0}`.
        delay: Duration::ZERO,
        open: None,
        on_open_change: None,
    }
}

impl<'a, Message: 'a> Tooltip<'a, Message> {
    /// Set the side of the trigger the tooltip appears on.
    pub fn side(mut self, side: Side) -> Self {
        self.position = self.position.side(side);
        self
    }

    /// Set how the tooltip lines up with its trigger (`Center` by default).
    pub fn align(mut self, align: Align) -> Self {
        self.position = self.position.align(align);
        self
    }

    /// Set the gap to the trigger, the arrow included (10px by default).
    pub fn side_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.position = self.position.offset(offset);
        self
    }

    /// Set how long the pointer rests on the trigger before the tooltip
    /// shows (`delayDuration`, none by default).
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Control the tooltip: it shows exactly when `open` is, and hovering
    /// only publishes [`on_open_change`](Self::on_open_change).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Set the message published when hovering shows or hides the tooltip.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
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
            .max_width(320)
            .style(|theme: &Theme| content(theme));

        let mut tooltip = anchored(self.trigger.compose(), Slot::from(panel).compose())
            .position(self.position)
            .hover(self.delay, Duration::ZERO)
            .arrow(ARROW, |theme: &Theme| theme.neutral.foreground.into());

        if let Some(open) = self.open {
            tooltip = tooltip.open(open);
        }

        if let Some(on_change) = self.on_open_change {
            tooltip = tooltip.on_toggle(on_change);
        }

        tooltip.boxed()
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
