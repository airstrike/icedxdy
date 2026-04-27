mod action;
mod default;
mod demo;
mod destructive;

use std::collections::BTreeSet;

use iced::widget::Id;
use icedxdy::widget::markdown;
use iced::Task;

use icedxdy::{Component, Theme};

use crate::example;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Alert",
    description: "Displays a callout for user attention.",
    examples: &[
        example::Meta { id: "demo", title: "Alert", description: None, snippet: "examples/browser/snippets/alert-demo.md" },
        example::Meta { id: "basic", title: "Basic", description: Some("A basic alert with an icon, title and description."), snippet: "examples/browser/snippets/alert-basic.md" },
        example::Meta { id: "destructive", title: "Destructive", description: Some("Use variant destructive to create a destructive alert."), snippet: "examples/browser/snippets/alert-destructive.md" },
        example::Meta { id: "action", title: "Action", description: Some("Use AlertAction to add a button or other action element to the alert."), snippet: "examples/browser/snippets/alert-action.md" },
    ],
};

#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
}

pub struct AlertPage {
    state: page::State,
}

impl AlertPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, &META),
        })
    }
}

fn view(index: usize) -> Component<'static, Message> {
    match index {
        0 => demo::view(),
        1 => default::view(),
        2 => destructive::view(),
        3 => action::view(),
        _ => unreachable!(),
    }
}

impl page::Page for AlertPage {
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
        view(index).map(move |_| crate::Message::Page(pi, page::Message::Noop)).into()
    }
}
