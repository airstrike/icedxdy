// Intentional skips: spinner-input-group (InputGroup is not ported) and
// spinner-custom (it swaps in a different lucide icon spun with
// `animate-spin`; iced has no CSS animation to spin an arbitrary icon).

use iced::{Shrink, Task};

use icedxdy::widget::{badge, button, column, empty, item, row, spinner, text};
use icedxdy::{Size, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Spinner",
    description: "An indicator that can be used to show a loading state.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/spinner-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "size",
                title: "Size",
                description: Some("Use `.size(..)` to change the size of the spinner."),
                snippet: "examples/browser/snippets/spinner-size.md",
            },
            page::Block::Example {
                id: "button",
                title: "Button",
                description: Some("Add a spinner to a button to indicate a loading state."),
                snippet: "examples/browser/snippets/spinner-button.md",
            },
            page::Block::Example {
                id: "badge",
                title: "Badge",
                description: Some("Add a spinner to a badge to indicate a loading state."),
                snippet: "examples/browser/snippets/spinner-badge.md",
            },
            page::Block::Example {
                id: "empty",
                title: "Empty",
                description: Some("Add a spinner to an empty state to indicate a loading state."),
                snippet: "examples/browser/snippets/spinner-empty.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/spinner-api-reference.md"),
];

pub struct SpinnerPage {
    state: page::State,
}

impl SpinnerPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for SpinnerPage {
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
            // shadcn `spinner-demo.tsx`.
            "demo" => column![
                item::item([
                    item::media(spinner()).into(),
                    item::content([item::title("Processing payment...").into()]).into(),
                    item::content([text::small("$100.00").into()])
                        .width(Shrink)
                        .into(),
                ])
                .style(item::Muted)
            ]
            .width(iced::Fill)
            .max_width(320.0)
            .into(),

            // shadcn `spinner-size.tsx`: `size-3`, `size-4`, `size-6`, `size-8`.
            "size" => row![
                spinner().size(12.0),
                spinner().size(16.0),
                spinner().size(24.0),
                spinner().size(32.0),
            ]
            .spacing(24)
            .align_y(iced::Center)
            .into(),

            // shadcn `spinner-button.tsx`.
            "button" => column![
                button![spinner(), "Loading..."]
                    .size(Size::Sm)
                    .disabled(true),
                button![spinner(), "Please wait"]
                    .size(Size::Sm)
                    .style(button::Outline)
                    .disabled(true),
                button![spinner(), "Processing"]
                    .size(Size::Sm)
                    .style(button::Secondary)
                    .disabled(true),
            ]
            .spacing(16)
            .align_x(iced::Center)
            .into(),

            // shadcn `spinner-badge.tsx`.
            "badge" => row![
                badge![spinner().size(12.0), "Syncing"],
                badge![spinner().size(12.0), "Updating"].style(badge::Secondary),
                badge![spinner().size(12.0), "Processing"].style(badge::Outline),
            ]
            .spacing(16)
            .align_y(iced::Center)
            .into(),

            // shadcn `spinner-empty.tsx`.
            "empty" => empty::empty()
                .media(empty::media(spinner()).style(empty::Icon))
                .title("Processing your request")
                .description("Please wait while we process your request. Do not refresh the page.")
                .content(button("Cancel").style(button::Outline).size(Size::Sm))
                .into(),

            _ => unreachable!("unknown spinner preview id: {id}"),
        }
    }
}
