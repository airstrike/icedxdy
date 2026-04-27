//! Alert component
//!
//! A callout box with optional icon, title, and description.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::alert;
//!
//! alert()
//!     .icon(icon::terminal())
//!     .title("Heads up!")
//!     .description("You can add components to your app using the cli.")
//! ```

use crate::component::{Component, IntoElement};
use iced_core::Length::*;
use iced_widget::Renderer;
use icedxdy_core::{Theme, theme};

/// Alert variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Destructive,
}

/// A callout box with optional icon, title, and description.
pub struct Alert<'a, Message> {
    icon: Option<Component<'a, Message>>,
    title: Option<Component<'a, Message>>,
    description: Option<Component<'a, Message>>,
    variant: Variant,
}

/// Create an [`Alert`] with default variant.
pub fn alert<'a, Message: 'a>() -> Alert<'a, Message> {
    Alert {
        icon: None,
        title: None,
        description: None,
        variant: Variant::Default,
    }
}

impl<'a, Message: 'a> Alert<'a, Message> {
    /// Set the icon element.
    pub fn icon(mut self, icon: impl Into<Component<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the title text. Rendered as small + semibold + tracking-tight.
    pub fn title(mut self, title: impl Into<Component<'a, Message>>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the description text. Rendered as small + muted style.
    pub fn description(mut self, description: impl Into<Component<'a, Message>>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the variant.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Shorthand for `.variant(Variant::Destructive)`.
    pub fn destructive(self) -> Self {
        self.variant(Variant::Destructive)
    }
}

impl<'a, Message: 'a> From<Alert<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(a: Alert<'a, Message>) -> Self {
        Box::new(a).into_element()
    }
}

impl<'a, Message: 'a> From<Alert<'a, Message>> for Component<'a, Message> {
    fn from(a: Alert<'a, Message>) -> Self {
        Component::new(a)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Alert<'a, Message> {
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        // FIXME: use a slice and optional elements instead of if let Some and
        // pushing the elements into the column
        // Build the text column (title + description)
        let mut text_col = iced_widget::Column::new().spacing(4);

        if let Some(title) = self.title {
            text_col = text_col.push(title.into_iced());
        }

        if let Some(desc) = self.description {
            text_col = text_col.push(desc.into_iced());
        }

        // Build the row: icon (if present) + text column
        let mut content_row = iced_widget::Row::new()
            .spacing(12)
            .align_y(iced_core::Alignment::Start);

        if let Some(icon) = self.icon {
            content_row = content_row.push(icon.into_iced());
        }

        content_row = content_row.push(text_col);

        // FIXME: use new style API
        // Container style based on variant
        let container_style: fn(&Theme) -> iced_widget::container::Style = match self.variant {
            Variant::Default => theme::alert::default,
            Variant::Destructive => theme::alert::destructive,
        };

        iced_widget::container(content_row)
            .padding([12.0, 16.0])
            .width(Fill)
            .style(container_style)
            .into()
    }
}
