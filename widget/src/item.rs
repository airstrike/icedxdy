//! Item component slots.
//!
//! Mirrors shadcn's `Item` primitive for compact rows or muted panels inside
//! cards.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::item;
//!
//! item::item([
//!     item::media(icon::badge_check()).style(item::Icon).into(),
//!     item::content([item::title("Your profile has been verified.").into()]).into(),
//! ])
//! .style(item::Outline)
//! ```
//!
//! # Variants
//!
//! shadcn's `variant` props are styles: `.style(item::Outline)` and
//! `.style(item::Muted)` on the item, `.style(item::Icon)` and
//! `.style(item::Image)` on its media.

use iced_core::Length::{self, Fill};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::text::LineHeight;
use iced_core::{Alignment, Padding};
use iced_widget::Renderer;
use iced_widget::Widget;
use icedxdy_core::theme::container::Spec;
use icedxdy_core::theme::media::Frame;
use icedxdy_core::{Gap, Orientation, Theme};

use crate::compose::{Compose, Slot};
use crate::text;

pub use icedxdy_core::theme::item::{Default, Icon, Image, Muted, Outline};

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
    children: Vec<Slot<'a, Message>>,
    style: Spec,
    size: Size,
    orientation: Orientation,
}

/// Create an item from children.
pub fn item<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Item<'a, Message> {
    Item {
        children: children.into_iter().collect(),
        style: Default.into(),
        size: Size::Default,
        orientation: Orientation::Horizontal,
    }
}

impl<'a, Message: 'a> Item<'a, Message> {
    /// Set the style: a variant (`item::Outline`, `item::Muted`) or a
    /// tweaked `Spec`.
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = style.into();
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
        Box::new(item).compose()
    }
}

impl<'a, Message: 'a> From<Item<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Item<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Item<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Item {
            children,
            style,
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
            children.into_iter().map(Slot::compose).collect();

        let content: iced_core::Element<'a, Message, Theme, Renderer> = match orientation {
            Orientation::Horizontal => iced_widget::Row::with_children(elements)
                .spacing(spacing)
                .align_y(Alignment::Center)
                .width(Fill)
                .boxed(),
            Orientation::Vertical => iced_widget::Column::with_children(elements)
                .spacing(spacing)
                .width(Fill)
                .boxed(),
        };

        crate::container(content)
            .padding(padding)
            .width(Fill)
            .class(style)
            .into()
    }
}

/// Item content slot.
pub struct Content<'a, Message> {
    children: Vec<Slot<'a, Message>>,
    spacing: f32,
    width: Length,
}

/// Create an item content slot.
pub fn content<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
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
        Box::new(content).compose()
    }
}

impl<'a, Message: 'a> From<Content<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Content<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Content<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Content {
            children,
            spacing,
            width,
        } = *self;

        let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
            children.into_iter().map(Slot::compose).collect();

        iced_widget::Column::with_children(elements)
            .spacing(spacing)
            .width(width)
            .boxed()
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

/// Item media slot.
pub struct Media<'a, Message> {
    content: Slot<'a, Message>,
    frame: Option<Frame>,
}

/// Create an unframed item media slot.
pub fn media<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Media<'a, Message> {
    Media {
        content: content.into(),
        frame: None,
    }
}

impl<'a, Message: 'a> Media<'a, Message> {
    /// Set the media style: `item::Icon` or `item::Image`.
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
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Media { content, frame } = *self;
        let container = crate::container(content.compose())
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let Some(frame) = frame else {
            return container.into();
        };

        let container = match frame.tile() {
            Some(tile) => container.width(tile).height(tile),
            None => container,
        };

        container.clip(frame.clip()).class(frame.paint()).into()
    }
}

/// Item actions row.
pub fn actions<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
        children.into_iter().map(Slot::compose).collect();

    let element: iced_core::Element<'a, Message, Theme, Renderer> =
        iced_widget::Row::with_children(elements)
            .spacing(Gap::G2)
            .align_y(Alignment::Center)
            .boxed();

    Slot::from(element)
}

/// Item header row.
pub fn header<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    slot_row(children)
}

/// Item footer row.
pub fn footer<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    slot_row(children)
}

fn slot_row<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
        children.into_iter().map(Slot::compose).collect();

    let element: iced_core::Element<'a, Message, Theme, Renderer> =
        iced_widget::Row::with_children(elements)
            .spacing(Gap::G2)
            .align_y(Alignment::Center)
            .width(Fill)
            .boxed();

    Slot::from(element)
}
