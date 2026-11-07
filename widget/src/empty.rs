//! Empty-state component slots.
//!
//! Mirrors shadcn's `Empty` family: a centered empty-state body, optional
//! media, title/description header, and trailing content slot.

use iced_core::Length::Fill;
use iced_core::Padding;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::text::LineHeight;
use iced_widget::Renderer;
use iced_widget::Widget;
use icedxdy_core::theme::container::Style;
use icedxdy_core::{Border, Gap, Theme, radius};

use crate::component::{Component, IntoElement};
use crate::{Size, text};

/// Empty media variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Icon,
}

/// Empty-state root.
pub struct Empty<'a, Message> {
    media: Option<Component<'a, Message>>,
    title: Option<Component<'a, Message>>,
    description: Option<Component<'a, Message>>,
    content: Option<Component<'a, Message>>,
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
    pub fn media(mut self, media: impl Into<Component<'a, Message>>) -> Self {
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
    pub fn content(mut self, content: impl Into<Component<'a, Message>>) -> Self {
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
        Box::new(empty).into_element()
    }
}

impl<'a, Message: 'a> From<Empty<'a, Message>> for Component<'a, Message> {
    fn from(empty: Empty<'a, Message>) -> Self {
        Component::new(empty)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Empty<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
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
            body = body.push(media.into_iced());
        }

        if title.is_some() || description.is_some() {
            body = body.push(header(title, description));
        }

        if let Some(content) = content {
            let element: iced_core::Element<'a, Message, Theme, Renderer> =
                iced_widget::container(content.into_iced())
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
    content: Component<'a, Message>,
    variant: Variant,
}

/// Create an empty media slot.
pub fn media<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Media<'a, Message> {
    Media {
        content: content.into(),
        variant: Variant::Default,
    }
}

/// Create an icon-styled empty media slot.
pub fn media_icon<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Media<'a, Message> {
    media(content).variant(Variant::Icon)
}

impl<'a, Message: 'a> Media<'a, Message> {
    /// Set the media variant.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
}

impl<'a, Message: 'a> From<Media<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(media: Media<'a, Message>) -> Self {
        Box::new(media).into_element()
    }
}

impl<'a, Message: 'a> From<Media<'a, Message>> for Component<'a, Message> {
    fn from(media: Media<'a, Message>) -> Self {
        Component::new(media)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Media<'a, Message> {
    fn size(&mut self, size: Size) {
        self.content.size(size);
    }

    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Media {
            mut content,
            variant,
        } = *self;

        if matches!(variant, Variant::Icon) {
            content.size(Size::Default);
        }

        let mut inner = iced_widget::container(content.into_iced())
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        if matches!(variant, Variant::Icon) {
            inner = inner
                .width(32.0)
                .height(32.0)
                .style(|theme| media_icon_style(theme).into());
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
    fn into_title(self) -> Component<'a, Message>;
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for &'a str {
    fn into_title(self) -> Component<'a, Message> {
        title(self).into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for String {
    fn into_title(self) -> Component<'a, Message> {
        title(self).into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for text::Text<'a> {
    fn into_title(self) -> Component<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for Component<'a, Message> {
    fn into_title(self) -> Component<'a, Message> {
        self
    }
}

/// Convert a value into an empty description.
pub trait IntoDescription<'a, Message> {
    fn into_description(self) -> Component<'a, Message>;
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for &'a str {
    fn into_description(self) -> Component<'a, Message> {
        description(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for String {
    fn into_description(self) -> Component<'a, Message> {
        description(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for text::Text<'a> {
    fn into_description(self) -> Component<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for Component<'a, Message> {
    fn into_description(self) -> Component<'a, Message> {
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
    title: Option<Component<'a, Message>>,
    description: Option<Component<'a, Message>>,
) -> iced_core::Element<'a, Message, Theme, Renderer> {
    let mut column = iced_widget::Column::new()
        .spacing(Gap::G2)
        .align_x(Horizontal::Center)
        .width(Fill);

    if let Some(title) = title {
        column = column.push(title.into_iced());
    }
    if let Some(description) = description {
        column = column.push(description.into_iced());
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

fn media_icon_style(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.neutral.foreground.into()),
        background: Some(theme.muted.base.into()),
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}
