// API Reference is rendered via the `dialog-api-reference.md` snippet at the
// bottom of BLOCKS.
//
// Intentional skips:
// - dialog-rtl — RTL is not in scope (per CLAUDE.md).
// - dialog-scrollable-content, dialog-sticky-footer — need a scroll area
//   sized to the viewport inside the overlay.

use iced::Task;

use icedxdy::widget::{button, column, dialog, field, input};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Dialog",
    description: "A window overlaid on either the primary window or another dialog window, \
                  rendering the content underneath inert.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/dialog-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "close-button",
                title: "Custom Close Button",
                description: Some("Replace the default close control with your own button."),
                snippet: "examples/browser/snippets/dialog-close-button.md",
            },
            page::Block::Example {
                id: "no-close-button",
                title: "No Close Button",
                description: Some("Use `.show_close_button(false)` to hide the close button."),
                snippet: "examples/browser/snippets/dialog-no-close-button.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/dialog-api-reference.md"),
];

/// The dialog page. Every dialog is uncontrolled; its inputs keep what is
/// typed while it stays open, and mount fresh each time it opens.
pub struct DialogPage {
    state: page::State,
}

impl DialogPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn labelled<'a>(label: &'a str, value: &'a str) -> Slot<'a, Message> {
    field(input("").default_value(value).width(iced::Fill))
        .label(label)
        .into()
}

impl page::Page for DialogPage {
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
            // shadcn `dialog-demo.tsx`.
            "demo" => dialog(
                |d| d.trigger(button("Open Dialog").style(button::Outline)),
                |d| {
                    column![
                        dialog::header(
                            "Edit profile",
                            "Make changes to your profile here. Click save when you're done.",
                        ),
                        column![
                            labelled("Name", "Pedro Duarte"),
                            labelled("Username", "@peduarte"),
                        ]
                        .spacing(16),
                        dialog::footer([
                            d.close(button("Cancel").style(button::Outline)),
                            d.close(button("Save changes")),
                        ]),
                    ]
                    .spacing(16)
                    .into()
                },
            )
            .into(),

            // shadcn `dialog-close-button.tsx`.
            "close-button" => dialog(
                |d| d.trigger(button("Share").style(button::Outline)),
                |d| {
                    column![
                        dialog::header(
                            "Share link",
                            "Anyone who has this link will be able to view this.",
                        ),
                        input("")
                            .default_value("https://ui.shadcn.com/docs/installation")
                            .width(iced::Fill),
                        dialog::footer([d.close(button("Close"))]),
                    ]
                    .spacing(16)
                    .into()
                },
            )
            .into(),

            // shadcn `dialog-no-close-button.tsx`.
            "no-close-button" => dialog(
                |d| d.trigger(button("No Close Button").style(button::Outline)),
                |_| {
                    dialog::header(
                        "No Close Button",
                        "This dialog doesn't have a close button in the top-right corner.",
                    )
                },
            )
            .show_close_button(false)
            .into(),

            _ => unreachable!("unknown dialog preview id: {id}"),
        }
    }
}
