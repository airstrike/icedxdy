//! Button components
//!
//! Semantic button helpers based on shadcn/ui button variants.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::button;
//!
//! button::primary("Submit").on_press(Message::Submit)
//! button::destructive("Delete").on_press(Message::Delete)
//! button::link("Learn more").on_press(Message::Navigate)
//! ```

use iced_core::text::Span;
use iced_core::Font;
use iced_widget::button::Button;
use iced_widget::text::{Rich, Text};

use crate::theme;
use crate::Theme;

/// A link widget that looks like a text link with underline on hover
pub struct Link {
    label: String,
}

impl Link {
    /// Create a new link with the given label
    pub fn new(label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
        }
    }

    /// Set the message to produce when the link is clicked
    pub fn on_press<'a, Message: Clone + 'a>(
        self,
        message: Message,
    ) -> Rich<'a, Message, Message, Theme> {
        let span: Span<'a, Message, Font> = Span::new(self.label).link(message);
        Rich::with_spans([span])
            .on_link_click(|msg| msg)
            .style(theme::text::link)
    }
}

/// Primary button - main call to action
///
/// Dark background, light text (bg-primary text-primary-foreground)
pub fn primary<'a, Message>(label: impl ToString) -> Button<'a, Message, Theme> {
    Button::new(Text::new(label.to_string())).style(theme::button::default)
}

/// Secondary button - secondary actions
///
/// Muted background (bg-secondary text-secondary-foreground)
pub fn secondary<'a, Message>(label: impl ToString) -> Button<'a, Message, Theme> {
    Button::new(Text::new(label.to_string())).style(theme::button::secondary)
}

/// Outline button - bordered, transparent background
///
/// Border only, hover shows accent (border bg-background hover:bg-accent)
pub fn outline<'a, Message>(label: impl ToString) -> Button<'a, Message, Theme> {
    Button::new(Text::new(label.to_string())).style(theme::button::outline)
}

/// Ghost button - minimal, transparent
///
/// No background until hover (hover:bg-accent)
pub fn ghost<'a, Message>(label: impl ToString) -> Button<'a, Message, Theme> {
    Button::new(Text::new(label.to_string())).style(theme::button::ghost)
}

/// Destructive button - dangerous actions
///
/// Red background (bg-destructive text-destructive-foreground)
pub fn destructive<'a, Message>(label: impl ToString) -> Button<'a, Message, Theme> {
    Button::new(Text::new(label.to_string())).style(theme::button::destructive)
}

/// Link - text that looks and acts like a link
///
/// Primary text color, underline on hover. Uses rich_text with native click handling.
///
/// ```ignore
/// button::link("Learn more").on_press(Message::Navigate)
/// ```
pub fn link(label: impl ToString) -> Link {
    Link::new(label)
}
