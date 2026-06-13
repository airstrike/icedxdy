//! Item component slots.
//!
//! Mirrors shadcn's `Item` primitive for compact rows or muted panels inside
//! cards.

use iced_core::Length::{self, Fill};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::text::LineHeight;
use iced_core::{Alignment, Padding};
use iced_widget::Renderer;
use icedxdy_core::theme::container::Style;
use icedxdy_core::{Border, Color, Gap, Orientation, Theme, radius};

use crate::component::{Component, IntoElement};
use crate::text;

/// Item variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Outline,
    Muted,
}

/// Item sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    #[default]
    Default,
    Sm,
    Xs,
}

/// Item root.
pub struct Item<'a, Message> {
    children: Vec<Component<'a, Message>>,
    variant: Variant,
    size: Size,
    orientation: Orientation,
}

/// Create an item from children.
pub fn item<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Item<'a, Message> {
    Item {
        children: children.into_iter().collect(),
        variant: Variant::Default,
        size: Size::Default,
        orientation: Orientation::Horizontal,
    }
}

/// Create a muted item from children.
pub fn muted<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Item<'a, Message> {
    item(children).variant(Variant::Muted)
}

/// Create an outline item from children.
pub fn outline<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Item<'a, Message> {
    item(children).variant(Variant::Outline)
}

impl<'a, Message: 'a> Item<'a, Message> {
    /// Set the item variant.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the item size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Set row vs column layout.
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }
}

impl<'a, Message: 'a> From<Item<'a, Message>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(item: Item<'a, Message>) -> Self {
        Box::new(item).into_element()
    }
}

impl<'a, Message: 'a> From<Item<'a, Message>> for Component<'a, Message> {
    fn from(item: Item<'a, Message>) -> Self {
        Component::new(item)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Item<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Item {
            children,
            variant,
            size,
            orientation,
        } = *self;

        let spacing = match size {
            Size::Xs => Gap::G2,
            Size::Default | Size::Sm => Gap(10.0),
        };

        let padding = match size {
            Size::Xs => Padding {
                top: 8.0,
                bottom: 8.0,
                left: 10.0,
                right: 10.0,
            },
            Size::Default | Size::Sm => Padding {
                top: 10.0,
                bottom: 10.0,
                left: 12.0,
                right: 12.0,
            },
        };

        let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
            children.into_iter().map(Component::into_iced).collect();

        let content: iced_core::Element<'a, Message, Theme, Renderer> = match orientation {
            Orientation::Horizontal => iced_widget::Row::with_children(elements)
                .spacing(spacing)
                .align_y(Alignment::Center)
                .width(Fill)
                .into(),
            Orientation::Vertical => iced_widget::Column::with_children(elements)
                .spacing(spacing)
                .width(Fill)
                .into(),
        };

        crate::container(content)
            .padding(padding)
            .width(Fill)
            .style(style(variant))
            .into()
    }
}

/// Item content slot.
pub struct Content<'a, Message> {
    children: Vec<Component<'a, Message>>,
    spacing: f32,
    width: Length,
}

/// Create an item content slot.
pub fn content<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Content<'a, Message> {
    Content {
        children: children.into_iter().collect(),
        spacing: 4.0,
        width: Fill,
    }
}

impl<'a, Message: 'a> Content<'a, Message> {
    /// Set slot spacing.
    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Set slot width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: 'a> From<Content<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(content: Content<'a, Message>) -> Self {
        Box::new(content).into_element()
    }
}

impl<'a, Message: 'a> From<Content<'a, Message>> for Component<'a, Message> {
    fn from(content: Content<'a, Message>) -> Self {
        Component::new(content)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Content<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Content {
            children,
            spacing,
            width,
        } = *self;

        let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
            children.into_iter().map(Component::into_iced).collect();

        iced_widget::Column::with_children(elements)
            .spacing(spacing)
            .width(width)
            .into()
    }
}

/// Item title typography.
pub fn title<'a>(content: impl iced_core::text::IntoFragment<'a>) -> text::Text<'a> {
    text::small(content)
        .medium()
        .line_height(LineHeight::Relative(1.375))
}

/// Item description typography.
pub fn description<'a>(content: impl iced_core::text::IntoFragment<'a>) -> text::Text<'a> {
    text::muted(content).line_height(LineHeight::Relative(1.5))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

/// Item media slot.
pub struct Media<'a, Message> {
    content: Component<'a, Message>,
    variant: MediaVariant,
}

/// Create an item media slot.
pub fn media<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Media<'a, Message> {
    Media {
        content: content.into(),
        variant: MediaVariant::Default,
    }
}

/// Create an icon item media slot.
pub fn media_icon<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Media<'a, Message> {
    media(content).variant(MediaVariant::Icon)
}

/// Create an image item media slot.
pub fn media_image<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Media<'a, Message> {
    media(content).variant(MediaVariant::Image)
}

impl<'a, Message: 'a> Media<'a, Message> {
    pub fn variant(mut self, variant: MediaVariant) -> Self {
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
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Media { content, variant } = *self;
        let content = content.into_iced();

        match variant {
            MediaVariant::Default | MediaVariant::Icon => crate::container(content)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into(),
            MediaVariant::Image => crate::container(content)
                .width(40.0)
                .height(40.0)
                .clip(true)
                .style(media_image_style)
                .into(),
        }
    }
}

/// Item actions row.
pub fn actions<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Component<'a, Message> {
    let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
        children.into_iter().map(Component::into_iced).collect();

    let element: iced_core::Element<'a, Message, Theme, Renderer> =
        iced_widget::Row::with_children(elements)
            .spacing(Gap::G2)
            .align_y(Alignment::Center)
            .into();

    Component::from(element)
}

/// Item header row.
pub fn header<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Component<'a, Message> {
    slot_row(children)
}

/// Item footer row.
pub fn footer<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Component<'a, Message> {
    slot_row(children)
}

fn slot_row<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Component<'a, Message> {
    let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
        children.into_iter().map(Component::into_iced).collect();

    let element: iced_core::Element<'a, Message, Theme, Renderer> =
        iced_widget::Row::with_children(elements)
            .spacing(Gap::G2)
            .align_y(Alignment::Center)
            .width(Fill)
            .into();

    Component::from(element)
}

fn media_image_style(_theme: &Theme) -> Style {
    Style {
        border: Border {
            radius: radius::SM.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

fn style(variant: Variant) -> impl Fn(&Theme) -> Style + 'static {
    move |theme| {
        let mut style = Style {
            border: Border {
                radius: radius::LG.into(),
                width: 1.0,
                color: Color::TRANSPARENT,
            },
            ..Style::default()
        };

        match variant {
            Variant::Default => {}
            Variant::Outline => {
                style.border.color = theme.neutral.border.into();
            }
            Variant::Muted => {
                style.background = Some(theme.muted.base.alpha(0.5).into());
            }
        }

        style
    }
}
