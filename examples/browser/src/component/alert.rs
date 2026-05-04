mod action;
mod default;
mod destructive;

use std::collections::BTreeSet;

use iced::widget::Id;
use icedxdy::widget::markdown;
use iced::Task;

use icedxdy::widget::{alert, column, container};
use icedxdy::{Component, Theme};

use crate::example;
use crate::icon;
use crate::page;

// Intentional skips per CLAUDE.md: alert-colors (Custom Colors)
// and alert-rtl (RTL).
pub static META: page::Meta = page::Meta {
    title: "Alert",
    description: "Displays a callout for user attention.",
    examples: &[
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
        0 => default::view(),
        1 => destructive::view(),
        2 => action::view(),
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

    fn hero(&self) -> Option<page::Hero<'_, crate::Message, Theme>> {
        // Verbatim shadcn `alert-demo.tsx`: two stacked alerts capped at
        // max-w-md (448px). page.rs handles the card chrome + centering.
        let stack = column![
            alert()
                .icon(icon::circle_check_big())
                .title("Payment successful")
                .description(
                    "Your payment of $29.99 has been processed. A receipt has been sent to your email address.",
                ),
            alert()
                .icon(icon::info())
                .title("New feature available")
                .description(
                    "We've added dark mode support. You can enable it in your account settings.",
                ),
        ]
        .spacing(16)
        .width(iced::Length::Fill);

        let capped = container(stack).width(iced::Length::Fill).max_width(448);

        Some(page::Hero::Card(capped.into()))
    }
}
