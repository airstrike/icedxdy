// Intentional skips: input-group examples built on DropdownMenu, Popover,
// Tooltip triggers inside addons, ButtonGroup and Spinner layouts beyond
// the ones below, and input-group-rtl (RTL). Pressing an addon does not
// focus the input as shadcn's `onClick` does.

use iced::{Fill, Task};

use icedxdy::widget::{Column, input, input_group, kbd, row, textarea};
use icedxdy::{Slot, Theme};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Input Group",
    description: "Display additional information or actions to an input or textarea.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/input-group-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "icon",
                title: "Icon",
                description: None,
                snippet: "examples/browser/snippets/input-group-icon.md",
            },
            page::Block::Example {
                id: "text",
                title: "Text",
                description: Some("Display additional text information alongside inputs."),
                snippet: "examples/browser/snippets/input-group-text.md",
            },
            page::Block::Example {
                id: "button",
                title: "Button",
                description: Some("Add buttons to perform actions within the input group."),
                snippet: "examples/browser/snippets/input-group-button.md",
            },
            page::Block::Example {
                id: "kbd",
                title: "Kbd",
                description: None,
                snippet: "examples/browser/snippets/input-group-kbd.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/input-group-api-reference.md"),
];

pub struct InputGroupPage {
    state: page::State,
}

impl InputGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// The examples' `grid w-full max-w-sm gap-6` column.
fn stack<'a>(
    groups: impl IntoIterator<Item = Slot<'a, Message>>,
) -> iced::Element<'a, Message, Theme> {
    Column::with_children(groups)
        .spacing(24)
        .width(Fill)
        .max_width(384.0)
        .into()
}

/// An addon after the control.
fn end<'a>(content: impl Into<Slot<'a, Message>>) -> input_group::Addon<'a, Message> {
    input_group::addon(content).align(input_group::Align::InlineEnd)
}

impl page::Page for InputGroupPage {
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
            // shadcn `input-group-demo.tsx`.
            "demo" => stack([input_group(input("Search..."))
                .addon(input_group::addon(icon::search()))
                .addon(end("12 results"))
                .into()]),

            // shadcn `input-group-icon.tsx`.
            "icon" => stack([
                input_group(input("Search..."))
                    .addon(input_group::addon(icon::search()))
                    .into(),
                input_group(input("Enter your email"))
                    .addon(input_group::addon(icon::mail()))
                    .into(),
                input_group(input("Card number"))
                    .addon(input_group::addon(icon::credit_card()))
                    .addon(end(icon::check()))
                    .into(),
                input_group(input("Card number"))
                    .addon(end(row![icon::star(), icon::info()].spacing(8)))
                    .into(),
            ]),

            // shadcn `input-group-text.tsx`.
            "text" => stack([
                input_group(input("0.00"))
                    .addon(input_group::addon(input_group::text("$")))
                    .addon(end(input_group::text("USD")))
                    .into(),
                input_group(input("example.com"))
                    .addon(input_group::addon(input_group::text("https://")))
                    .addon(end(input_group::text(".com")))
                    .into(),
                input_group(input("Enter your username"))
                    .addon(end(input_group::text("@company.com")))
                    .into(),
                input_group(textarea("Enter your message"))
                    .addon(
                        input_group::addon(input_group::text("120 characters left").size(12))
                            .align(input_group::Align::BlockEnd),
                    )
                    .into(),
            ]),

            // shadcn `input-group-button.tsx`, its first group.
            "button" => stack([input_group(input("https://x.com/shadcn"))
                .addon(end(input_group::button(icon::copy()).square()))
                .into()]),

            // shadcn `input-group-kbd.tsx`.
            "kbd" => stack([input_group(input("Search..."))
                .addon(input_group::addon(icon::search()))
                .addon(end(kbd("⌘K")))
                .into()]),

            _ => unreachable!("unknown input group preview id: {id}"),
        }
    }
}
