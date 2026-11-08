// Intentional skips: radio-group-choice-card (the radio sits right of a
// FieldTitle inside a FieldLabel card), radio-group-fieldset and
// radio-group-invalid (FieldSet, FieldLegend and an invalid item style are
// not ported) and radio-group-rtl (RTL). In radio-group-disabled shadcn's
// `Field data-disabled` also dims the label; icedxdy's field has no
// disabled state yet, so only the item dims.

use iced::Task;

use icedxdy::widget::{column, field, radio_group, text};
use icedxdy::{Horizontal, Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Radio Group",
    description: "A set of checkable buttons, known as radio buttons, where no more than one of the buttons can be checked at a time.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/radio-group-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "description",
                title: "Description",
                description: Some(
                    "Radio group items with a description using the `field` component.",
                ),
                snippet: "examples/browser/snippets/radio-group-description.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some("Use `.disabled(true)` on an item (or the group) to disable it."),
                snippet: "examples/browser/snippets/radio-group-disabled.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/radio-group-api-reference.md"),
];

pub struct RadioGroupPage {
    state: page::State,
}

impl RadioGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// `field-description` typography under a label, beside its item.
fn described<'a>(
    item: impl Into<Slot<'a, Message>>,
    label: &'a str,
    description: &'a str,
) -> Slot<'a, Message> {
    field(item)
        .orientation(Horizontal)
        .label(label)
        .description(description)
        .into()
}

impl page::Page for RadioGroupPage {
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
            // shadcn `radio-group-demo.tsx`.
            "demo" => radio_group(|g| {
                column![
                    field(g.item("default"))
                        .orientation(Horizontal)
                        .label("Default"),
                    field(g.item("comfortable"))
                        .orientation(Horizontal)
                        .label("Comfortable"),
                    field(g.item("compact"))
                        .orientation(Horizontal)
                        .label("Compact"),
                ]
                .spacing(8)
                .into()
            })
            .default_value("comfortable")
            .into(),

            // shadcn `radio-group-description.tsx`.
            "description" => radio_group(|g| {
                column![
                    described(
                        g.item("default"),
                        "Default",
                        "Standard spacing for most use cases."
                    ),
                    described(
                        g.item("comfortable"),
                        "Comfortable",
                        "More space between elements."
                    ),
                    described(
                        g.item("compact"),
                        "Compact",
                        "Minimal spacing for dense layouts."
                    ),
                ]
                .spacing(8)
                .into()
            })
            .default_value("comfortable")
            .into(),

            // shadcn `radio-group-disabled.tsx`.
            "disabled" => radio_group(|g| {
                column![
                    field(g.item("option1").disabled(true))
                        .orientation(Horizontal)
                        .label(text::small("Disabled")),
                    field(g.item("option2"))
                        .orientation(Horizontal)
                        .label(text::small("Option 2")),
                    field(g.item("option3"))
                        .orientation(Horizontal)
                        .label(text::small("Option 3")),
                ]
                .spacing(8)
                .into()
            })
            .default_value("option2")
            .into(),

            _ => unreachable!("unknown radio group preview id: {id}"),
        }
    }
}
