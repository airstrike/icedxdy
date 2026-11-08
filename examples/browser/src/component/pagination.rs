// Intentional skips: pagination-icons-only (its rows-per-page picker is a
// Select, which is not ported) and pagination-rtl (RTL).

use iced::Task;

use icedxdy::Theme;
use icedxdy::widget::pagination;

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Pagination",
    description: "Pagination with page navigation, next and previous links.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/pagination-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[page::Block::Example {
            id: "simple",
            title: "Simple",
            description: Some("A simple pagination with only page numbers."),
            snippet: "examples/browser/snippets/pagination-simple.md",
        }],
    },
    page::Block::Md("examples/browser/snippets/pagination-api-reference.md"),
];

pub struct PaginationPage {
    state: page::State,
}

impl PaginationPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for PaginationPage {
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
            // shadcn `pagination-demo.tsx`.
            "demo" => pagination([
                pagination::previous().icon(icon::chevron_left()).into(),
                pagination::link("1").into(),
                pagination::link("2").active(true).into(),
                pagination::link("3").into(),
                pagination::ellipsis().icon(icon::ellipsis()).into(),
                pagination::next().icon(icon::chevron_right()).into(),
            ])
            .into(),

            // shadcn `pagination-simple.tsx`.
            "simple" => pagination([
                pagination::link("1").into(),
                pagination::link("2").active(true).into(),
                pagination::link("3").into(),
                pagination::link("4").into(),
                pagination::link("5").into(),
            ])
            .into(),

            _ => unreachable!("unknown pagination preview id: {id}"),
        }
    }
}
