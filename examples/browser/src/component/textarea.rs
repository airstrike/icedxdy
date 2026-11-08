// Intentional skips: textarea-rtl (RTL). In textarea-field shadcn puts the
// description between the label and the textarea; icedxdy's field always
// puts it below its control. In textarea-disabled shadcn's
// `Field data-disabled` also dims the label; field has no disabled state.

use iced::{Fill, Task};

use icedxdy::widget::{button, column, container, field, textarea};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Textarea",
    description: "Displays a form textarea or a component that looks like a textarea.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/textarea-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "field",
                title: "Field",
                description: Some("Use `field(..)` to add a label and a description."),
                snippet: "examples/browser/snippets/textarea-field.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some("Use `.disabled(true)` to disable the textarea."),
                snippet: "examples/browser/snippets/textarea-disabled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some("Use `.style(textarea::Invalid)` to show an error state."),
                snippet: "examples/browser/snippets/textarea-invalid.md",
            },
            page::Block::Example {
                id: "button",
                title: "Button",
                description: Some("Pair the textarea with a button."),
                snippet: "examples/browser/snippets/textarea-button.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/textarea-api-reference.md"),
];

pub struct TextareaPage {
    state: page::State,
}

impl TextareaPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// The preview's `w-full` column, capped like the docs' preview.
fn full<'a>(content: impl Into<Slot<'a, Message>>) -> iced::Element<'a, Message, Theme> {
    container(content).width(Fill).max_width(384).into()
}

impl page::Page for TextareaPage {
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
            // shadcn `textarea-demo.tsx`.
            "demo" => full(textarea("Type your message here.")),

            // shadcn `textarea-field.tsx`.
            "field" => full(
                field(textarea("Type your message here."))
                    .label("Message")
                    .description("Enter your message below."),
            ),

            // shadcn `textarea-disabled.tsx`.
            "disabled" => {
                full(field(textarea("Type your message here.").disabled(true)).label("Message"))
            }

            // shadcn `textarea-invalid.tsx`.
            "invalid" => full(
                field(textarea("Type your message here.").style(textarea::Invalid))
                    .label("Message")
                    .description("Please enter a valid message."),
            ),

            // shadcn `textarea-button.tsx`.
            "button" => full(
                column![
                    textarea("Type your message here."),
                    button("Send message").width(Fill),
                ]
                .spacing(8)
                .width(Fill),
            ),

            _ => unreachable!("unknown textarea preview id: {id}"),
        }
    }
}
