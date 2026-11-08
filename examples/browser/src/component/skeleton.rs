// shadcn's skeleton pulses (`animate-pulse`); icedxdy's is static.

use iced::{Fill, FillPortion, Task, Widget};

use icedxdy::card;
use icedxdy::widget::{Column, aspect_ratio, column, row, skeleton, space};
use icedxdy::{Gap, Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Skeleton",
    description: "Use to show a placeholder while content is loading.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/skeleton-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "avatar",
                title: "Avatar",
                description: None,
                snippet: "examples/browser/snippets/skeleton-avatar.md",
            },
            page::Block::Example {
                id: "card",
                title: "Card",
                description: None,
                snippet: "examples/browser/snippets/skeleton-card.md",
            },
            page::Block::Example {
                id: "text",
                title: "Text",
                description: None,
                snippet: "examples/browser/snippets/skeleton-text.md",
            },
            page::Block::Example {
                id: "form",
                title: "Form",
                description: None,
                snippet: "examples/browser/snippets/skeleton-form.md",
            },
            page::Block::Example {
                id: "table",
                title: "Table",
                description: None,
                snippet: "examples/browser/snippets/skeleton-table.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/skeleton-api-reference.md"),
];

/// `rounded-full`.
const FULL: f32 = u32::MAX as f32;

pub struct SkeletonPage {
    state: page::State,
}

impl SkeletonPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// A 16px line spanning `part` of `whole` of its parent's width
/// (`w-2/3` is `line(2, 3)`).
fn line<'a>(part: u16, whole: u16) -> Slot<'a, Message> {
    row![
        skeleton().height(16).width(FillPortion(part)),
        space::horizontal().width(FillPortion(whole - part)),
    ]
    .width(Fill)
    .into()
}

impl page::Page for SkeletonPage {
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
            // shadcn `skeleton-demo.tsx`.
            "demo" => row![
                skeleton().width(48).height(48).radius(FULL),
                column![
                    skeleton().width(250).height(16),
                    skeleton().width(200).height(16),
                ]
                .spacing(8),
            ]
            .spacing(16)
            .align_y(iced::Center)
            .into(),

            // shadcn `skeleton-avatar.tsx`.
            "avatar" => row![
                skeleton().width(40).height(40).radius(FULL),
                column![
                    skeleton().width(150).height(16),
                    skeleton().width(100).height(16),
                ]
                .spacing(8),
            ]
            .spacing(16)
            .align_y(iced::Center)
            .into(),

            // shadcn `skeleton-card.tsx`.
            "card" => card(
                column![
                    card::header(line(2, 3)).description(line(1, 2)),
                    card::content(aspect_ratio(
                        skeleton().width(Fill).height(Fill),
                        16.0 / 9.0
                    )),
                ]
                .spacing(Gap::G4)
                .padding(iced::Padding::ZERO.top(Gap::G4).bottom(Gap::G4))
                .width(Fill),
            )
            .width(320.0)
            .boxed(),

            // shadcn `skeleton-text.tsx`.
            "text" => column![
                skeleton().width(Fill).height(16),
                skeleton().width(Fill).height(16),
                line(3, 4),
            ]
            .spacing(8)
            .width(Fill)
            .max_width(320.0)
            .into(),

            // shadcn `skeleton-form.tsx`.
            "form" => column![
                column![
                    skeleton().width(80).height(16),
                    skeleton().width(Fill).height(32),
                ]
                .spacing(12),
                column![
                    skeleton().width(96).height(16),
                    skeleton().width(Fill).height(32),
                ]
                .spacing(12),
                skeleton().width(96).height(32),
            ]
            .spacing(28)
            .width(Fill)
            .max_width(320.0)
            .into(),

            // shadcn `skeleton-table.tsx`.
            "table" => Column::with_children((0..5).map(|_| {
                row![
                    skeleton().width(Fill).height(16),
                    skeleton().width(96).height(16),
                    skeleton().width(80).height(16),
                ]
                .spacing(16)
                .into()
            }))
            .spacing(8)
            .width(Fill)
            .max_width(384.0)
            .into(),

            _ => unreachable!("unknown skeleton preview id: {id}"),
        }
    }
}
