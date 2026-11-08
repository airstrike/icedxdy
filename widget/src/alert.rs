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
//!
//! alert::alert()
//!     .icon(icon::circle_alert())
//!     .title("Payment failed")
//!     .description("Your payment could not be processed.")
//!     .style(alert::Destructive)
//! ```
//!
//! # Variants
//!
//! shadcn's `variant` prop is a style: `.style(alert::Destructive)`. The
//! style also decides the description color (muted by default,
//! `destructive/90` for [`Destructive`]).

use iced_core::alignment::{Horizontal, Vertical};
use iced_core::text::IntoFragment;
use iced_core::{Length::*, Padding, Pixels};
use iced_widget::Renderer;
use iced_widget::Widget;
use iced_widget::text::{Style, StyleFn};
use icedxdy_core::Theme;

use crate::Size;
use crate::compose::{Compose, Inherited, Slot};
use crate::text;

pub use icedxdy_core::theme::alert::{Default, Destructive, Spec};

/// A callout box with optional icon, title, description, and action.
pub struct Alert<'a, Message> {
    icon: Option<Slot<'a, Message>>,
    title: Option<text::Text<'a>>,
    description: Option<text::Text<'a>>,
    action: Option<Slot<'a, Message>>,
    style: Spec,
    max_width: f32,
}

/// Create an [`Alert`] with the default style.
pub fn alert<'a, Message: 'a>() -> Alert<'a, Message> {
    Alert {
        icon: None,
        title: None,
        description: None,
        action: None,
        style: Default.into(),
        max_width: f32::INFINITY,
    }
}

impl<'a, Message: 'a> Alert<'a, Message> {
    /// Set the icon element.
    pub fn icon(mut self, icon: impl Into<Slot<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the title text. Rendered at `text-sm` with medium weight.
    pub fn title(mut self, title: impl IntoFragment<'a>) -> Self {
        self.title = Some(text::Text::new(title).size(text::size::SM).medium());
        self
    }

    /// Set the description text, rendered at `text-sm` in the style's
    /// description color.
    pub fn description(mut self, description: impl IntoFragment<'a>) -> Self {
        self.description = Some(text::Text::new(description).size(text::size::SM));
        self
    }

    /// Add an action element (typically a button) shown at the top-right.
    pub fn action(mut self, action: impl Into<Slot<'a, Message>>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Set the style: a variant (`alert::Destructive`) or a tweaked `Spec`
    /// (`alert::Destructive + Bg(..)`).
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = style.into();
        self
    }

    /// Cap the alert's outer width. Defaults to no cap. Mirrors shadcn's
    /// `max-w-md` (448px) usage in the docs examples — most call sites
    /// pass `.max_width(448)` to keep alerts comfortably narrow.
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }
}

impl<'a, Message: 'a> From<Alert<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(a: Alert<'a, Message>) -> Self {
        Box::new(a).compose()
    }
}

impl<'a, Message: 'a> From<Alert<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Alert<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Alert<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Alert {
            icon,
            title,
            description,
            action,
            style,
            max_width,
        } = *self;

        let description = description.map(|t| {
            let style = style.clone();
            let style: StyleFn<'_, Theme> = Box::new(move |theme| Style {
                color: Some(style.description(theme).into()),
                ..Style::default()
            });
            t.style(style)
        });

        let mut text_col: iced_widget::Column<iced_core::Element<'a, Message, Theme, Renderer>> =
            iced_widget::Column::new().spacing(2);
        if let Some(title) = title {
            text_col = text_col.push(title.into());
        }
        if let Some(desc) = description {
            text_col = text_col.push(desc.into());
        }

        let body: iced_core::Element<'a, Message, Theme, Renderer> = if let Some(mut icon) = icon {
            icon.inherit(&Inherited::default().with_size(Size::Custom(16.0)));
            let icon_wrapper = iced_widget::container(icon.compose()).padding(Padding {
                top: 2.0,
                ..Padding::ZERO
            });
            iced_widget::Row::new()
                .spacing(8)
                .align_y(iced_core::Alignment::Start)
                .push(icon_wrapper.boxed())
                .push(text_col.boxed())
                .boxed()
        } else {
            text_col.boxed()
        };

        let card_style: iced_widget::container::StyleFn<'_, Theme> = style.into();

        let padding = if action.is_some() {
            Padding {
                top: 8.0,
                right: 72.0,
                bottom: 8.0,
                left: 10.0,
            }
        } else {
            Padding {
                top: 8.0,
                right: 10.0,
                bottom: 8.0,
                left: 10.0,
            }
        };

        let card = iced_widget::container(body)
            .padding(padding)
            .width(Fill)
            .style(card_style);

        let outer: iced_core::Element<'a, Message, Theme, Renderer> = match action {
            None => card.boxed(),
            Some(action) => {
                let overlay = iced_widget::container(action.compose())
                    .width(Fill)
                    .height(Fill)
                    .align_x(Horizontal::Right)
                    .align_y(Vertical::Top)
                    .padding(Padding {
                        top: 8.0,
                        right: 8.0,
                        bottom: 0.0,
                        left: 0.0,
                    });
                iced_widget::stack![card, overlay].boxed()
            }
        };

        // Self-cap the outer width when the caller set `.max_width(...)`.
        // `Fill` here is what makes the constraint bind — without it the
        // shrink-to-content behavior would ignore the cap. Skipped when
        // no cap is set so the alert keeps its natural shrink behavior.
        if max_width.is_finite() {
            iced_widget::container(outer)
                .width(Fill.max(max_width))
                .boxed()
        } else {
            outer
        }
    }
}
