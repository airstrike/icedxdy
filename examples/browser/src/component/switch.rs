use iced::Task;

use icedxdy::Theme;
use icedxdy::theme;
use icedxdy::widget::{column, row, switch, text};

use crate::Message;
use crate::page;

// Intentional skips per CLAUDE.md: switch-rtl (RTL),
// switch-choice-card (composite of switch + container; defer to a later
// minimap), switch-sizes (size variants are out-of-scope for the MVP —
// shadcn ships `sm` and `default`; vendoring the full size matrix is its
// own design surface).
//
// TODO Phase 4: htmlFor label-click-toggle association — shadcn's Field
// provides `htmlFor` semantics so clicking the label toggles the switch.
// iced has no equivalent; will wire once Field MVP lands.
pub static META: page::Meta = page::Meta {
    title: "Switch",
    description: "A control that allows the user to toggle between checked and not checked.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "description",
                title: "Description",
                description: Some(
                    "Pair a switch with a label and supporting description.",
                ),
                snippet: "examples/browser/snippets/switch-description.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some("A disabled switch cannot be toggled."),
                snippet: "examples/browser/snippets/switch-disabled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some(
                    "Use the invalid variant to surface validation errors with a destructive border.",
                ),
                snippet: "examples/browser/snippets/switch-invalid.md",
            },
        ],
    },
];

// Stable field ids for `BoolChanged` dispatch — independent of block
// position so reordering blocks doesn't break value routing.
const F_DEMO: usize = 0;
const F_DESC: usize = 1;
const F_INVALID: usize = 2;

pub struct SwitchPage {
    state: page::State,
    demo: bool,
    desc: bool,
    invalid: bool,
}

impl SwitchPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            demo: false,
            desc: false,
            invalid: false,
        })
    }
}

impl page::Page for SwitchPage {
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
                F_DEMO => self.demo = v,
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
        let toggle =
            move |field: usize| move |b: bool| Message::Page(pi, page::Message::BoolChanged(field, b));

        let preview: iced::Element<'_, Message, Theme> = match id {
            // Demo (hero): switch + plain inline label. Verbatim from
            // `.nova/apps/v4/examples/radix/switch-demo.tsx`.
            "demo" => row![
                switch(self.demo).on_toggle(toggle(F_DEMO)),
                text::label("Airplane Mode"),
            ]
            .spacing(8)
            .into(),

            // Description: horizontal field with label + muted description on
            // the LEFT and the switch on the RIGHT. Verbatim from
            // `.nova/apps/v4/examples/radix/switch-description.tsx`.
            // Field/FieldContent/FieldLabel/FieldDescription are not yet
            // shipped — Phase 4 lands them; for now we hand-roll the row +
            // column + muted-description shape that Field will eventually
            // wrap.
            // TODO Phase 4: max-width 384 (currently fixed width — Row lacks
            // max_width(); swap .width(384) to .max_width(384) once available).
            "description" => row![
                column![
                    text::label("Share across devices"),
                    text::muted(
                        "Focus is shared across devices, and turns off when you leave the app.",
                    ),
                ]
                .spacing(2)
                .width(iced::Fill),
                switch(self.desc).on_toggle(toggle(F_DESC)),
            ]
            .spacing(8)
            .width(384)
            .into(),

            // Disabled: no `.on_toggle(...)` call ⇒ widget is disabled.
            // Disabled label: nova cascades opacity-50 via data-disabled on Field.
            // text::muted approximates the dimming today; text::label (Phase 3)
            // + a proper opacity tweak (Phase 4) will land the exact spec.
            "disabled" => row![
                switch(false),
                text::muted("Disabled").medium(),
            ]
            .spacing(8)
            .into(),

            // Invalid: destructive border via the `theme::switch::Invalid`
            // variant, matching shadcn's `aria-invalid` styling. Layout
            // mirrors the Description example.
            // TODO Phase 4: max-width 384 (currently fixed width — Row lacks
            // max_width(); swap .width(384) to .max_width(384) once available).
            // Invalid label: nova cascades text-destructive via data-invalid on
            // Field. text::error (Phase 3) will consolidate this idiom.
            "invalid" => row![
                column![
                    text::label("Accept terms and conditions")
                        .style(theme::text::Destructive),
                    text::muted("You must accept the terms and conditions to continue."),
                ]
                .spacing(2)
                .width(iced::Fill),
                switch(self.invalid)
                    .on_toggle(toggle(F_INVALID))
                    .style(theme::switch::Invalid),
            ]
            .spacing(8)
            .width(384)
            .into(),

            _ => unreachable!("unknown switch preview id: {id}"),
        };

        preview
    }
}
