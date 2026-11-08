// API Reference is rendered via the `hover-card-api-reference.md` snippet
// at the bottom of BLOCKS.
//
// Intentional skips:
// - hover-card-rtl: RTL is not in scope (per CLAUDE.md).

use std::time::Duration;

use iced::Task;

use icedxdy::widget::{button, column, container, hover_card, row, text};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Hover Card",
    description: "For sighted users to preview content available behind a link.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/hover-card-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[page::Block::Example {
            id: "sides",
            title: "Sides",
            description: Some(
                "Use `.side(..)` to change the side of the trigger the card opens on.",
            ),
            snippet: "examples/browser/snippets/hover-card-sides.md",
        }],
    },
    page::Block::Md("examples/browser/snippets/hover-card-api-reference.md"),
];

pub struct HoverCardPage {
    state: page::State,
}

impl HoverCardPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// An outline trigger whose card opens on `side`.
fn side<'a>(label: &'a str, side: hover_card::Side) -> Slot<'a, Message> {
    hover_card(
        button(label).style(button::Outline),
        column![
            text("Hover Card").medium(),
            text(format!(
                "This hover card appears on the {} side of the trigger.",
                label.to_lowercase()
            )),
        ]
        .spacing(4),
    )
    .side(side)
    .open_delay(Duration::from_millis(100))
    .close_delay(Duration::from_millis(100))
    .into()
}

impl page::Page for HoverCardPage {
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
            // shadcn `hover-card-demo.tsx`.
            "demo" => hover_card(
                button("Hover Here").style(button::Link),
                column![
                    text("@nextjs").semibold(),
                    text("The React Framework – created and maintained by @vercel."),
                    container(text::muted("Joined December 2021").size(text::size::XS))
                        .padding([4, 0, 0, 0]),
                ]
                .spacing(2),
            )
            .open_delay(Duration::from_millis(10))
            .close_delay(Duration::from_millis(100))
            .into(),

            // shadcn `hover-card-sides.tsx`.
            "sides" => row![
                side("Left", hover_card::Side::Left),
                side("Top", hover_card::Side::Top),
                side("Bottom", hover_card::Side::Bottom),
                side("Right", hover_card::Side::Right),
            ]
            .spacing(8)
            .into(),

            _ => unreachable!("unknown hover card preview id: {id}"),
        }
    }
}
