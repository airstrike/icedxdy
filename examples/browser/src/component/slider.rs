// Intentional skips: slider-controlled, slider-range and slider-multiple
// (more than one thumb; icedxdy's slider has one), slider-vertical (the
// slider is horizontal only) and slider-rtl (RTL).

use iced::{Fill, Task};

use icedxdy::Theme;
use icedxdy::widget::{container, slider};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Slider",
    description: "An input where the user selects a value from within a given range.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/slider-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[page::Block::Example {
            id: "disabled",
            title: "Disabled",
            description: Some("Use `.disabled(true)` to disable the slider."),
            snippet: "examples/browser/snippets/slider-disabled.md",
        }],
    },
    page::Block::Md("examples/browser/snippets/slider-api-reference.md"),
];

pub struct SliderPage {
    state: page::State,
}

impl SliderPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for SliderPage {
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
            // shadcn `slider-demo.tsx`: `mx-auto w-full max-w-xs`.
            "demo" => container(slider(0.0..=100.0).default_value(75.0).step(1.0))
                .width(Fill)
                .max_width(320)
                .into(),

            // shadcn `slider-disabled.tsx`.
            "disabled" => container(
                slider(0.0..=100.0)
                    .default_value(50.0)
                    .step(1.0)
                    .disabled(true),
            )
            .width(Fill)
            .max_width(320)
            .into(),

            _ => unreachable!("unknown slider preview id: {id}"),
        }
    }
}
