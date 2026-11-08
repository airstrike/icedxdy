//! Menu content: the items, labels, separators and submenus shared by
//! [`DropdownMenu`](crate::dropdown_menu) and
//! [`ContextMenu`](crate::context_menu) (Radix's `Menu` parts, nova style).
//!
//! ```ignore
//! use icedxdy::widget::menu;
//!
//! column![
//!     menu::group([
//!         menu::label("My Account"),
//!         menu::item("Profile").shortcut("⇧⌘P").into(),
//!         menu::item("Billing").on_select(Message::Billing).into(),
//!     ]),
//!     menu::separator(),
//!     menu::item("Log out").style(menu::Destructive).into(),
//!     menu::item("API").disabled(true).into(),
//! ]
//! ```
//!
//! Selecting an item publishes its own message and closes the menu, as
//! Radix's `onSelect` does; a disabled item does neither.

use std::time::Duration;

use iced_core::Length::Fill;
use iced_core::text::IntoFragment;
use iced_core::{Alignment, Element, Padding};
use iced_widget::{Renderer, Space, Widget, anchored};
use icedxdy_core::Theme;
use icedxdy_core::theme::menu::item;
use icedxdy_core::theme::popover::content;

use crate::button::widget::{self, Button};
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker::{self, Press};
use crate::popover::Signal;
use crate::{column, row, text};

pub use icedxdy_core::theme::menu::item::{Destructive, Variant};

/// A `MenuItem`: a row that publishes its message and closes the menu
/// when selected.
pub struct Item<'a, Message> {
    content: Slot<'a, Message>,
    shortcut: Option<&'a str>,
    on_select: Option<Message>,
    disabled: bool,
    inset: bool,
    variant: Variant,
    indicator: Indicator,
    closes: bool,
}

/// What an item shows in its trailing indicator slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Indicator {
    /// A plain item: no slot.
    None,
    /// A checkbox or radio item: a check while checked.
    Check(bool),
    /// A submenu trigger: a chevron.
    Chevron,
}

/// Create a [`menu::Item`](Item) showing `content`.
pub fn item<'a, Message>(content: impl Into<Slot<'a, Message>>) -> Item<'a, Message> {
    Item {
        content: content.into(),
        shortcut: None,
        on_select: None,
        disabled: false,
        inset: false,
        variant: Variant::Default,
        indicator: Indicator::None,
        closes: true,
    }
}

/// A `MenuCheckboxItem`: shows a check while `checked`; selecting it
/// publishes `on_checked_change` with the flipped value.
pub fn checkbox_item<'a, Message>(
    content: impl Into<Slot<'a, Message>>,
    checked: bool,
    on_checked_change: impl Fn(bool) -> Message,
) -> Item<'a, Message> {
    Item {
        on_select: Some(on_checked_change(!checked)),
        indicator: Indicator::Check(checked),
        ..item(content)
    }
}

/// A `MenuRadioItem` for `value`: shows a check while `selected` holds it;
/// selecting it publishes `on_select(value)`.
pub fn radio_item<'a, Message, V>(
    content: impl Into<Slot<'a, Message>>,
    value: V,
    selected: Option<V>,
    on_select: impl Fn(V) -> Message,
) -> Item<'a, Message>
where
    V: Eq + Copy,
{
    Item {
        on_select: Some(on_select(value)),
        indicator: Indicator::Check(selected == Some(value)),
        ..item(content)
    }
}

impl<'a, Message> Item<'a, Message> {
    /// Show a `MenuShortcut` at the end of the item.
    pub fn shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    /// Set the message published when the item is selected.
    pub fn on_select(mut self, message: Message) -> Self {
        self.on_select = Some(message);
        self
    }

    /// Disable the item: it dims and ignores the pointer.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Indent the item to line up with items that have an icon
    /// (`inset`).
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// Whether selecting the item closes the menu through its marker (a
    /// select closes itself instead).
    pub(crate) fn closes(mut self, closes: bool) -> Self {
        self.closes = closes;
        self
    }

    /// Set the item's variant (`.style(menu::Destructive)`).
    pub fn style(mut self, variant: impl Into<Variant>) -> Self {
        self.variant = variant.into();
        self
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Item<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.content.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Item {
            content,
            shortcut,
            on_select,
            disabled,
            inset,
            variant,
            indicator,
            closes,
        } = *self;

        let mut line = row![content].spacing(6).align_y(Alignment::Center);

        line = line.push(Slot::from(Space::new().width(Fill)));

        if let Some(shortcut) = shortcut {
            // `ml-auto text-xs tracking-widest text-muted-foreground`.
            line = line.push(Slot::from(text::muted(shortcut).size(text::size::XS)));
        }

        match indicator {
            Indicator::None => {}
            Indicator::Check(checked) => {
                line = line.push(Slot::from(
                    Container::new(if checked {
                        Slot::from(text::small("\u{2713}"))
                    } else {
                        Slot::from(Space::new())
                    })
                    .width(16),
                ));
            }
            Indicator::Chevron => {
                line = line.push(Slot::from(text::small("\u{203a}")));
            }
        }

        // `px-1.5 py-1`, `pl-7` when inset, and a check item's `pr-8`
        // holding its indicator at `right-2`.
        let padding = Padding {
            top: 4.0,
            bottom: 4.0,
            left: if inset { 28.0 } else { 6.0 },
            right: if let Indicator::Check(_) = indicator {
                8.0
            } else {
                6.0
            },
        };

        let style: widget::StyleFn<'b, Theme> =
            Box::new(move |theme: &Theme, status: widget::Status| {
                item::style(theme, status.into(), variant).into()
            });

        let button = Button::new(Slot::from(line).compose())
            .padding(padding)
            .width(Fill)
            .translate(false)
            .disabled(disabled)
            .on_press_maybe(on_select)
            .style(style);

        if closes {
            marker::marker(button.boxed(), Signal::Close, Press::Captured).into()
        } else {
            button.boxed()
        }
    }
}

impl<'a, Message: Clone + 'a> From<Item<'a, Message>> for Slot<'a, Message> {
    fn from(item: Item<'a, Message>) -> Self {
        Slot::new(item)
    }
}

/// A `MenuLabel`: a muted `text-xs font-medium` heading.
pub fn label<'a, Message: 'a>(label: impl IntoFragment<'a>) -> Slot<'a, Message> {
    Container::new(text::muted(label).size(text::size::XS).medium())
        .padding([4, 6])
        .into()
}

/// A `MenuSeparator`: a 1px rule with `my-1`.
pub fn separator<'a, Message: 'a>() -> Slot<'a, Message> {
    Container::new(crate::separator::separator())
        .padding([4, 0])
        .into()
}

/// A `MenuGroup`: its children stacked.
pub fn group<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    column::Column::with_children(children).width(Fill).into()
}

/// A `MenuSub`: an item labelled `content` that opens `items` beside it
/// while hovered. Selecting an item inside closes the whole menu.
pub fn sub<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
    items: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    Slot::new(Sub {
        trigger: Item {
            indicator: Indicator::Chevron,
            closes: false,
            ..item(content)
        },
        items: group(items),
    })
}

/// A submenu: its trigger item and its panel.
struct Sub<'a, Message> {
    trigger: Item<'a, Message>,
    items: Slot<'a, Message>,
}

impl<'a, Message: Clone + 'a> Compose<Message> for Sub<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.trigger.inherit(from);
        self.items.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        // `MenuSubContent`: `min-w-[96px] p-1 shadow-lg`, beside the
        // trigger, its first item level with it.
        let panel = Container::new(self.items)
            .padding(4)
            .width(Fill)
            .style(content);

        let sub = anchored(
            Slot::from(self.trigger).compose(),
            Slot::from(panel).compose(),
        )
        .side(anchored::Side::Right)
        .align(anchored::Align::Start)
        .offset(4)
        .align_offset(-4.0)
        .min_width(96)
        .hover(Duration::ZERO, Duration::from_millis(150));

        marker::relay::<Signal, _, _, _>(sub.boxed()).into()
    }
}
