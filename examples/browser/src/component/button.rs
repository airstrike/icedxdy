// TODO: API Reference section once a table widget lands.
//
// Live docs API Reference covers `Button`: `variant`
// (`"default" | "outline" | "ghost" | "destructive" | "secondary" | "link"`,
// default `"default"`), `size`
// (`"default" | "xs" | "sm" | "lg" | "icon" | "icon-xs" | "icon-sm" | "icon-lg"`,
// default `"default"`), and `asChild` (`boolean`, default `false`). Captured
// here so the gap is auditable when the table widget lands.
//
// Intentional skips per CLAUDE.md: button-rtl (RTL). Button Group and
// As Child examples are also skipped — Button Group is a separate
// widget we have not yet ported, and As Child relies on Tailwind's
// `data-slot` cascade that we don't reproduce.

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

use iced::Task;

use icedxdy::widget::{button, row};
use icedxdy::{Component, Theme};

use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Button",
    description: "Displays a button or a component that looks like a button.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "",
    },
    page::Block::Md("examples/browser/snippets/button-usage.md"),
    page::Block::Md("examples/browser/snippets/button-cursor.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "size",
                title: "Size",
                description: Some("Use the size prop to change the size of the button."),
                snippet: "examples/browser/snippets/button-size.md",
            },
            page::Block::Example {
                id: "default",
                title: "Default",
                description: None,
                snippet: "examples/browser/snippets/button-default.md",
            },
            page::Block::Example {
                id: "outline",
                title: "Outline",
                description: None,
                snippet: "examples/browser/snippets/button-outline.md",
            },
            page::Block::Example {
                id: "secondary",
                title: "Secondary",
                description: None,
                snippet: "examples/browser/snippets/button-secondary.md",
            },
            page::Block::Example {
                id: "ghost",
                title: "Ghost",
                description: None,
                snippet: "examples/browser/snippets/button-ghost.md",
            },
            page::Block::Example {
                id: "destructive",
                title: "Destructive",
                description: None,
                snippet: "examples/browser/snippets/button-destructive.md",
            },
            page::Block::Example {
                id: "link",
                title: "Link",
                description: None,
                snippet: "examples/browser/snippets/button-link.md",
            },
            page::Block::Example {
                id: "icon",
                title: "Icon",
                description: None,
                snippet: "examples/browser/snippets/button-icon.md",
            },
            page::Block::Example {
                id: "with-icon",
                title: "With Icon",
                description: None,
                snippet: "examples/browser/snippets/button-with-icon.md",
            },
            page::Block::Example {
                id: "rounded",
                title: "Rounded",
                description: Some("Use rounded styling to make the button circular."),
                snippet: "examples/browser/snippets/button-rounded.md",
            },
            page::Block::Example {
                id: "spinner",
                title: "Spinner",
                description: Some("Show a loading spinner inside the button."),
                snippet: "examples/browser/snippets/button-spinner.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/button-api-reference.md"),
];

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
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn demo(pi: usize) -> Component<'static, crate::Message> {
    // Verbatim shadcn `button-demo.tsx`:
    //   <Button variant="outline">Button</Button>
    //   <Button variant="outline" size="icon"><ArrowUpIcon /></Button>
    row![
        button::outline("Button").on_press(crate::Message::Page(pi, page::Message::Noop)),
        button::icon(crate::icon::arrow_up())
            .on_press(crate::Message::Page(pi, page::Message::Noop)),
    ]
    .spacing(8)
    .into()
}

fn view(id: &str) -> Component<'static, Message> {
    match id {
        "size" => size::view(),
        "default" => default::view(),
        "outline" => outline::view(),
        "secondary" => secondary::view(),
        "ghost" => ghost::view(),
        "destructive" => destructive::view(),
        "link" => link::view(),
        "icon" => icon::view(),
        "with-icon" => with_icon::view(),
        "rounded" => rounded::view(),
        "spinner" => spinner::view(),
        _ => unreachable!("unknown button preview id: {id}"),
    }
}

impl page::Page for ButtonPage {
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
        if id == "demo" {
            return demo(pi).into();
        }
        view(id)
            .map(move |_| crate::Message::Page(pi, page::Message::Noop))
            .into()
    }
}
