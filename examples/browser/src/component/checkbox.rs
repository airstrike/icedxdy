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

pub struct CheckboxPage {
    state: page::State,
}

impl CheckboxPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
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
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let preview: iced::Element<'_, Message, Theme> = match id {
            // Demo (hero): 4 stacked horizontal Fields. Verbatim from
            // `.nova/apps/v4/examples/radix/checkbox-demo.tsx` — every row
            // uses horizontal Field orientation (row 4 wrapped in
            // FieldLabel-as-card, expressed via `.card(...)`).
            "demo" => {
                column![
                    // Row 1: simple Field with Label.
                    field(checkbox())
                        .orientation(field::Horizontal)
                        .label("Accept terms and conditions"),
                    // Row 2: Field with FieldContent (label + description).
                    field(checkbox().default_checked(true))
                        .orientation(field::Horizontal)
                        .label("Accept terms and conditions")
                        .description("By clicking this checkbox, you agree to the terms."),
                    // Row 3: disabled. Label rendered muted to approximate
                    // shadcn's `data-disabled:opacity-50` cascade on the
                    // FieldLabel (color shift, not a true alpha overlay).
                    field(checkbox().disabled(true))
                        .orientation(field::Horizontal)
                        .label(text::muted("Enable notifications")),
                    // Row 4: FieldLabel-as-card — bordered + padded card,
                    // primary tint while checked; pressing anywhere on it
                    // toggles the checkbox.
                    field(checkbox())
                        .orientation(field::Horizontal)
                        .label("Enable notifications")
                        .description("You can enable or disable notifications at any time.")
                        .card(),
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
            "checked-state" => field(checkbox().default_checked(true))
                .orientation(field::Horizontal)
                .label("Accept terms and conditions")
                .width(224)
                .into(),

            // Invalid State preamble: destructive border via
            // `theme::checkbox::Invalid`; label tinted destructive to mirror
            // shadcn's `data-invalid:text-destructive` cascade on FieldLabel.
            "invalid-state" => field(checkbox().style(theme::checkbox::Invalid))
                .orientation(field::Horizontal)
                .label(text::label("Accept terms and conditions").style(theme::text::Destructive))
                .width(224)
                .into(),

            // Basic: single Field with Label. Width 224 matches `w-56`.
            "basic" => field(checkbox())
                .orientation(field::Horizontal)
                .label("Accept terms and conditions")
                .width(224)
                .into(),

            // Description: pre-checked Field with FieldContent (label +
            // description). Width 288 matches `w-72`. Field's horizontal mode
            // top-aligns automatically when description is present (mirroring
            // shadcn's `has-[>[data-slot=field-content]]:items-start`).
            "description" => field(checkbox().default_checked(true))
                .orientation(field::Horizontal)
                .label("Accept terms and conditions")
                .description("By clicking this checkbox, you agree to the terms and conditions.")
                .width(288)
                .into(),

            // Disabled: `.disabled(true)` paints `Status::Disabled` on the
            // checkbox. Label uses `text::muted` to approximate shadcn's
            // `data-disabled:opacity-50` cascade — a color shift rather than a
            // true alpha overlay (real Field-level data-disabled propagation
            // is deferred until the cascade infra lands).
            "disabled" => field(checkbox().disabled(true))
                .orientation(field::Horizontal)
                .label(text::muted("Enable notifications"))
                .width(224)
                .into(),

            _ => unreachable!("unknown checkbox preview id: {id}"),
        };

        preview
    }
}
