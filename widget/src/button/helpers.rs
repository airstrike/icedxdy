use crate::compose::Slot;
use icedxdy_core::theme;

use super::{Button, Link, button};

/// Primary button (bg-primary text-primary-foreground)
pub fn primary<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::Default)
}

/// Secondary button (bg-secondary text-secondary-foreground)
pub fn secondary<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::Secondary)
}

/// Outline button (border bg-background hover:bg-accent)
pub fn outline<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::Outline)
}

/// Ghost button (hover:bg-accent)
pub fn ghost<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::Ghost)
}

/// Destructive button (bg-destructive text-destructive-foreground)
pub fn destructive<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::Destructive)
}

/// Icon-only button with outline style and square sizing
pub fn icon<'a, Message: Clone + 'a>(content: impl Into<Slot<'a, Message>>) -> Button<'a, Message> {
    button(content).square().style(theme::button::Outline)
}

/// Link - text with underline on hover
pub fn link(label: impl ToString) -> Link {
    Link::new(label)
}
