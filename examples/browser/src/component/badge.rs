use std::collections::BTreeSet;

use iced::widget::{Id, markdown, row};
use iced::Task;

use icedxdy::widget::badge;
use icedxdy::Theme;

use crate::example;
use crate::page;
use crate::Message;

pub static META: page::Meta = page::Meta {
    title: "Badge",
    description: "Displays a badge or a component that looks like a badge.",
    examples: &[
        example::Meta { id: "demo", title: "Badge", description: None, snippet: "examples/browser/snippets/badge-default.md" },
        example::Meta { id: "default", title: "Default", description: None, snippet: "examples/browser/snippets/badge-default.md" },
        example::Meta { id: "secondary", title: "Secondary", description: None, snippet: "examples/browser/snippets/badge-secondary.md" },
        example::Meta { id: "destructive", title: "Destructive", description: None, snippet: "examples/browser/snippets/badge-destructive.md" },
        example::Meta { id: "outline", title: "Outline", description: None, snippet: "examples/browser/snippets/badge-outline.md" },
    ],
};

pub struct BadgePage {
    state: page::State,
}

impl BadgePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, &META),
        })
    }
}

fn view(index: usize) -> iced::Element<'static, Message, Theme> {
    match index {
        0 => {
            row![
                badge("Badge"),
                badge("Secondary").variant(badge::Variant::Secondary),
                badge("Destructive").variant(badge::Variant::Destructive),
                badge("Outline").variant(badge::Variant::Outline),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into()
        }
        1 => badge("Badge").into(),
        2 => badge("Secondary").variant(badge::Variant::Secondary).into(),
        3 => badge("Destructive").variant(badge::Variant::Destructive).into(),
        4 => badge("Outline").variant(badge::Variant::Outline).into(),
        _ => unreachable!(),
    }
}

impl page::Page for BadgePage {
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
        view(index)
    }
}
