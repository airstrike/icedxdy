//! Toggle group component.
//!
//! Mirrors shadcn's `<ToggleGroup>` / `<ToggleGroupItem>` at the iced layer:
//! callers provide selected state and messages, while the group applies the
//! nova toggle item styling and grouped edge-radius behavior.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::toggle_group;
//!
//! toggle_group::toggle_group([
//!     toggle_group::item(icon::bold()).selected(true).on_press(Message::Bold),
//!     toggle_group::item(icon::italic()).on_press(Message::Italic),
//! ])
//! .style(toggle_group::Outline)
//! ```
//!
//! # Variants
//!
//! shadcn's `variant` prop is a style: `.style(toggle_group::Outline)` on
//! the group, or on one item to override the group's.

use iced_core::Length::{self, Shrink};
use iced_widget::Renderer;
use iced_widget::Widget;
use iced_widget::button::StyleFn;
use icedxdy_core::theme::button::Spec;
use icedxdy_core::theme::toggle;
use icedxdy_core::{Orientation, Theme};

use crate::button::group;
use crate::compose::{Compose, Slot};
use crate::{Button, Size};

pub use icedxdy_core::theme::toggle::{Default, Outline};

/// One toggle in a [`ToggleGroup`].
pub struct Item<'a, Message> {
    content: Slot<'a, Message>,
    selected: bool,
    on_press: Option<Message>,
    style: Option<Spec>,
}

/// Create a toggle group item showing `content`.
pub fn item<'a, Message: Clone + 'a>(content: impl Into<Slot<'a, Message>>) -> Item<'a, Message> {
    Item {
        content: content.into(),
        selected: false,
        on_press: None,
        style: None,
    }
}

impl<'a, Message: Clone + 'a> Item<'a, Message> {
    /// Show the item pressed on.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Publish `message` when the item is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Override the group's style for this item.
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = Some(style.into());
        self
    }
}

/// A row (or column) of toggles sharing one style and size.
pub struct ToggleGroup<'a, Message> {
    items: Vec<Item<'a, Message>>,
    style: Spec,
    size: Size,
    orientation: Orientation,
    spacing: f32,
    width: Length,
}

/// Create a [`ToggleGroup`] of `items`.
pub fn toggle_group<'a, Message: Clone + 'a>(
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> ToggleGroup<'a, Message> {
    ToggleGroup {
        items: items.into_iter().collect(),
        style: Default.into(),
        size: Size::Default,
        orientation: Orientation::Horizontal,
        spacing: 0.0,
        width: Shrink,
    }
}

impl<'a, Message: Clone + 'a> ToggleGroup<'a, Message> {
    /// Set the style of every item: a variant (`toggle_group::Outline`) or
    /// a tweaked `Spec`.
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = style.into();
        self
    }

    /// Set the size of every item.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Lay the items out in a row or a column.
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }

    /// Set the gap between items. At `0` the items join into one segmented
    /// control.
    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Set the group's width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: Clone + 'a> From<ToggleGroup<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(group: ToggleGroup<'a, Message>) -> Self {
        Box::new(group).compose()
    }
}

impl<'a, Message: Clone + 'a> From<ToggleGroup<'a, Message>> for Slot<'a, Message> {
    fn from(widget: ToggleGroup<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for ToggleGroup<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let ToggleGroup {
            items,
            style,
            size,
            orientation,
            spacing,
            width,
        } = *self;

        let buttons: Vec<Button<'a, Message>> = items
            .into_iter()
            .map(|item| {
                let spec = item.style.unwrap_or_else(|| style.clone());
                let selected = item.selected;
                let paint: StyleFn<'a, Theme> = Box::new(move |theme, status| {
                    let style = spec.resolve(theme, status);

                    if selected {
                        toggle::on(theme, style).into()
                    } else {
                        style.into()
                    }
                });
                let mut button = crate::button::button(item.content)
                    .size(size)
                    .translate(false)
                    .style(paint);
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
                    .boxed(),
                Orientation::Vertical => iced_widget::Column::with_children(elements)
                    .spacing(spacing)
                    .width(width)
                    .boxed(),
            };

            match orientation {
                Orientation::Horizontal => iced_widget::container(element).width(width).boxed(),
                Orientation::Vertical => element,
            }
        }
    }
}
