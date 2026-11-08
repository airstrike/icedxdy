// Intentional skips: alert-dialog-rtl (RTL). alert-dialog-basic is the
// demo with a longer description, so the page shows the demo only. The
// destructive example's description links "Settings"; here it is plain
// text.

use iced::{Fill, Task};

use icedxdy::theme::tweak::{Alpha, Bg, Fg, Token};
use icedxdy::widget::{alert_dialog, button, column};
use icedxdy::{Size, Slot, Theme};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Alert Dialog",
    description: "A modal dialog that interrupts the user with important content and expects a response.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/alert-dialog-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "small",
                title: "Small",
                description: Some("Use `.size(Size::Sm)` for a smaller alert dialog."),
                snippet: "examples/browser/snippets/alert-dialog-small.md",
            },
            page::Block::Example {
                id: "media",
                title: "Media",
                description: Some("Use `.media(..)` on the header to add an icon."),
                snippet: "examples/browser/snippets/alert-dialog-media.md",
            },
            page::Block::Example {
                id: "small-media",
                title: "Small with Media",
                description: None,
                snippet: "examples/browser/snippets/alert-dialog-small-media.md",
            },
            page::Block::Example {
                id: "destructive",
                title: "Destructive",
                description: Some("Use a destructive action button for irreversible actions."),
                snippet: "examples/browser/snippets/alert-dialog-destructive.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/alert-dialog-api-reference.md"),
];

pub struct AlertDialogPage {
    state: page::State,
}

impl AlertDialogPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// shadcn's "Show Dialog" outline trigger.
fn show(a: &alert_dialog::Handle) -> Slot<'static, Message> {
    a.trigger(button("Show Dialog").style(button::Outline))
}

impl page::Page for AlertDialogPage {
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
            // shadcn `alert-dialog-demo.tsx`.
            "demo" => alert_dialog(show, |a| {
                column![
                    a.header(
                        "Are you absolutely sure?",
                        "This action cannot be undone. This will permanently delete your \
                         account from our servers.",
                    ),
                    a.footer([
                        a.cancel(button("Cancel").style(button::Outline)),
                        a.action(button("Continue")),
                    ]),
                ]
                .width(Fill)
                .into()
            })
            .into(),

            // shadcn `alert-dialog-small.tsx`.
            "small" => alert_dialog(show, |a| {
                column![
                    a.header(
                        "Allow accessory to connect?",
                        "Do you want to allow the USB accessory to connect to this device?",
                    ),
                    a.footer([
                        a.cancel(button("Don't allow").style(button::Outline)),
                        a.action(button("Allow")),
                    ]),
                ]
                .width(Fill)
                .into()
            })
            .size(Size::Sm)
            .into(),

            // shadcn `alert-dialog-media.tsx`.
            "media" => alert_dialog(
                |a| a.trigger(button("Share Project").style(button::Outline)),
                |a| {
                    column![
                        a.header(
                            "Share this project?",
                            "Anyone with the link will be able to view and edit this project.",
                        )
                        .media(icon::circle_fading_plus()),
                        a.footer([
                            a.cancel(button("Cancel").style(button::Outline)),
                            a.action(button("Share")),
                        ]),
                    ]
                    .width(Fill)
                    .into()
                },
            )
            .into(),

            // shadcn `alert-dialog-small-media.tsx`.
            "small-media" => alert_dialog(show, |a| {
                column![
                    a.header(
                        "Allow accessory to connect?",
                        "Do you want to allow the USB accessory to connect to this device?",
                    )
                    .media(icon::bluetooth()),
                    a.footer([
                        a.cancel(button("Don't allow").style(button::Outline)),
                        a.action(button("Allow")),
                    ]),
                ]
                .width(Fill)
                .into()
            })
            .size(Size::Sm)
            .into(),

            // shadcn `alert-dialog-destructive.tsx`.
            "destructive" => alert_dialog(
                |a| a.trigger(button("Delete Chat").style(button::Destructive)),
                |a| {
                    column![
                        a.header(
                            "Delete chat?",
                            "This will permanently delete this chat conversation. View \
                             Settings delete any memories saved during this chat.",
                        )
                        .media(icon::trash_2())
                        .media_style(
                            alert_dialog::Media
                                + Bg(Token::Destructive)
                                + Alpha(0.1)
                                + Fg(Token::Destructive)
                        ),
                        a.footer([
                            a.cancel(button("Cancel").style(button::Outline)),
                            a.action(button("Delete").style(button::Destructive)),
                        ]),
                    ]
                    .width(Fill)
                    .into()
                },
            )
            .size(Size::Sm)
            .into(),

            _ => unreachable!("unknown alert dialog preview id: {id}"),
        }
    }
}
