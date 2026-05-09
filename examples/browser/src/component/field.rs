// API Reference is rendered via the `field-api-reference.md` snippet at
// the bottom of BLOCKS. Live docs API Reference covers `FieldSet`,
// `FieldLegend`, `FieldGroup`, `Field.orientation`, `FieldContent`,
// `FieldLabel`, `FieldTitle`, `FieldDescription`, `FieldSeparator`, and
// `FieldError`; the snippet captures the icedxdy-shaped builder API for
// `field(...)` plus the Vertical / Horizontal orientation alignment
// switch and the click-anywhere `on_press` semantic.
//
// Intentional skips per the forms minimap §"Out of scope": field-textarea
// (Textarea not yet ported), field-radio (RadioGroup not yet ported),
// field-fieldset / field-group / field-choice-card (FieldSet, FieldGroup,
// and FieldContent not yet ported), field-rtl (RTL).
//
// The shadcn `field-checkbox.tsx` and `field-switch.tsx` source examples
// use `orientation="horizontal"` — Field's MVP ships vertical-only, so the
// previews below render the checkbox/switch + label + description stacked
// vertically. Horizontal orientation is a follow-up minimap.

use iced::Fill;
use iced::Task;

use icedxdy::Theme;
use icedxdy::theme;
use icedxdy::widget::{checkbox, container, field, input, switch};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Field",
    description: "A vertical wrapper that pairs an input with a label, helper description, and error message — the form-input building block.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/field-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "input-with-error",
                title: "Input with error",
                description: Some(
                    "Surface validation errors below the input. The error message slot accepts plain strings (auto-styled) or pre-styled text.",
                ),
                snippet: "examples/browser/snippets/field-input-with-error.md",
            },
            page::Block::Example {
                id: "checkbox",
                title: "Checkbox",
                description: Some("Wrap a checkbox in a Field to attach a label and description."),
                snippet: "examples/browser/snippets/field-checkbox.md",
            },
            page::Block::Example {
                id: "switch",
                title: "Switch",
                description: Some("Wrap a switch in a Field to attach a label and description."),
                snippet: "examples/browser/snippets/field-switch.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/field-api-reference.md"),
];

// Stable field ids for value-routing — independent of block position so
// reordering blocks doesn't break dispatch.
const F_DEMO: usize = 0;
const F_ERROR: usize = 1;
const F_CHECKBOX: usize = 2;
const F_SWITCH: usize = 3;

pub struct FieldPage {
    state: page::State,
    demo_value: String,
    error_value: String,
    checkbox_on: bool,
    switch_on: bool,
}

impl FieldPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            demo_value: String::new(),
            // Pre-fill with an obviously-invalid value so the error
            // example's destructive border + error message are visible
            // on first paint without the user having to type.
            error_value: String::from("not-an-email"),
            checkbox_on: false,
            switch_on: true,
        })
    }
}

#[granita::reloadable]
impl page::Page for FieldPage {
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
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        match event {
            page::Message::InputChanged(field, value) => match field {
                F_DEMO => {
                    self.demo_value = value;
                }
                F_ERROR => {
                    self.error_value = value;
                }
                _ => {}
            },
            page::Message::BoolChanged(field, v) => match field {
                F_CHECKBOX => self.checkbox_on = v,
                F_SWITCH => self.switch_on = v,
                _ => {}
            },
            _ => {}
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match id {
            // Demo (hero): Field with input + label + description — the
            // canonical Field shape. Mirrors the "Card Number" Field block
            // inside `.nova/apps/v4/examples/radix/field-demo.tsx`
            // (lines 44-56) — the simplest representative slice of the
            // multi-Field demo form.
            "demo" => container(
                field(
                    input("1234 5678 9012 3456", &self.demo_value)
                        .on_input(move |v| {
                            Message::Page(pi, page::Message::InputChanged(F_DEMO, v))
                        })
                        .width(Fill),
                )
                .label("Card Number")
                .description("Enter your 16-digit card number"),
            )
            .width(320)
            .into(),

            // Input with error: Field with input + label + error message.
            // Mirrors `.nova/apps/v4/examples/radix/select-invalid.tsx` (the
            // closest shadcn-stocked FieldError example) adapted to an Input.
            // Caller wires `theme::text_input::Invalid` on the input
            // separately — Field does NOT auto-style its content as invalid
            // (no iced context propagation; per minimap §"Out of scope").
            "input-with-error" => container(
                field(
                    input("you@example.com", &self.error_value)
                        .on_input(move |v| {
                            Message::Page(pi, page::Message::InputChanged(F_ERROR, v))
                        })
                        .width(Fill)
                        .style(theme::text_input::Invalid),
                )
                .label("Email")
                .error("Please enter a valid email address."),
            )
            .width(320)
            .into(),

            // Checkbox: Field wrapping a Checkbox + label + description.
            // Adapted from `.nova/apps/v4/examples/radix/field-checkbox.tsx`
            // — that example uses orientation="horizontal" which the MVP
            // doesn't ship; here the slots stack vertically.
            "checkbox" => container(
                field(checkbox(self.checkbox_on).on_toggle(move |v| {
                    Message::Page(pi, page::Message::BoolChanged(F_CHECKBOX, v))
                }))
                .label("Sync Desktop & Documents folders")
                .description(
                    "Your Desktop & Documents folders are being synced with iCloud Drive.",
                ),
            )
            .width(320)
            .into(),

            // Switch: Field wrapping a Switch + label + description.
            // Adapted from `.nova/apps/v4/examples/radix/field-switch.tsx`
            // — that example uses orientation="horizontal" which the MVP
            // doesn't ship; vertical stack here.
            "switch" => container(
                field(switch(self.switch_on).on_toggle(move |v| {
                    Message::Page(pi, page::Message::BoolChanged(F_SWITCH, v))
                }))
                .label("Multi-factor authentication")
                .description("Require a second verification step at sign-in."),
            )
            .width(320)
            .into(),

            _ => unreachable!("unknown field preview id: {id}"),
        }
    }
}
