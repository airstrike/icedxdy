use std::collections::BTreeSet;

use iced::widget::Id;
use icedxdy::widget::markdown;
use iced::Task;

use icedxdy::widget::{badge, row, spinner};
use icedxdy::{Component, Theme};

use crate::example;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Badge",
    description: "Displays a badge or a component that looks like a badge.",
    examples: &[
        example::Meta { id: "demo", title: "Badge", description: None, snippet: "examples/browser/snippets/badge-demo.md" },
        example::Meta { id: "variants", title: "Variants", description: Some("Use the variant prop to change the variant of the badge."), snippet: "examples/browser/snippets/badge-variants.md" },
        example::Meta { id: "icon", title: "With Icon", description: Some("You can render an icon inside the badge."), snippet: "examples/browser/snippets/badge-icon.md" },
        example::Meta { id: "spinner", title: "With Spinner", description: Some("You can render a spinner inside the badge."), snippet: "examples/browser/snippets/badge-spinner.md" },
        example::Meta { id: "link", title: "As Link", description: Some("Use the on_press builder to render a clickable badge."), snippet: "examples/browser/snippets/badge-link.md" },
    ],
};

#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
}

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

fn view(index: usize) -> Component<'static, Message> {
    match index {
        // Demo: all 5 variants in a row
        0 => row![
            badge("Badge"),
            badge::secondary("Secondary"),
            badge::destructive("Destructive"),
            badge::outline("Outline"),
            badge::ghost("Ghost"),
        ]
        .spacing(8)
        .into(),
        // Variants
        1 => row![
            badge("Default"),
            badge::secondary("Secondary"),
            badge::destructive("Destructive"),
            badge::outline("Outline"),
            badge::ghost("Ghost"),
        ]
        .spacing(8)
        .into(),
        // With Icon
        2 => row![
            badge::secondary(row![icon::badge_check().size(14), "Verified"]),
            badge::outline(row!["Bookmark", icon::bookmark().size(14)]),
        ]
        .spacing(8)
        .into(),
        // With Spinner
        3 => row![
            badge::destructive(row![spinner().size(12.0), "Deleting"]),
            badge::secondary(row!["Generating", spinner().size(12.0)]),
        ]
        .spacing(8)
        .into(),
        // Link (clickable badge)
        4 => row![
            badge(row!["Open Link", icon::arrow_up_right().size(14)])
                .on_press(Message::Pressed),
        ]
        .spacing(8)
        .into(),
        _ => unreachable!(),
    }
}

impl page::Page for BadgePage {
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

    fn code_expanded(&self) -> &BTreeSet<usize> {
        &self.state.code_expanded
    }

    fn init(&self) -> Task<crate::Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<crate::Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, index: usize) -> iced::Element<'_, crate::Message, Theme> {
        let pi = self.state.page_index;
        view(index)
            .map(move |_| crate::Message::Page(pi, page::Message::Noop))
            .into()
    }
}
