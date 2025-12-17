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

use iced_widget::button::Button;
use iced_widget::text::Text;

use crate::theme;
use crate::Theme;

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

/// Link button - text-only, looks like a link
///
/// Primary text color, no background
pub fn link<'a, Message>(label: impl ToString) -> Button<'a, Message, Theme> {
    Button::new(Text::new(label.to_string())).style(theme::button::link)
}
