// Intentional skips per CLAUDE.md: separator-rtl (RTL).

use iced::Task;

use icedxdy::widget::{column, row, separator, space, text};
use icedxdy::{Slot, Theme, Vertical, theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Separator",
    description: "Visually or semantically separates content.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/separator-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "vertical",
                title: "Vertical",
                description: Some("Use `.orientation(Vertical)` for a vertical separator."),
                snippet: "examples/browser/snippets/separator-vertical.md",
            },
            page::Block::Example {
                id: "menu",
                title: "Menu",
                description: Some("Vertical separators between menu items with descriptions."),
                snippet: "examples/browser/snippets/separator-menu.md",
            },
            page::Block::Example {
                id: "list",
                title: "List",
                description: Some("Horizontal separators between list items."),
                snippet: "examples/browser/snippets/separator-list.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/separator-api-reference.md"),
];

pub struct SeparatorPage {
    state: page::State,
}

impl SeparatorPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// A menu entry: a medium title over an `text-xs` muted description.
fn menu_item<'a>(title: &'a str, description: &'a str) -> Slot<'a, Message> {
    column![
        text::small(title).medium(),
        text::xs(description).style(theme::text::Muted),
    ]
    .spacing(4)
    .into()
}

/// A `<dl>` row: the term on the left, its muted value on the right.
fn list_item<'a>(term: &'a str, value: &'a str) -> Slot<'a, Message> {
    row![text::small(term), space::horizontal(), text::muted(value)]
        .width(iced::Fill)
        .into()
}

impl page::Page for SeparatorPage {
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
            // shadcn `separator-demo.tsx`.
            "demo" => column![
                column![
                    text::label("shadcn/ui"),
                    text::muted("The Foundation for your Design System"),
                ]
                .spacing(6),
                separator(),
                text::small(
                    "A set of beautifully designed components that you can customize, \
                     extend, and build on.",
                ),
            ]
            .spacing(16)
            .max_width(384.0)
            .into(),

            // shadcn `separator-vertical.tsx`: `flex h-5 items-center gap-4`.
            "vertical" => row![
                text::small("Blog"),
                separator().orientation(Vertical),
                text::small("Docs"),
                separator().orientation(Vertical),
                text::small("Source"),
            ]
            .spacing(16)
            .height(20)
            .align_y(iced::Center)
            .into(),

            // shadcn `separator-menu.tsx` at its `md:` breakpoint.
            "menu" => row![
                menu_item("Settings", "Manage preferences"),
                separator().orientation(Vertical),
                menu_item("Account", "Profile & security"),
                separator().orientation(Vertical),
                menu_item("Help", "Support & docs"),
            ]
            .spacing(16)
            .align_y(iced::Center)
            .into(),

            // shadcn `separator-list.tsx`.
            "list" => column![
                list_item("Item 1", "Value 1"),
                separator(),
                list_item("Item 2", "Value 2"),
                separator(),
                list_item("Item 3", "Value 3"),
            ]
            .spacing(8)
            .width(iced::Fill)
            .max_width(384.0)
            .into(),

            _ => unreachable!("unknown separator preview id: {id}"),
        }
    }
}
