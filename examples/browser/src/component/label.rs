// A `<Label htmlFor>` links to its control by id; in icedxdy the link is a
// `field(..)`, whose label presses the control it wraps. `text::label`
// alone is the typography without the link.

use iced::Task;

use icedxdy::widget::{checkbox, field};
use icedxdy::{Horizontal, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Label",
    description: "Renders an accessible label associated with controls.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/label-demo.md",
    },
    page::Block::Md("examples/browser/snippets/label-api-reference.md"),
];

pub struct LabelPage {
    state: page::State,
}

impl LabelPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for LabelPage {
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
            // shadcn `label-demo.tsx`: pressing the label toggles the box.
            "demo" => field(checkbox())
                .orientation(Horizontal)
                .label("Accept terms and conditions")
                .into(),

            _ => unreachable!("unknown label preview id: {id}"),
        }
    }
}
