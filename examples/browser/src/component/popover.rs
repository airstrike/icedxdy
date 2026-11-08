// API Reference is rendered via the `popover-api-reference.md` snippet at
// the bottom of BLOCKS.
//
// Intentional skips:
// - popover-rtl: RTL is not in scope (per CLAUDE.md).

use iced::{Fill, FillPortion, Task};

use icedxdy::widget::field::Orientation;
use icedxdy::widget::{button, column, field, input, popover, row, text};
use icedxdy::{Size, Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Popover",
    description: "Displays rich content in a portal, triggered by a button.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/popover-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A simple popover with a header, title, and description."),
                snippet: "examples/browser/snippets/popover-basic.md",
            },
            page::Block::Example {
                id: "alignments",
                title: "Align",
                description: Some("Use `.align(..)` to align the popover to the trigger."),
                snippet: "examples/browser/snippets/popover-alignments.md",
            },
            page::Block::Example {
                id: "form",
                title: "With Form",
                description: Some("A popover with form fields inside."),
                snippet: "examples/browser/snippets/popover-form.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/popover-api-reference.md"),
];

/// The popover page. Every popover is uncontrolled; its inputs mount fresh
/// each time it opens.
pub struct PopoverPage {
    state: page::State,
}

impl PopoverPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// A `grid-cols-3` row: the label in the first column, the input across
/// the other two.
fn dimension<'a>(label: &'a str, value: &'a str) -> Slot<'a, Message> {
    row![
        text(label).width(FillPortion(1)),
        input("").default_value(value).width(FillPortion(2)),
    ]
    .spacing(16)
    .align_y(iced::Center)
    .into()
}

/// An outline `size="sm"` trigger whose `w-40` panel is aligned `align`.
fn aligned<'a>(label: &'a str, align: popover::Align, content: &'a str) -> Slot<'a, Message> {
    popover(
        move |p| p.trigger(button(label).size(Size::Sm).style(button::Outline)),
        move |_| text(content).into(),
    )
    .align(align)
    .width(160)
    .into()
}

impl page::Page for PopoverPage {
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
            // shadcn `popover-demo.tsx`.
            "demo" => popover(
                |p| p.trigger(button("Open popover").style(button::Outline)),
                |_| {
                    column![
                        column![
                            text("Dimensions").medium(),
                            text::muted("Set the dimensions for the layer."),
                        ]
                        .spacing(8),
                        column![
                            dimension("Width", "100%"),
                            dimension("Max. width", "300px"),
                            dimension("Height", "25px"),
                            dimension("Max. height", "none"),
                        ]
                        .spacing(8),
                    ]
                    .spacing(16)
                    .into()
                },
            )
            .width(320)
            .into(),

            // shadcn `popover-basic.tsx`.
            "basic" => popover(
                |p| p.trigger(button("Open Popover").style(button::Outline)),
                |_| popover::header("Dimensions", "Set the dimensions for the layer."),
            )
            .align(popover::Align::Start)
            .into(),

            // shadcn `popover-alignments.tsx`.
            "alignments" => row![
                aligned("Start", popover::Align::Start, "Aligned to start"),
                aligned("Center", popover::Align::Center, "Aligned to center"),
                aligned("End", popover::Align::End, "Aligned to end"),
            ]
            .spacing(24)
            .into(),

            // shadcn `popover-form.tsx`.
            "form" => popover(
                |p| p.trigger(button("Open Popover").style(button::Outline)),
                |_| {
                    column![
                        popover::header("Dimensions", "Set the dimensions for the layer."),
                        column![
                            field(input("").default_value("100%").width(Fill))
                                .label("Width")
                                .orientation(Orientation::Horizontal),
                            field(input("").default_value("25px").width(Fill))
                                .label("Height")
                                .orientation(Orientation::Horizontal),
                        ]
                        .spacing(16),
                    ]
                    .spacing(10)
                    .into()
                },
            )
            .align(popover::Align::Start)
            .width(256)
            .into(),

            _ => unreachable!("unknown popover preview id: {id}"),
        }
    }
}
