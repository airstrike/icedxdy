//! Pagination - shadcn/ui faithful port.
//!
//! Page navigation: a centered row of ghost buttons, the current page in
//! outline. shadcn's `Pagination` / `PaginationContent` / `PaginationItem`
//! nesting collapses into [`pagination`] over a list of items.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::pagination;
//!
//! pagination([
//!     pagination::previous().on_press(Message::Previous).into(),
//!     pagination::link("1").on_press(Message::Page(1)).into(),
//!     pagination::link("2").active(true).into(),
//!     pagination::link("3").on_press(Message::Page(3)).into(),
//!     pagination::ellipsis().into(),
//!     pagination::next().on_press(Message::Next).into(),
//! ])
//! ```
//!
//! The previous and next chevrons and the ellipsis default to Unicode
//! glyphs (`‹`, `›`, `…`); the widget crate carries no icon font, so pass a
//! lucide icon with `.icon(..)` for parity.

use iced_core::Alignment;
use iced_core::Length::Fill;
use iced_core::alignment::{Horizontal, Vertical};
use iced_widget::Renderer;
use icedxdy_core::Theme;

use crate::button;
use crate::compose::{Compose, Slot};
use crate::row::Row;
use crate::text::{self, Text};

/// A row of page links, centered in the available width.
pub struct Pagination<'a, Message> {
    items: Vec<Slot<'a, Message>>,
}

/// Create a [`Pagination`] from its items, in order.
pub fn pagination<'a, Message: 'a>(
    items: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Pagination<'a, Message> {
    Pagination {
        items: items.into_iter().collect(),
    }
}

impl<'a, Message: 'a> Compose<Message> for Pagination<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        // `mx-auto flex w-full justify-center` around `flex items-center gap-0.5`.
        crate::container(
            Row::with_children(self.items)
                .spacing(2)
                .align_y(Alignment::Center),
        )
        .width(Fill)
        .align_x(Horizontal::Center)
        .into()
    }
}

impl<'a, Message: 'a> From<Pagination<'a, Message>> for Slot<'a, Message> {
    fn from(pagination: Pagination<'a, Message>) -> Self {
        Slot::new(pagination)
    }
}

impl<'a, Message: 'a> From<Pagination<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(pagination: Pagination<'a, Message>) -> Self {
        Box::new(pagination).compose()
    }
}

/// A page number: a square ghost button, outlined while it is the current
/// page (`PaginationLink`).
pub struct Link<'a, Message> {
    content: Slot<'a, Message>,
    is_active: bool,
    on_press: Option<Message>,
}

/// Create a [`Link`] showing `content`.
pub fn link<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Link<'a, Message> {
    Link {
        content: content.into(),
        is_active: false,
        on_press: None,
    }
}

impl<'a, Message: 'a> Link<'a, Message> {
    /// Mark the link as the current page (`isActive`).
    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }

    /// Publish `message` when the link is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Link<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let button = button::button(self.content).square();
        let button = if self.is_active {
            button.style(button::Outline)
        } else {
            button.style(button::Ghost)
        };

        button.on_press_maybe(self.on_press).into()
    }
}

impl<'a, Message: Clone + 'a> From<Link<'a, Message>> for Slot<'a, Message> {
    fn from(link: Link<'a, Message>) -> Self {
        Slot::new(link)
    }
}

/// Which way a [`Step`] goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Previous,
    Next,
}

/// A "Previous" or "Next" ghost button with its chevron
/// (`PaginationPrevious`, `PaginationNext`).
pub struct Step<'a, Message> {
    direction: Direction,
    label: Slot<'a, Message>,
    icon: Option<Slot<'a, Message>>,
    on_press: Option<Message>,
}

/// Create the "Previous" [`Step`].
pub fn previous<'a, Message: 'a>() -> Step<'a, Message> {
    Step {
        direction: Direction::Previous,
        label: "Previous".into(),
        icon: None,
        on_press: None,
    }
}

/// Create the "Next" [`Step`].
pub fn next<'a, Message: 'a>() -> Step<'a, Message> {
    Step {
        direction: Direction::Next,
        label: "Next".into(),
        icon: None,
        on_press: None,
    }
}

impl<'a, Message: 'a> Step<'a, Message> {
    /// Replace the label (shadcn's `text` prop).
    pub fn label(mut self, label: impl Into<Slot<'a, Message>>) -> Self {
        self.label = label.into();
        self
    }

    /// Replace the chevron glyph, typically with a lucide chevron icon.
    pub fn icon(mut self, icon: impl Into<Slot<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Publish `message` when the step is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Step<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Step {
            direction,
            label,
            icon,
            on_press,
        } = *self;

        let icon = icon.unwrap_or_else(|| {
            let glyph = match direction {
                Direction::Previous => "\u{2039}",
                Direction::Next => "\u{203A}",
            };
            Text::new(glyph).size(text::size::BASE).into()
        });

        let children = match direction {
            Direction::Previous => [icon, label],
            Direction::Next => [label, icon],
        };

        button::button(Row::with_children(children))
            .style(button::Ghost)
            .on_press_maybe(on_press)
            .into()
    }
}

impl<'a, Message: Clone + 'a> From<Step<'a, Message>> for Slot<'a, Message> {
    fn from(step: Step<'a, Message>) -> Self {
        Slot::new(step)
    }
}

/// A 32px "more pages" marker (`PaginationEllipsis`).
pub struct Ellipsis<'a, Message> {
    icon: Option<Slot<'a, Message>>,
}

/// Create an [`Ellipsis`].
pub fn ellipsis<'a, Message: 'a>() -> Ellipsis<'a, Message> {
    Ellipsis { icon: None }
}

impl<'a, Message: 'a> Ellipsis<'a, Message> {
    /// Replace the `…` glyph, typically with lucide's `ellipsis` icon.
    pub fn icon(mut self, icon: impl Into<Slot<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Ellipsis<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let icon = self
            .icon
            .unwrap_or_else(|| Text::new("\u{2026}").size(text::size::SM).into());

        // `flex size-8 items-center justify-center`.
        crate::container(icon)
            .width(32)
            .height(32)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}

impl<'a, Message: 'a> From<Ellipsis<'a, Message>> for Slot<'a, Message> {
    fn from(ellipsis: Ellipsis<'a, Message>) -> Self {
        Slot::new(ellipsis)
    }
}
