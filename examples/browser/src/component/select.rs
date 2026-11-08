// API Reference is rendered via the `select-api-reference.md` snippet at
// the bottom of BLOCKS.
//
// Intentional skips:
// - select-rtl: RTL is not in scope (per CLAUDE.md).
// - select-align-item: Radix's `item-aligned` position (the chosen item
//   laid over the trigger) is not ported; every select opens as `popper`.

use iced::{Fill, Task};

use icedxdy::Theme;
use icedxdy::widget::{field, select};

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Select",
    description: "Displays a list of options for the user to pick from, triggered by a button.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/select-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "groups",
                title: "Groups",
                description: Some(
                    "Use `select::group`, `select::label` and `select::separator` to organize items.",
                ),
                snippet: "examples/browser/snippets/select-groups.md",
            },
            page::Block::Example {
                id: "scrollable",
                title: "Scrollable",
                description: Some("A select with many items that scrolls."),
                snippet: "examples/browser/snippets/select-scrollable.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: None,
                snippet: "examples/browser/snippets/select-disabled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some("Use `.style(select::Invalid)` inside a field with an error."),
                snippet: "examples/browser/snippets/select-invalid.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/select-api-reference.md"),
];

/// The select page. Every select is uncontrolled.
pub struct SelectPage {
    state: page::State,
}

impl SelectPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn fruits() -> [select::Entry; 5] {
    [
        select::item("apple", "Apple"),
        select::item("banana", "Banana"),
        select::item("blueberry", "Blueberry"),
        select::item("grapes", "Grapes"),
        select::item("pineapple", "Pineapple"),
    ]
}

fn zone(label: &str, items: &[(&str, &str)]) -> select::Entry {
    select::group(
        std::iter::once(select::label(label)).chain(
            items
                .iter()
                .map(|(value, label)| select::item(*value, *label)),
        ),
    )
}

impl page::Page for SelectPage {
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
            // shadcn `select-demo.tsx`.
            "demo" => select([select::group(
                std::iter::once(select::label("Fruits")).chain(fruits()),
            )])
            .placeholder("Select a fruit")
            .icon(icon::chevron_down)
            .width(192)
            .into(),

            // shadcn `select-groups.tsx`.
            "groups" => select([
                select::group([
                    select::label("Fruits"),
                    select::item("apple", "Apple"),
                    select::item("banana", "Banana"),
                    select::item("blueberry", "Blueberry"),
                ]),
                select::separator(),
                select::group([
                    select::label("Vegetables"),
                    select::item("carrot", "Carrot"),
                    select::item("broccoli", "Broccoli"),
                    select::item("spinach", "Spinach"),
                ]),
            ])
            .placeholder("Select a fruit")
            .icon(icon::chevron_down)
            .width(192)
            .into(),

            // shadcn `select-scrollable.tsx`.
            "scrollable" => select([
                zone(
                    "North America",
                    &[
                        ("est", "Eastern Standard Time"),
                        ("cst", "Central Standard Time"),
                        ("mst", "Mountain Standard Time"),
                        ("pst", "Pacific Standard Time"),
                        ("akst", "Alaska Standard Time"),
                        ("hst", "Hawaii Standard Time"),
                    ],
                ),
                zone(
                    "Europe & Africa",
                    &[
                        ("gmt", "Greenwich Mean Time"),
                        ("cet", "Central European Time"),
                        ("eet", "Eastern European Time"),
                        ("west", "Western European Summer Time"),
                        ("cat", "Central Africa Time"),
                        ("eat", "East Africa Time"),
                    ],
                ),
                zone(
                    "Asia",
                    &[
                        ("msk", "Moscow Time"),
                        ("ist", "India Standard Time"),
                        ("cst_china", "China Standard Time"),
                        ("jst", "Japan Standard Time"),
                        ("kst", "Korea Standard Time"),
                        ("ist_indonesia", "Indonesia Central Standard Time"),
                    ],
                ),
                zone(
                    "Australia & Pacific",
                    &[
                        ("awst", "Australian Western Standard Time"),
                        ("acst", "Australian Central Standard Time"),
                        ("aest", "Australian Eastern Standard Time"),
                        ("nzst", "New Zealand Standard Time"),
                        ("fjt", "Fiji Time"),
                    ],
                ),
                zone(
                    "South America",
                    &[
                        ("art", "Argentina Time"),
                        ("bot", "Bolivia Time"),
                        ("brt", "Brasilia Time"),
                        ("clt", "Chile Standard Time"),
                    ],
                ),
            ])
            .placeholder("Select a timezone")
            .icon(icon::chevron_down)
            .width(256)
            .into(),

            // shadcn `select-disabled.tsx`.
            "disabled" => select([select::group([
                select::item("apple", "Apple"),
                select::item("banana", "Banana"),
                select::item("blueberry", "Blueberry"),
                select::item("grapes", "Grapes").disabled(true),
                select::item("pineapple", "Pineapple"),
            ])])
            .placeholder("Select a fruit")
            .icon(icon::chevron_down)
            .disabled(true)
            .width(192)
            .into(),

            // shadcn `select-invalid.tsx`.
            "invalid" => field(
                select([select::group([
                    select::item("apple", "Apple"),
                    select::item("banana", "Banana"),
                    select::item("blueberry", "Blueberry"),
                ])])
                .placeholder("Select a fruit")
                .icon(icon::chevron_down)
                .style(select::Invalid)
                .width(Fill),
            )
            .label("Fruit")
            .error("Please select a fruit.")
            .width(192)
            .into(),

            _ => unreachable!("unknown select preview id: {id}"),
        }
    }
}
