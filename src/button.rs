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
//! button::outline("Settings").on_press(Message::Nav)
//! button::icon(icon::chevron_right().size(16)).on_press(Message::Next)
//! ```

use iced_core::text::Span;
use iced_core::{Element, Font, Padding};
use iced_widget::button::Button;
use iced_widget::text::Rich;
use iced_widget::Renderer;

use crate::theme;
use crate::Theme;

/// Button size variants matching shadcn/ui
///
/// Each size defines its own padding. The `Icon` variant uses equal
/// padding on all sides to produce a square button.
#[derive(Debug, Clone, Copy, Default)]
pub enum Size {
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
}

impl Size {
    /// Padding for this size, matching shadcn: sm=`py-1 px-3`, default=`py-2 px-4`,
    /// lg=`py-2 px-8`, icon=`p-2`
    pub const fn padding(self) -> Padding {
        match self {
            Size::Sm => Padding { top: 4.0, bottom: 4.0, left: 12.0, right: 12.0 },
            Size::Default => Padding { top: 8.0, bottom: 8.0, left: 16.0, right: 16.0 },
            Size::Lg => Padding { top: 8.0, bottom: 8.0, left: 32.0, right: 32.0 },
            Size::Icon => Padding { top: 8.0, bottom: 8.0, left: 8.0, right: 8.0 },
        }
    }
}

/// Create a button with default size padding
pub fn button<'a, Message>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme> {
    Button::new(content).padding(Size::Default.padding())
}

/// A link widget that looks like a text link with underline on hover
pub struct Link {
    label: String,
}

impl Link {
    pub fn new(label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
        }
    }

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

/// Primary button (bg-primary text-primary-foreground)
pub fn primary<'a, Message>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme> {
    button(content).style(theme::button::default)
}

/// Secondary button (bg-secondary text-secondary-foreground)
pub fn secondary<'a, Message>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme> {
    button(content).style(theme::button::secondary)
}

/// Outline button (border bg-background hover:bg-accent)
pub fn outline<'a, Message>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme> {
    button(content).style(theme::button::outline)
}

/// Ghost button (hover:bg-accent)
pub fn ghost<'a, Message>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme> {
    button(content).style(theme::button::ghost)
}

/// Destructive button (bg-destructive text-destructive-foreground)
pub fn destructive<'a, Message>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme> {
    button(content).style(theme::button::destructive)
}

/// Icon-only button with outline style and square padding
pub fn icon<'a, Message>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme> {
    button(content)
        .padding(Size::Icon.padding())
        .style(theme::button::outline)
}

/// Link - text with underline on hover
pub fn link(label: impl ToString) -> Link {
    Link::new(label)
}
