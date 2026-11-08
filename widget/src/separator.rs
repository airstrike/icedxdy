//! Separator — shadcn/ui faithful port (Radix `Separator`).
//!
//! A 1px `bg-border` line: full width when horizontal (the default), full
//! height of its row when vertical (`self-stretch`).
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::{column, row, separator};
//! use icedxdy::Vertical;
//!
//! column![content_a, separator(), content_b]
//!
//! row!["Blog", separator().orientation(Vertical), "Docs"]
//! ```

use iced_widget::{Renderer, Widget, rule};
use icedxdy_core::{Orientation, Theme};

use crate::compose::{Compose, Slot};

/// A thin line dividing content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Separator {
    orientation: Orientation,
}

/// Create a horizontal [`Separator`].
pub fn separator() -> Separator {
    Separator {
        orientation: Orientation::Horizontal,
    }
}

impl Separator {
    /// Set whether the line runs across (horizontal) or down (vertical).
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }
}

impl<Message> Compose<Message> for Separator {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        match self.orientation {
            Orientation::Horizontal => rule::horizontal(1).boxed(),
            Orientation::Vertical => rule::vertical(1).boxed(),
        }
    }
}

impl<'a, Message: 'a> From<Separator> for Slot<'a, Message> {
    fn from(separator: Separator) -> Self {
        Slot::new(separator)
    }
}

impl<'a, Message: 'a> From<Separator> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(separator: Separator) -> Self {
        Box::new(separator).compose()
    }
}
