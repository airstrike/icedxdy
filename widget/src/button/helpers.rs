use crate::element::Element;
use icedxdy_core::theme;

use super::{button, Button, Link};

/// Primary button (bg-primary text-primary-foreground)
pub fn primary<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::default)
}

/// Secondary button (bg-secondary text-secondary-foreground)
pub fn secondary<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::secondary)
}

/// Outline button (border bg-background hover:bg-accent)
pub fn outline<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::outline)
}

/// Ghost button (hover:bg-accent)
pub fn ghost<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::ghost)
}

/// Destructive button (bg-destructive text-destructive-foreground)
pub fn destructive<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    button(content).style(theme::button::destructive)
}

/// Icon-only button with outline style and square sizing
pub fn icon<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    button(content)
        .square()
        .style(theme::button::outline)
}

/// Link - text with underline on hover
pub fn link(label: impl ToString) -> Link {
    Link::new(label)
}
