use std::collections::BTreeSet;

use iced::widget::Id;
use icedxdy::widget::markdown;
use iced::Task;

use icedxdy::widget::{badge, row, spinner};
use icedxdy::Theme;

use crate::example;
use crate::icon;
use crate::page;
use crate::Message;

pub static META: page::Meta = page::Meta {
    title: "Badge",
    description: "Displays a badge or a component that looks like a badge.",
    examples: &[
        example::Meta { id: "demo", title: "Badge", description: None, snippet: "examples/browser/snippets/badge-demo.md" },
        example::Meta { id: "variants", title: "Variants", description: Some("Use the variant prop to change the variant of the badge."), snippet: "examples/browser/snippets/badge-variants.md" },
        example::Meta { id: "with-icon", title: "With Icon", description: Some("You can render an icon inside the badge."), snippet: "examples/browser/snippets/badge-with-icon.md" },
        example::Meta { id: "with-spinner", title: "With Spinner", description: Some("You can render a spinner inside the badge."), snippet: "examples/browser/snippets/badge-with-spinner.md" },
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
        // Demo: all 5 variants in a row
        0 => {
            iced::widget::row![
                badge("Badge"),
                badge("Secondary").variant(badge::Variant::Secondary),
                badge("Destructive").variant(badge::Variant::Destructive),
                badge("Outline").variant(badge::Variant::Outline),
                badge("Ghost").variant(badge::Variant::Ghost),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into()
        }
        // Variants
        1 => {
            iced::widget::row![
                badge("Default"),
                badge("Secondary").variant(badge::Variant::Secondary),
                badge("Destructive").variant(badge::Variant::Destructive),
                badge("Outline").variant(badge::Variant::Outline),
                badge("Ghost").variant(badge::Variant::Ghost),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into()
        }
        // With Icon
        2 => {
            iced::widget::row![
                badge(row![icon::badge_check().size(14), "Verified"])
                    .variant(badge::Variant::Secondary),
                badge(row!["Bookmark", icon::bookmark().size(14)])
                    .variant(badge::Variant::Outline),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into()
        }
        // With Spinner
        3 => {
            iced::widget::row![
                badge(row![spinner().size(12.0), "Deleting"])
                    .variant(badge::Variant::Destructive),
                badge(row!["Generating", spinner().size(12.0)])
                    .variant(badge::Variant::Secondary),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into()
        }
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

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, index: usize) -> iced::Element<'_, Message, Theme> {
        view(index)
    }
}
