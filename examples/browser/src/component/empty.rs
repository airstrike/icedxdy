// Intentional skips: empty-outline (a dashed border; iced draws solid
// borders only), empty-background (a `bg-muted/30` class on the root),
// empty-avatar (a 48px avatar, between the avatar sizes), and
// empty-avatar-group and empty-input-group (avatar groups with overlap and
// InputGroup are not ported).

use iced::Task;

use icedxdy::theme::tweak::{Fg, Token};
use icedxdy::widget::{button, column, empty, row};
use icedxdy::{Size, Slot, Theme};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Empty",
    description: "Use the Empty component to display an empty state.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/empty-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[page::Block::Example {
            id: "card",
            title: "Card",
            description: Some("Content with a row of actions and a link below."),
            snippet: "examples/browser/snippets/empty-card.md",
        }],
    },
    page::Block::Md("examples/browser/snippets/empty-api-reference.md"),
];

pub struct EmptyPage {
    state: page::State,
}

impl EmptyPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// shadcn's muted "Learn More ↗" link button.
fn learn_more<'a>(label: &'a str, size: Size) -> Slot<'a, Message> {
    button![Slot::from(label), icon::arrow_up_right()]
        .style(button::Link + Fg(Token::MutedForeground))
        .size(size)
        .into()
}

impl page::Page for EmptyPage {
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
            // shadcn `empty-demo.tsx`.
            "demo" => empty::empty()
                .media(empty::media(icon::folder_code()).style(empty::Icon))
                .title("No Projects Yet")
                .description(
                    "You haven't created any projects yet. Get started by creating your \
                     first project.",
                )
                .content(
                    column![
                        row![
                            button("Create Project"),
                            button("Import Project").style(button::Outline),
                        ]
                        .spacing(8),
                        learn_more("Learn More", Size::Sm),
                    ]
                    .spacing(16)
                    .align_x(iced::Center),
                )
                .into(),

            // shadcn `empty-card.tsx`.
            "card" => empty::empty()
                .media(empty::media(icon::folder()).style(empty::Icon))
                .title("No projects yet")
                .description(
                    "You haven't created any projects yet. Get started by creating your \
                     first project.",
                )
                .content(
                    column![
                        row![
                            button("Create project"),
                            button("Import project").style(button::Outline),
                        ]
                        .spacing(8),
                        learn_more("Learn more", Size::Default),
                    ]
                    .spacing(16)
                    .align_x(iced::Center),
                )
                .into(),

            _ => unreachable!("unknown empty preview id: {id}"),
        }
    }
}
