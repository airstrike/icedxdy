// Intentional skips: the "Promise" button of sonner-types (toast.promise
// follows a future; an app raises its own toasts with `toast::show` when
// its task completes). The toaster wraps the whole browser, as shadcn's
// `<Toaster />` sits once in the root layout. Sonner collapses its stack
// and expands it on hover; icedxdy shows the newest three expanded.

use iced::Task;

use icedxdy::widget::{button, column, row, toast};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Sonner",
    description: "An opinionated toast component.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/sonner-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "types",
                title: "Types",
                description: Some("Use `.kind(..)` to pick the toast's icon."),
                snippet: "examples/browser/snippets/sonner-types.md",
            },
            page::Block::Example {
                id: "description",
                title: "Description",
                description: Some("Use `.description(..)` to add a description to the toast."),
                snippet: "examples/browser/snippets/sonner-description.md",
            },
            page::Block::Example {
                id: "position",
                title: "Position",
                description: Some("Use `.position(..)` to change where the toast appears."),
                snippet: "examples/browser/snippets/sonner-position.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/sonner-api-reference.md"),
];

pub struct SonnerPage {
    state: page::State,
}

impl SonnerPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// An outline button raising `toast`.
fn raise<'a>(label: &'a str, toast: toast::Toast<Message>) -> Slot<'a, Message> {
    toast::trigger(button(label).style(button::Outline), toast)
}

/// shadcn's "Event has been created" toast.
fn created() -> toast::Toast<Message> {
    toast::toast("Event has been created")
}

impl page::Page for SonnerPage {
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
            // shadcn `sonner-demo.tsx`.
            "demo" => raise(
                "Show Toast",
                created()
                    .description("Sunday, December 03, 2023 at 9:00 AM")
                    .action(
                        "Undo",
                        Message::Page(self.state.page_index, page::Message::Noop),
                    ),
            )
            .into(),

            // shadcn `sonner-types.tsx`.
            "types" => row![
                raise("Default", created()),
                raise("Success", created().kind(toast::Kind::Success)),
                raise(
                    "Info",
                    toast::toast("Be at the area 10 minutes before the event time")
                        .kind(toast::Kind::Info),
                ),
                raise(
                    "Warning",
                    toast::toast("Event start time cannot be earlier than 8am")
                        .kind(toast::Kind::Warning),
                ),
                raise(
                    "Error",
                    toast::toast("Event has not been created").kind(toast::Kind::Error),
                ),
            ]
            .spacing(8)
            .into(),

            // shadcn `sonner-description.tsx`.
            "description" => raise(
                "Show Toast",
                created().description("Monday, January 3rd at 6:00pm"),
            )
            .into(),

            // shadcn `sonner-position.tsx`.
            "position" => column![
                row![
                    raise("Top Left", created().position(toast::Position::TopLeft)),
                    raise("Top Center", created().position(toast::Position::TopCenter)),
                    raise("Top Right", created().position(toast::Position::TopRight)),
                ]
                .spacing(8),
                row![
                    raise(
                        "Bottom Left",
                        created().position(toast::Position::BottomLeft)
                    ),
                    raise(
                        "Bottom Center",
                        created().position(toast::Position::BottomCenter)
                    ),
                    raise(
                        "Bottom Right",
                        created().position(toast::Position::BottomRight)
                    ),
                ]
                .spacing(8),
            ]
            .spacing(8)
            .into(),

            _ => unreachable!("unknown sonner preview id: {id}"),
        }
    }
}
