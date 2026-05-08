// API Reference is rendered via the `checkbox-api-reference.md` snippet at
// the bottom of BLOCKS. The live docs page defers the prop matrix to
// Radix's own docs (`Checkbox.checked: boolean | "indeterminate"`,
// `defaultChecked`, `onCheckedChange`, `disabled`, `aria-invalid`); the
// snippet captures the icedxdy-shaped builder API and the `State`
// triplet promotion (`Checked` / `Unchecked` / `Indeterminate`).
//
// Intentional skips per CLAUDE.md: checkbox-rtl (RTL), checkbox-group
// (requires FieldGroup with multiple horizontally-labelled fields — deferred),
// checkbox-table (out of scope).

use iced::Task;

use icedxdy::Theme;
use icedxdy::theme;
use icedxdy::widget::{checkbox, column, field, text};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Checkbox",
    description: "A control that allows the user to toggle between checked and not checked.",
};

pub static BLOCKS: &[page::Block] = &[
    // Hero demo
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/checkbox-demo.md",
    },
    // Preamble: Checked State — interactive demo showing the checked visual.
    page::Block::Example {
        id: "checked-state",
        title: "Checked State",
        description: None,
        snippet: "examples/browser/snippets/checkbox-checked-state.md",
    },
    // Preamble: Invalid State — interactive demo showing the destructive border.
    page::Block::Example {
        id: "invalid-state",
        title: "Invalid State",
        description: None,
        snippet: "examples/browser/snippets/checkbox-invalid-state.md",
    },
    // Examples group
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: None,
                snippet: "examples/browser/snippets/checkbox-basic.md",
            },
            page::Block::Example {
                id: "description",
                title: "Description",
                description: None,
                snippet: "examples/browser/snippets/checkbox-description.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: None,
                snippet: "examples/browser/snippets/checkbox-disabled.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/checkbox-api-reference.md"),
];

// Stable field ids for `BoolChanged` dispatch — independent of block
// position so reordering blocks doesn't break value routing.
const F_BASIC: usize = 0;
const F_DEMO_TERMS_1: usize = 1;
const F_DEMO_TERMS_2: usize = 2;
const F_DEMO_NOTIF_2: usize = 3;
const F_DESC: usize = 4;
const F_CHECKED_STATE: usize = 5;
const F_INVALID_STATE: usize = 6;

pub struct CheckboxPage {
    state: page::State,
    basic: bool,
    demo_terms_1: bool,
    demo_terms_2: bool,
    demo_notif_2: bool,
    desc: bool,
    checked_state: bool,
    invalid_state: bool,
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
            // Checked-state preamble ships pre-checked to show the filled state.
            checked_state: true,
            invalid_state: false,
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
                F_CHECKED_STATE => self.checked_state = v,
                F_INVALID_STATE => self.invalid_state = v,
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
            // Demo (hero): 4 stacked horizontal Fields. Verbatim from
            // `.nova/apps/v4/examples/radix/checkbox-demo.tsx` — every row is
            // `<Field orientation="horizontal">` (row 4 wrapped in
            // FieldLabel-as-card, expressed via `.card(...)`).
            "demo" => {
                let notif_2 = self.demo_notif_2;
                let notif_on_press =
                    Message::Page(pi, page::Message::BoolChanged(F_DEMO_NOTIF_2, !notif_2));
                column![
                    // Row 1: simple Field with Label.
                    field(checkbox(self.demo_terms_1).on_toggle(toggle(F_DEMO_TERMS_1)))
                        .orientation(field::Horizontal)
                        .label("Accept terms and conditions"),
                    // Row 2: Field with FieldContent (label + description).
                    field(checkbox(self.demo_terms_2).on_toggle(toggle(F_DEMO_TERMS_2)))
                        .orientation(field::Horizontal)
                        .label("Accept terms and conditions")
                        .description("By clicking this checkbox, you agree to the terms."),
                    // Row 3: disabled — no `.on_toggle(...)` resolves to
                    // `Status::Disabled`. Label rendered muted to approximate
                    // shadcn's `data-disabled:opacity-50` cascade on the
                    // FieldLabel (color shift, not a true alpha overlay).
                    field(checkbox(false))
                        .orientation(field::Horizontal)
                        .label(text::muted("Enable notifications")),
                    // Row 4: FieldLabel-as-card — bordered + padded card with
                    // primary tint when checked. `.card(notif_2)` drives
                    // active/inactive tint; `.on_press` fires the toggle for
                    // clicks outside the checkbox area.
                    field(checkbox(notif_2).on_toggle(toggle(F_DEMO_NOTIF_2)))
                        .orientation(field::Horizontal)
                        .label("Enable notifications")
                        .description("You can enable or disable notifications at any time.")
                        .card(notif_2)
                        .on_press(notif_on_press),
                ]
                .spacing(20)
                // nova uses `max-w-sm` (max-width 384px, not fixed). iced
                // Column lacks max_width; using fixed 384 as the nearest
                // approximation until our Column gains a max_width builder.
                .width(384)
                .into()
            }

            // Checked State preamble: interactive checkbox pre-checked to
            // show the primary-filled state. Width 224 matches `w-56`.
            "checked-state" => {
                field(checkbox(self.checked_state).on_toggle(toggle(F_CHECKED_STATE)))
                    .orientation(field::Horizontal)
                    .label("Accept terms and conditions")
                    .width(224)
                    .into()
            }

            // Invalid State preamble: destructive border via
            // `theme::checkbox::Invalid`; label tinted destructive to mirror
            // shadcn's `data-invalid:text-destructive` cascade on FieldLabel.
            "invalid-state" => field(
                checkbox(self.invalid_state)
                    .on_toggle(toggle(F_INVALID_STATE))
                    .style(theme::checkbox::Invalid),
            )
            .orientation(field::Horizontal)
            .label(text::label("Accept terms and conditions").style(theme::text::Destructive))
            .width(224)
            .into(),

            // Basic: single Field with Label. Width 224 matches `w-56`.
            "basic" => field(checkbox(self.basic).on_toggle(toggle(F_BASIC)))
                .orientation(field::Horizontal)
                .label("Accept terms and conditions")
                .width(224)
                .into(),

            // Description: pre-checked Field with FieldContent (label +
            // description). Width 288 matches `w-72`. Field's horizontal mode
            // top-aligns automatically when description is present (mirroring
            // shadcn's `has-[>[data-slot=field-content]]:items-start`).
            "description" => field(checkbox(self.desc).on_toggle(toggle(F_DESC)))
                .orientation(field::Horizontal)
                .label("Accept terms and conditions")
                .description("By clicking this checkbox, you agree to the terms and conditions.")
                .width(288)
                .into(),

            // Disabled: no `.on_toggle(...)` resolves to `Status::Disabled` on
            // the checkbox. Label uses `text::muted` to approximate shadcn's
            // `data-disabled:opacity-50` cascade — a color shift rather than a
            // true alpha overlay (real Field-level data-disabled propagation
            // is deferred until the cascade infra lands).
            "disabled" => field(checkbox(false))
                .orientation(field::Horizontal)
                .label(text::muted("Enable notifications"))
                .width(224)
                .into(),

            _ => unreachable!("unknown checkbox preview id: {id}"),
        };

        preview
    }
}
