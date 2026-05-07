use iced::Task;

use icedxdy::Theme;
use icedxdy::theme;
use icedxdy::widget::{checkbox, column, field, row, text};

use crate::Message;
use crate::page;

// Intentional skips per CLAUDE.md: checkbox-rtl (RTL), checkbox-group
// (requires Field — defer to Phase 4), checkbox-table (out of scope).
pub static META: page::Meta = page::Meta {
    title: "Checkbox",
    description: "A control that allows the user to toggle between checked and not checked.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/checkbox-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A simple checkbox with a label."),
                snippet: "examples/browser/snippets/checkbox-basic.md",
            },
            page::Block::Example {
                id: "description",
                title: "Description",
                description: Some(
                    "Pair a checkbox with a label and supporting description.",
                ),
                snippet: "examples/browser/snippets/checkbox-description.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some("A disabled checkbox cannot be toggled."),
                snippet: "examples/browser/snippets/checkbox-disabled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some(
                    "Use the invalid variant to surface validation errors with a destructive border.",
                ),
                snippet: "examples/browser/snippets/checkbox-invalid.md",
            },
        ],
    },
];

// Stable field ids for `BoolChanged` dispatch — independent of block
// position so reordering blocks doesn't break value routing.
const F_BASIC: usize = 0;
const F_DEMO_TERMS_1: usize = 1;
const F_DEMO_TERMS_2: usize = 2;
const F_DEMO_NOTIF_2: usize = 3;
const F_DESC: usize = 4;
const F_INVALID: usize = 5;

pub struct CheckboxPage {
    state: page::State,
    basic: bool,
    demo_terms_1: bool,
    demo_terms_2: bool,
    demo_notif_2: bool,
    desc: bool,
    invalid: bool,
}

impl CheckboxPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            basic: false,
            // Demo-row 2 ships pre-checked (`defaultChecked` in shadcn).
            demo_terms_1: false,
            demo_terms_2: true,
            demo_notif_2: false,
            // Description ships pre-checked (`defaultChecked` in shadcn).
            desc: true,
            invalid: false,
        })
    }
}

impl page::Page for CheckboxPage {
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
        if let page::Message::BoolChanged(field, v) = event {
            match field {
                F_BASIC => self.basic = v,
                F_DEMO_TERMS_1 => self.demo_terms_1 = v,
                F_DEMO_TERMS_2 => self.demo_terms_2 = v,
                F_DEMO_NOTIF_2 => self.demo_notif_2 = v,
                F_DESC => self.desc = v,
                F_INVALID => self.invalid = v,
                _ => {}
            }
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        let toggle = move |field: usize| {
            move |b: bool| Message::Page(pi, page::Message::BoolChanged(field, b))
        };

        let preview: iced::Element<'_, Message, Theme> = match id {
            // Demo (hero): 4 stacked field-shaped rows. Verbatim from
            // `.nova/apps/v4/examples/radix/checkbox-demo.tsx`.
            "demo" => {
                let notif_2 = self.demo_notif_2;
                // Pre-compute the on_press message (set to opposite state) so
                // we can pass it by value, while the on_toggle closure is built
                // fresh for the checkbox handler.
                let notif_on_press = Message::Page(pi, page::Message::BoolChanged(F_DEMO_NOTIF_2, !notif_2));
                column![
                    row![
                        checkbox(self.demo_terms_1).on_toggle(toggle(F_DEMO_TERMS_1)),
                        text::label("Accept terms and conditions"),
                    ]
                    .spacing(8),
                    row![
                        checkbox(self.demo_terms_2).on_toggle(toggle(F_DEMO_TERMS_2)),
                        column![
                            text::label("Accept terms and conditions"),
                            text::muted("By clicking this checkbox, you agree to the terms."),
                        ]
                        .spacing(2),
                    ]
                    .spacing(8),
                    // Row 3: disabled — no `.on_toggle(...)`. FieldLabel inherits
                    // opacity-50 via `data-disabled`; hand-rolled with `text::muted`.
                    row![
                        checkbox(false),
                        text::muted("Enable notifications"),
                    ]
                    .spacing(8),
                    // Row 4: FieldLabel-as-card — bordered + padded card with
                    // primary tint when checked. Matches shadcn checkbox-demo.tsx
                    // lines 40-50 (`<FieldLabel><Field orientation="horizontal">`).
                    // `.card(notif_2)` drives active/inactive tint. `.on_press`
                    // fires the toggle for clicks outside the checkbox area.
                    field::field(
                        checkbox(notif_2).on_toggle(toggle(F_DEMO_NOTIF_2)),
                    )
                    .orientation(field::Orientation::Horizontal)
                    .label("Enable notifications")
                    .description("You can enable or disable notifications at any time.")
                    .card(notif_2)
                    .on_press(notif_on_press),
                ]
                .spacing(20)
                // nova uses `max-w-sm` (max-width 384px, not fixed).
                // iced Column lacks max_width; using fixed 384 as the nearest
                // approximation until our Column gains a max_width builder.
                .width(384)
                .into()
            }

            // Basic: single checkbox + label. Width 224 matches shadcn's
            // `w-56` constraint on the parent FieldGroup.
            "basic" => row![
                checkbox(self.basic).on_toggle(toggle(F_BASIC)),
                text::label("Accept terms and conditions"),
            ]
            .spacing(8)
            .width(224)
            .into(),

            // Description: checked by default, label + muted description.
            "description" => row![
                checkbox(self.desc).on_toggle(toggle(F_DESC)),
                column![
                    text::label("Accept terms and conditions"),
                    text::muted(
                        "By clicking this checkbox, you agree to the terms and conditions.",
                    ),
                ]
                .spacing(2),
            ]
            .spacing(8)
            .width(288)
            .into(),

            // Disabled: no `.on_toggle(...)` call ⇒ widget is disabled.
            // FieldLabel inherits opacity-50 via `data-disabled`; hand-rolled
            // with `text::muted`.
            // TODO Phase 4: replace with proper opacity-50 cascade once Field
            // propagates disabled state to label.
            "disabled" => row![
                checkbox(false),
                text::muted("Enable notifications"),
            ]
            .spacing(8)
            .width(224)
            .into(),

            // Invalid: destructive border via the `theme::checkbox::Invalid`
            // variant, mirroring shadcn's `aria-invalid` styling.
            // FieldLabel inherits `text-destructive` via `data-invalid`;
            // hand-rolled with `.style(theme::text::Destructive)`.
            "invalid" => row![
                checkbox(self.invalid)
                    .on_toggle(toggle(F_INVALID))
                    .style(theme::checkbox::Invalid),
                text::label("Accept terms and conditions")
                    .style(theme::text::Destructive),
            ]
            .spacing(8)
            .width(224)
            .into(),

            _ => unreachable!("unknown checkbox preview id: {id}"),
        };

        preview
    }
}
