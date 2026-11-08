//! Empty-state component slots.
//!
//! Mirrors shadcn's `Empty` family: a centered empty-state body, optional
//! media, title/description header, and trailing content slot.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::empty;
//!
//! empty::empty()
//!     .media(empty::media(icon::folder_code()).style(empty::Icon))
//!     .title("No Projects Yet")
//!     .description("You haven't created any projects yet.")
//! ```
//!
//! # Variants
//!
//! shadcn's `EmptyMedia` `variant` prop is a style: `.style(empty::Icon)`.

use iced_core::Length::Fill;
use iced_core::Padding;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::text::LineHeight;
use iced_widget::Renderer;
use iced_widget::Widget;
use icedxdy_core::theme::container::Style;
use icedxdy_core::theme::media::Frame;
use icedxdy_core::{Border, Gap, Theme, radius};

use crate::compose::{Compose, Inherited, Slot};
use crate::{Size, text};

pub use icedxdy_core::theme::empty::Icon;

/// Empty-state root.
pub struct Empty<'a, Message> {
    media: Option<Slot<'a, Message>>,
    title: Option<Slot<'a, Message>>,
    description: Option<Slot<'a, Message>>,
    content: Option<Slot<'a, Message>>,
    padding: Padding,
}

/// Create an empty-state root.
pub fn empty<'a, Message: 'a>() -> Empty<'a, Message> {
    Empty {
        media: None,
        title: None,
        description: None,
        content: None,
        padding: Padding::new(24.0),
    }
}

impl<'a, Message: 'a> Empty<'a, Message> {
    /// Set the media slot.
    pub fn media(mut self, media: impl Into<Slot<'a, Message>>) -> Self {
        self.media = Some(media.into());
        self
    }

    /// Set the title slot.
    pub fn title(mut self, title: impl IntoTitle<'a, Message>) -> Self {
        self.title = Some(title.into_title());
        self
    }

    /// Set the description slot.
    pub fn description(mut self, description: impl IntoDescription<'a, Message>) -> Self {
        self.description = Some(description.into_description());
        self
    }

    /// Set the content slot.
    pub fn content(mut self, content: impl Into<Slot<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Override root padding. shadcn's default is `p-6`; many blocks pass
    /// class overrides like `p-4`.
    pub fn padding(mut self, padding: impl Into<icedxdy_core::Padding>) -> Self {
        let padding: icedxdy_core::Padding = padding.into();
        self.padding = padding.into();
        self
    }
}

impl<'a, Message: 'a> From<Empty<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(empty: Empty<'a, Message>) -> Self {
        Box::new(empty).compose()
    }
}

impl<'a, Message: 'a> From<Empty<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Empty<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Empty<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Empty {
            media,
            title,
            description,
            content,
            padding,
        } = *self;

        let mut body = iced_widget::Column::new()
            .spacing(Gap::G4)
            .align_x(Horizontal::Center)
            .width(Fill);

        if let Some(media) = media {
            body = body.push(media.compose());
        }

        if title.is_some() || description.is_some() {
            body = body.push(header(title, description));
        }

        if let Some(content) = content {
            let element: iced_core::Element<'a, Message, Theme, Renderer> =
                iced_widget::container(content.compose())
                    .center_x(Fill.max(384.0))
                    .boxed();
            body = body.push(element);
        }

        iced_widget::container(body)
            .padding(padding)
            .width(Fill)
            .center_y(Fill)
            .style(|theme| empty_style(theme).into())
            .boxed()
    }
}

/// Empty media slot.
pub struct Media<'a, Message> {
    content: Slot<'a, Message>,
    frame: Option<Frame>,
}

/// Create an unframed empty media slot.
pub fn media<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Media<'a, Message> {
    Media {
        content: content.into(),
        frame: None,
    }
}

impl<'a, Message: 'a> Media<'a, Message> {
    /// Set the media style: `empty::Icon` puts the content on a muted tile.
    pub fn style(mut self, frame: impl Into<Frame>) -> Self {
        self.frame = Some(frame.into());
        self
    }
}

impl<'a, Message: 'a> From<Media<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(media: Media<'a, Message>) -> Self {
        Box::new(media).compose()
    }
}

impl<'a, Message: 'a> From<Media<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Media<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Media<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.content.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Media { mut content, frame } = *self;

        // A framed tile sizes its icon (`[&_svg:not([class*='size-'])]:size-4`).
        if frame.as_ref().is_some_and(|frame| frame.tile().is_some()) {
            content.inherit(&Inherited::default().with_size(Size::Default));
        }

        let mut inner = iced_widget::container(content.compose())
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        if let Some(frame) = frame {
            if let Some(tile) = frame.tile() {
                inner = inner.width(tile).height(tile);
            }

            let paint: iced_widget::container::StyleFn<'_, Theme> = frame.paint().into();
            inner = inner.clip(frame.clip()).style(paint);
        }

        iced_widget::container(inner)
            .padding(Padding {
                bottom: 8.0,
                ..Padding::ZERO
            })
            .boxed()
    }
}

/// Convert a value into an empty title.
pub trait IntoTitle<'a, Message> {
    fn into_title(self) -> Slot<'a, Message>;
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for &'a str {
    fn into_title(self) -> Slot<'a, Message> {
        title(self).into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for String {
    fn into_title(self) -> Slot<'a, Message> {
        title(self).into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for text::Text<'a> {
    fn into_title(self) -> Slot<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for Slot<'a, Message> {
    fn into_title(self) -> Slot<'a, Message> {
        self
    }
}

/// Convert a value into an empty description.
pub trait IntoDescription<'a, Message> {
    fn into_description(self) -> Slot<'a, Message>;
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for &'a str {
    fn into_description(self) -> Slot<'a, Message> {
        description(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for String {
    fn into_description(self) -> Slot<'a, Message> {
        description(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for text::Text<'a> {
    fn into_description(self) -> Slot<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for Slot<'a, Message> {
    fn into_description(self) -> Slot<'a, Message> {
        self
    }
}

/// Empty title typography.
pub fn title<'a>(content: impl iced_core::text::IntoFragment<'a>) -> text::Text<'a> {
    text::small(content).medium().letter_spacing(-0.025)
}

/// Empty description typography.
pub fn description<'a>(content: impl iced_core::text::IntoFragment<'a>) -> text::Text<'a> {
    text::muted(content).line_height(LineHeight::Relative(1.625))
}

fn header<'a, Message: 'a>(
    title: Option<Slot<'a, Message>>,
    description: Option<Slot<'a, Message>>,
) -> iced_core::Element<'a, Message, Theme, Renderer> {
    let mut column = iced_widget::Column::new()
        .spacing(Gap::G2)
        .align_x(Horizontal::Center)
        .width(Fill);

    if let Some(title) = title {
        column = column.push(title.compose());
    }
    if let Some(description) = description {
        column = column.push(description.compose());
    }

    iced_widget::container(column)
        .center_x(Fill.max(384.0))
        .boxed()
}

fn empty_style(_theme: &Theme) -> Style {
    Style {
        border: Border {
            radius: radius::XL.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}
