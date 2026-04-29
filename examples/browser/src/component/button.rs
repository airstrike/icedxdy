mod default;
mod destructive;
mod ghost;
mod icon;
mod link;
mod outline;
mod rounded;
mod secondary;
mod size;
mod spinner;
mod with_icon;

use std::collections::BTreeSet;

use iced::widget::Id;
use icedxdy::widget::markdown;
use iced::Task;

use icedxdy::widget::{button, row};
use icedxdy::{Component, Theme};

use crate::example;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Button",
    description: "Displays a button or a component that looks like a button.",
    examples: &[
        example::Meta { id: "size", title: "Size", description: Some("Use the size prop to change the size of the button."), snippet: "examples/browser/snippets/button-size.md" },
        example::Meta { id: "default", title: "Default", description: None, snippet: "examples/browser/snippets/button-default.md" },
        example::Meta { id: "outline", title: "Outline", description: None, snippet: "examples/browser/snippets/button-outline.md" },
        example::Meta { id: "secondary", title: "Secondary", description: None, snippet: "examples/browser/snippets/button-secondary.md" },
        example::Meta { id: "ghost", title: "Ghost", description: None, snippet: "examples/browser/snippets/button-ghost.md" },
        example::Meta { id: "destructive", title: "Destructive", description: None, snippet: "examples/browser/snippets/button-destructive.md" },
        example::Meta { id: "link", title: "Link", description: None, snippet: "examples/browser/snippets/button-link.md" },
        example::Meta { id: "icon", title: "Icon", description: None, snippet: "examples/browser/snippets/button-icon.md" },
        example::Meta { id: "with-icon", title: "With Icon", description: None, snippet: "examples/browser/snippets/button-with-icon.md" },
        example::Meta { id: "rounded", title: "Rounded", description: Some("Use rounded styling to make the button circular."), snippet: "examples/browser/snippets/button-rounded.md" },
        example::Meta { id: "spinner", title: "Spinner", description: Some("Show a loading spinner inside the button."), snippet: "examples/browser/snippets/button-spinner.md" },
    ],
};

#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
}

pub struct ButtonPage {
    state: page::State,
}

impl ButtonPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, &META),
        })
    }
}

fn view(index: usize) -> Component<'static, Message> {
    match index {
        0 => size::view(),
        1 => default::view(),
        2 => outline::view(),
        3 => secondary::view(),
        4 => ghost::view(),
        5 => destructive::view(),
        6 => link::view(),
        7 => icon::view(),
        8 => with_icon::view(),
        9 => rounded::view(),
        10 => spinner::view(),
        _ => unreachable!(),
    }
}

impl page::Page for ButtonPage {
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
        // Verbatim from .nova/apps/v4/examples/radix/button-demo.tsx:
        //   <Button variant="outline">Button</Button>
        //   <Button variant="outline" size="icon"><ArrowUpIcon /></Button>
        // page.rs handles the card chrome + centering.
        let content: Component<'_, crate::Message> = row![
            button::outline("Button"),
            button::icon(crate::icon::arrow_up()),
        ]
        .spacing(8)
        .into();

        Some(page::Hero::Card(content.into()))
    }
}
