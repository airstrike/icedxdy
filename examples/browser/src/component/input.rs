// TODO: API Reference section once a table widget lands.
//
// The live docs page does not surface a structured API Reference table;
// it defers prop documentation to Radix and the source. Captured rows
// (from usage examples on the page): the HTML `type` attribute (default
// `text`; accepts any HTML input type), `disabled` (default `false`),
// `required` (default `false`), and `aria-invalid` (validation styling).
// Captured here so the gap is auditable when the table widget lands.
//
// Intentional skips per CLAUDE.md: input-rtl (RTL). Field Group, File,
// Grid, Input Group, Button Group, and Form examples are also skipped —
// they rely on widgets (FieldGroup, InputGroup, ButtonGroup, Select)
// that have not yet been ported.

use iced::Fill;
use iced::Task;

use icedxdy::Theme;
use icedxdy::rich_text;
use icedxdy::theme;
use icedxdy::theme::tweak::Token;
use icedxdy::widget::rich_text::span;
use icedxdy::widget::{Slot, badge, button, field, input, row, space, text};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Input",
    description: "A text input component for forms and user data entry with built-in styling and accessibility features.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "",
    },
    page::Block::Md("examples/browser/snippets/input-usage.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A simple input with a placeholder."),
                snippet: "examples/browser/snippets/input-basic.md",
            },
            page::Block::Example {
                id: "field",
                title: "Field",
                description: Some("An input with a label and description."),
                snippet: "examples/browser/snippets/input-field.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some("A disabled input field."),
                snippet: "examples/browser/snippets/input-disabled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some(
                    "An input with validation errors, styled with a destructive border and focus ring.",
                ),
                snippet: "examples/browser/snippets/input-invalid.md",
            },
            page::Block::Example {
                id: "inline",
                title: "Inline",
                description: Some("Use an input with a button to create a search input."),
                snippet: "examples/browser/snippets/input-inline.md",
            },
            page::Block::Example {
                id: "required",
                title: "Required",
                description: Some(
                    "An input with a required label, marked by a destructive asterisk suffix.",
                ),
                snippet: "examples/browser/snippets/input-required.md",
            },
            page::Block::Example {
                id: "badge",
                title: "Badge",
                description: Some(
                    "A field label with a trailing badge to flag the input's status.",
                ),
                snippet: "examples/browser/snippets/input-badge.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/input-api-reference.md"),
];

pub struct InputPage {
    state: page::State,
}

impl InputPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for InputPage {
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
        let pi = self.state.page_index;
        match id {
            // Demo (hero): Field+Label+Input+Description for an "API Key"
            // form. Verbatim shadcn `input-demo.tsx`.
            "demo" => field(input("sk-...").width(Fill))
                .label("API Key")
                .description("Your API key is encrypted and stored securely.")
                .width(320)
                .into(),

            // Basic: simple input with placeholder. Width 320 matches nova's
            // form input width — Input now defaults to Length::Shrink, so set
            // it explicitly here.
            "basic" => input("Enter text").width(320).into(),

            // Field: input with label "Username" and description.
            "field" => field(input("Enter your username").width(Fill))
                .label("Username")
                .description("Choose a unique username for your account.")
                .width(320)
                .into(),

            // Disabled: nova `disabled` on the input.
            "disabled" => field(input("Email").width(Fill).disabled(true))
                .label("Email")
                .description("This field is currently disabled.")
                .width(320)
                .into(),

            // Invalid: destructive border + destructive focus ring
            // (aria-invalid). First-class `theme::text_input::Invalid`.
            "invalid" => field(input("Error").width(Fill).style(theme::text_input::Invalid))
                .label("Invalid Input")
                .description("This field contains validation errors.")
                .width(320)
                .into(),

            // Inline: input + button side by side.
            "inline" => row![
                input("Search...").width(Fill),
                button("Search").on_press(Message::Page(pi, page::Message::Noop)),
            ]
            .spacing(8)
            .width(400)
            .into(),

            // Required: rich-text label with a destructive asterisk.
            "required" => field(input("This field is required").width(Fill))
                .label(Slot::from(rich_text![
                    span("Required Field ").size(text::size::SM).medium(),
                    span("*").size(text::size::SM).color(Token::Destructive),
                ]))
                .description("This field must be filled out.")
                .width(320)
                .into(),

            // Badge: a label with a trailing `Beta` badge pushed flush right
            // (shadcn `ml-auto`). Verbatim shadcn `input-badge.tsx`.
            "badge" => field(input("https://api.example.com/webhook").width(Fill))
                .label(Slot::from(
                    row![
                        text::label("Webhook URL"),
                        space::horizontal(),
                        badge("Beta").style(badge::Secondary),
                    ]
                    .width(Fill),
                ))
                .width(320)
                .into(),

            _ => unreachable!("unknown input preview id: {id}"),
        }
    }
}
