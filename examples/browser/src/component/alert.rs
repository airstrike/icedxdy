mod action;
mod default;
mod destructive;

use iced::Task;

use icedxdy::widget::{alert, column, container};
use icedxdy::{Component, Theme};

use crate::icon;
use crate::page;

// Intentional skips per CLAUDE.md: alert-colors (Custom Colors)
// and alert-rtl (RTL).
pub static META: page::Meta = page::Meta {
    title: "Alert",
    description: "Displays a callout for user attention.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "",
    },
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some("A basic alert with an icon, title and description."),
        snippet: "examples/browser/snippets/alert-basic.md",
    },
    page::Block::Example {
        id: "destructive",
        title: "Destructive",
        description: Some("Use variant destructive to create a destructive alert."),
        snippet: "examples/browser/snippets/alert-destructive.md",
    },
    page::Block::Example {
        id: "action",
        title: "Action",
        description: Some("Use AlertAction to add a button or other action element to the alert."),
        snippet: "examples/browser/snippets/alert-action.md",
    },
];

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
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn demo<'a>() -> Component<'a, Message> {
    // Verbatim shadcn `alert-demo.tsx`: two stacked alerts capped at
    // max-w-md (448px).
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

    container(stack)
        .width(iced::Length::Fill)
        .max_width(448)
        .into()
}

impl page::Page for AlertPage {
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

    fn preview(&self, id: &str) -> iced::Element<'_, crate::Message, Theme> {
        let pi = self.state.page_index;
        let view: Component<'_, Message> = match id {
            "demo" => demo(),
            "basic" => default::view(),
            "destructive" => destructive::view(),
            "action" => action::view(),
            _ => unreachable!("unknown alert preview id: {id}"),
        };
        view.map(move |_| crate::Message::Page(pi, page::Message::Noop))
            .into()
    }
}
