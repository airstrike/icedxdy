// Intentional skips: item-link, item-image, item-group, item-header,
// item-avatar and item-dropdown (`asChild` links, remote images, avatar
// images, grid groups and DropdownMenu are not ported) and item-rtl (RTL).
// item-demo's second item is an `asChild` link; here it renders as a plain
// item.

use iced::{Fill, Task};

use icedxdy::widget::{Column, button, column, item};
use icedxdy::{Size, Slot, Theme};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Item",
    description: "A versatile component that you can use to display any content.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/item-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "variant",
                title: "Variant",
                description: Some("Use `.style(..)` to change the visual style of the item."),
                snippet: "examples/browser/snippets/item-variant.md",
            },
            page::Block::Example {
                id: "size",
                title: "Size",
                description: Some("Use `.size(..)` to change the size of the item."),
                snippet: "examples/browser/snippets/item-size.md",
            },
            page::Block::Example {
                id: "icon",
                title: "Icon",
                description: Some("Use `item::media(..).style(item::Icon)` to display an icon."),
                snippet: "examples/browser/snippets/item-icon.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/item-api-reference.md"),
];

pub struct ItemPage {
    state: page::State,
}

impl ItemPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// An item with an inbox icon, a title and a description.
fn inbox<'a>(title: &'a str, description: &'a str) -> item::Item<'a, Message> {
    item::item([
        item::media(icon::inbox()).style(item::Icon).into(),
        item::content([
            item::title(title).into(),
            item::description(description).into(),
        ])
        .into(),
    ])
}

/// The `max-w-md` column the examples stack their items in.
fn stack<'a>(
    items: impl IntoIterator<Item = Slot<'a, Message>>,
) -> iced::Element<'a, Message, Theme> {
    Column::with_children(items)
        .spacing(24)
        .width(Fill)
        .max_width(448.0)
        .into()
}

impl page::Page for ItemPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // shadcn `item-demo.tsx`.
            "demo" => stack([
                item::item([
                    item::content([
                        item::title("Basic Item").into(),
                        item::description("A simple item with title and description.").into(),
                    ])
                    .into(),
                    item::actions([button("Action")
                        .style(button::Outline)
                        .size(Size::Sm)
                        .into()]),
                ])
                .style(item::Outline)
                .into(),
                item::item([
                    item::media(icon::badge_check().size(20)).into(),
                    item::content([item::title("Your profile has been verified.").into()]).into(),
                    item::actions([icon::chevron_right().size(16).into()]),
                ])
                .style(item::Outline)
                .size(item::Size::Sm)
                .into(),
            ]),

            // shadcn `item-variant.tsx`.
            "variant" => stack([
                inbox("Default Variant", "Transparent background with no border.").into(),
                inbox("Outline Variant", "Outlined style with a visible border.")
                    .style(item::Outline)
                    .into(),
                inbox("Muted Variant", "Muted background for secondary content.")
                    .style(item::Muted)
                    .into(),
            ]),

            // shadcn `item-size.tsx`.
            "size" => stack([
                inbox("Default Size", "The standard size for most use cases.")
                    .style(item::Outline)
                    .into(),
                inbox("Small Size", "A compact size for dense layouts.")
                    .style(item::Outline)
                    .size(item::Size::Sm)
                    .into(),
                inbox("Extra Small Size", "The most compact size available.")
                    .style(item::Outline)
                    .size(item::Size::Xs)
                    .into(),
            ]),

            // shadcn `item-icon.tsx`.
            "icon" => column![
                item::item([
                    item::media(icon::shield_alert()).style(item::Icon).into(),
                    item::content([
                        item::title("Security Alert").into(),
                        item::description("New login detected from unknown device.").into(),
                    ])
                    .into(),
                    item::actions([button("Review")
                        .size(Size::Sm)
                        .style(button::Outline)
                        .into()]),
                ])
                .style(item::Outline)
            ]
            .width(Fill)
            .max_width(512.0)
            .into(),

            _ => unreachable!("unknown item preview id: {id}"),
        }
    }
}
