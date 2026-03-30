use std::collections::BTreeSet;

use iced::widget::{Id, column, markdown};
use iced::Task;

use icedxdy::widget::text;
use icedxdy::Theme;

use crate::example;
use crate::page;
use crate::Message;

pub static META: page::Meta = page::Meta {
    title: "Text",
    description: "Typography components for headings, paragraphs, and more.",
    examples: &[
        example::Meta { id: "demo", title: "Typography", description: None, snippet: "" },
    ],
};

pub struct TextPage {
    state: page::State,
}

impl TextPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, &META),
        })
    }
}

impl page::Page for TextPage {
    fn page_index(&self) -> usize { self.state.page_index }
    fn meta(&self) -> &'static page::Meta {
        &META
    }

    fn snippets(&self) -> &[Option<Vec<markdown::Item>>] {
        &self.state.snippets
    }

    fn visible(&self) -> &BTreeSet<usize> {
        &self.state.visible
    }

    fn example_ids(&self) -> &[Id] {
        &self.state.example_ids
    }

    fn scrollable_id(&self) -> Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        Task::none()
    }

    fn preview(&self, index: usize) -> iced::Element<'_, Message, Theme> {
        match index {
            0 => {
                column![
                    text::h1("Heading 1"),
                    text::h2("Heading 2"),
                    text::h3("Heading 3"),
                    text::h4("Heading 4"),
                    text::p("Paragraph text - the quick brown fox jumps over the lazy dog."),
                    text::lead("Lead text - introductory paragraph style."),
                    text::large("Large text"),
                    text::small("Small text"),
                    text::muted("Muted text - secondary information."),
                    text::xs("Extra small text - fine print and labels."),
                ]
                .spacing(12)
                .into()
            }
            _ => unreachable!(),
        }
    }
}
