use iced::Task;

use icedxdy::Theme;
use icedxdy::widget::{column, field, input};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Label",
    description: "A field-style label typography helper (text-sm font-medium) for form inputs.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/label-demo.md",
    },
];

const F_DEMO: usize = 0;

pub struct LabelPage {
    state: page::State,
    demo_value: String,
}

impl LabelPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            demo_value: String::new(),
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
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        if let page::Message::InputChanged(F_DEMO, v) = event {
            self.demo_value = v;
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match id {
            // Demo: label helper above an input — demonstrates `text::label`
            // typography (text-sm font-medium). Verbatim shape of shadcn's
            // `label-demo.tsx`: a Label + Input pair at fixed width.
            "demo" => column![
                field(
                    input("m@example.com", &self.demo_value)
                        .on_input(move |v| {
                            Message::Page(pi, page::Message::InputChanged(F_DEMO, v))
                        })
                        .width(iced::Fill),
                )
                .label("Your email"),
            ]
            .width(320)
            .into(),

            _ => unreachable!("unknown label preview id: {id}"),
        }
    }
}
