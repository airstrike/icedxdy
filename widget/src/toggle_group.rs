//! Toggle group component.
//!
//! Mirrors shadcn's `<ToggleGroup>` / `<ToggleGroupItem>` at the iced layer:
//! callers provide selected state and messages, while the group applies the
//! nova toggle item styling and grouped edge-radius behavior.

use iced_core::Length::{self, Shrink};
use iced_widget::Renderer;
use icedxdy_core::theme::button;
use icedxdy_core::{Orientation, Theme};

use crate::button::group;
use crate::component::{Component, IntoElement};
use crate::{Button, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Outline,
}

pub struct Item<'a, Message> {
    content: Component<'a, Message>,
    selected: bool,
    on_press: Option<Message>,
    variant: Option<Variant>,
}

pub fn item<'a, Message: Clone + 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Item<'a, Message> {
    Item {
        content: content.into(),
        selected: false,
        on_press: None,
        variant: None,
    }
}

impl<'a, Message: Clone + 'a> Item<'a, Message> {
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }
}

pub struct ToggleGroup<'a, Message> {
    items: Vec<Item<'a, Message>>,
    variant: Variant,
    size: Size,
    orientation: Orientation,
    spacing: f32,
    width: Length,
}

pub fn toggle_group<'a, Message: Clone + 'a>(
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> ToggleGroup<'a, Message> {
    ToggleGroup {
        items: items.into_iter().collect(),
        variant: Variant::Default,
        size: Size::Default,
        orientation: Orientation::Horizontal,
        spacing: 0.0,
        width: Shrink,
    }
}

impl<'a, Message: Clone + 'a> ToggleGroup<'a, Message> {
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }

    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: Clone + 'a> From<ToggleGroup<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(group: ToggleGroup<'a, Message>) -> Self {
        Box::new(group).into_element()
    }
}

impl<'a, Message: Clone + 'a> From<ToggleGroup<'a, Message>> for Component<'a, Message> {
    fn from(group: ToggleGroup<'a, Message>) -> Self {
        Component::new(group)
    }
}

impl<'a, Message: Clone + 'a> IntoElement<Message> for ToggleGroup<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let ToggleGroup {
            items,
            variant,
            size,
            orientation,
            spacing,
            width,
        } = *self;

        let buttons: Vec<Button<'a, Message>> = items
            .into_iter()
            .map(|item| {
                let variant = item.variant.unwrap_or(variant);
                let mut button = crate::button::button(item.content)
                    .size(size)
                    .translate(false)
                    .style(style(variant, item.selected));
                if let Some(message) = item.on_press {
                    button = button.on_press(message);
                }
                button
            })
            .collect();

        if spacing == 0.0 {
            let items = buttons.into_iter().map(group::Item::from);
            group::group(items)
                .orientation(orientation)
                .width(width)
                .into()
        } else {
            let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
                buttons.into_iter().map(Into::into).collect();
            let element: iced_core::Element<'a, Message, Theme, Renderer> = match orientation {
                Orientation::Horizontal => iced_widget::Row::with_children(elements)
                    .spacing(spacing)
                    .align_y(iced_core::Alignment::Center)
                    .into(),
                Orientation::Vertical => iced_widget::Column::with_children(elements)
                    .spacing(spacing)
                    .width(width)
                    .into(),
            };

            match orientation {
                Orientation::Horizontal => iced_widget::container(element).width(width).into(),
                Orientation::Vertical => element,
            }
        }
    }
}

fn style<'a>(
    variant: Variant,
    selected: bool,
) -> iced_widget::button::StyleFn<'a, icedxdy_core::Theme> {
    Box::new(move |theme, status| {
        let mut style: iced_widget::button::Style = match variant {
            Variant::Default => button::ghost(theme, status).into(),
            Variant::Outline => button::outline(theme, status).into(),
        };

        if selected {
            style.background = Some(theme.muted.base.into());
            style.text_color = theme.neutral.foreground.into();
        }

        style
    })
}
