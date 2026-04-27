//! Alert component
//!
//! A callout box with optional icon, title, description, and an
//! optional top-right action (typically a button).
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::alert;
//!
//! alert::alert()
//!     .icon(icon::terminal())
//!     .title("Heads up!")
//!     .description("You can add components to your app using the cli.")
//! ```

use iced_core::alignment::{Horizontal, Vertical};
use iced_core::text::IntoFragment;
use iced_core::{Length::*, Padding};
use iced_widget::Renderer;
use icedxdy_core::{Theme, theme};

use crate::Size;
use crate::component::{Component, IntoElement};
use crate::text;

/// Alert variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Destructive,
}

/// A callout box with optional icon, title, description, and action.
pub struct Alert<'a, Message> {
    icon: Option<Component<'a, Message>>,
    title: Option<text::Text<'a>>,
    description: Option<text::Text<'a>>,
    action: Option<Component<'a, Message>>,
    variant: Variant,
}

/// Create an [`Alert`] with default variant.
pub fn alert<'a, Message: 'a>() -> Alert<'a, Message> {
    Alert {
        icon: None,
        title: None,
        description: None,
        action: None,
        variant: Variant::Default,
    }
}

impl<'a, Message: 'a> Alert<'a, Message> {
    /// Set the icon element.
    pub fn icon(mut self, icon: impl Into<Component<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the title text. Rendered at `text-sm` with medium weight.
    pub fn title(mut self, title: impl IntoFragment<'a>) -> Self {
        self.title = Some(text::Text::new(title).size(text::size::SM).medium());
        self
    }

    /// Set the description text. Rendered at `text-sm` muted in the
    /// default variant; destructive at alpha 0.9 in the destructive variant.
    pub fn description(mut self, description: impl IntoFragment<'a>) -> Self {
        self.description = Some(text::Text::new(description).size(text::size::SM));
        self
    }

    /// Add an action element (typically a button) shown at the top-right.
    pub fn action(mut self, action: impl Into<Component<'a, Message>>) -> Self {
        self.action = Some(action.into());
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
        let Alert { icon, title, description, action, variant } = *self;

        let description = description.map(|t| match variant {
            Variant::Default => t.style(theme::text::Muted),
            Variant::Destructive => t.style(theme::text::Destructive + theme::tweak::Alpha(0.9)),
        });

        let mut text_col = iced_widget::Column::new().spacing(2);
        if let Some(title) = title {
            text_col = text_col.push(title);
        }
        if let Some(desc) = description {
            text_col = text_col.push(desc);
        }

        let body: iced_core::Element<'a, Message, Theme, Renderer> = if let Some(mut icon) = icon {
            icon.size(Size::Custom(16.0));
            let icon_wrapper = iced_widget::container(icon.into_iced())
                .padding(Padding { top: 2.0, ..Padding::ZERO });
            iced_widget::Row::new()
                .spacing(8)
                .align_y(iced_core::Alignment::Start)
                .push(icon_wrapper)
                .push(text_col)
                .into()
        } else {
            text_col.into()
        };

        let container_style: iced_widget::container::StyleFn<'_, Theme> = match variant {
            Variant::Default => theme::alert::Default.into(),
            Variant::Destructive => theme::alert::Destructive.into(),
        };

        let padding = if action.is_some() {
            Padding { top: 8.0, right: 72.0, bottom: 8.0, left: 10.0 }
        } else {
            Padding { top: 8.0, right: 10.0, bottom: 8.0, left: 10.0 }
        };

        let card = iced_widget::container(body)
            .padding(padding)
            .width(Fill)
            .style(container_style);

        match action {
            None => card.into(),
            Some(action) => {
                let overlay = iced_widget::container(action.into_iced())
                    .width(Fill)
                    .height(Fill)
                    .align_x(Horizontal::Right)
                    .align_y(Vertical::Top)
                    .padding(Padding { top: 8.0, right: 8.0, bottom: 0.0, left: 0.0 });
                iced_widget::stack![card, overlay].into()
            }
        }
    }
}
