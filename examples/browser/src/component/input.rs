use iced::Fill;
use iced::Task;

use icedxdy::Theme;
use icedxdy::rich_text;
use icedxdy::theme;
use icedxdy::theme::tweak::Token;
use icedxdy::widget::rich_text::span;
use icedxdy::widget::{button, column, field, input, row, text, Component};

use crate::Message;
use crate::page;

// Intentional skips per CLAUDE.md: input-rtl (RTL). Field Group, File,
// Grid, Badge, Input Group, Button Group, and Form examples are also
// skipped — they rely on widgets (Field, FieldGroup, InputGroup,
// ButtonGroup, Select) that have not yet been ported.
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
        ],
    },
    page::Block::Md("examples/browser/snippets/input-api-reference.md"),
];

// Stable field ids for `InputChanged` dispatch — independent of block
// position so reordering blocks doesn't break value routing.
const F_DEMO: usize = 0;
const F_BASIC: usize = 1;
const F_FIELD: usize = 2;
const F_INLINE: usize = 3;
const F_INVALID: usize = 4;
const F_REQUIRED: usize = 5;

pub struct InputPage {
    state: page::State,
    demo_value: String,
    basic_value: String,
    field_value: String,
    inline_value: String,
    invalid_value: String,
    required_value: String,
}

impl InputPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            demo_value: String::new(),
            basic_value: String::new(),
            field_value: String::new(),
            inline_value: String::new(),
            invalid_value: String::new(),
            required_value: String::new(),
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
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        match event {
            page::Message::InputChanged(field, value) => {
                match field {
                    F_DEMO => self.demo_value = value,
                    F_BASIC => self.basic_value = value,
                    F_FIELD => self.field_value = value,
                    F_INLINE => self.inline_value = value,
                    F_INVALID => self.invalid_value = value,
                    F_REQUIRED => self.required_value = value,
                    _ => {}
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match id {
            // Demo (hero): Field+Label+Input+Description for an "API Key"
            // form. Verbatim shadcn `input-demo.tsx`.
            "demo" => column![
                field(
                    input("sk-...", &self.demo_value)
                        .on_input(move |v| {
                            Message::Page(pi, page::Message::InputChanged(F_DEMO, v))
                        })
                        .width(Fill),
                )
                .label("API Key")
                .description("Your API key is encrypted and stored securely."),
            ]
            .width(320)
            .into(),

            // Basic: simple input with placeholder. Width 320 matches nova's
            // form input width — Input now defaults to Length::Shrink, so set
            // it explicitly here.
            "basic" => input("Enter text", &self.basic_value)
                .on_input(move |v| Message::Page(pi, page::Message::InputChanged(F_BASIC, v)))
                .width(320)
                .into(),

            // Field: input with label "Username" and description.
            "field" => column![
                field(
                    input("Enter your username", &self.field_value)
                        .on_input(move |v| {
                            Message::Page(pi, page::Message::InputChanged(F_FIELD, v))
                        })
                        .width(Fill),
                )
                .label("Username")
                .description("Choose a unique username for your account."),
            ]
            .width(320)
            .into(),

            // Disabled — no `.on_input(...)` call ⇒ widget is disabled.
            "disabled" => column![
                field(input("Email", "").width(Fill))
                    .label("Email")
                    .description("This field is currently disabled."),
            ]
            .width(320)
            .into(),

            // Invalid: destructive border + destructive focus ring
            // (aria-invalid). First-class `theme::text_input::Invalid`.
            "invalid" => column![
                field(
                    input("Error", &self.invalid_value)
                        .on_input(move |v| {
                            Message::Page(pi, page::Message::InputChanged(F_INVALID, v))
                        })
                        .width(Fill)
                        .style(theme::text_input::Invalid),
                )
                .label("Invalid Input")
                .description("This field contains validation errors."),
            ]
            .width(320)
            .into(),

            // Inline: input + button side by side.
            "inline" => row![
                input("Search...", &self.inline_value)
                    .on_input(move |v| Message::Page(pi, page::Message::InputChanged(F_INLINE, v)))
                    .width(Fill),
                button("Search").on_press(Message::Page(pi, page::Message::Noop)),
            ]
            .spacing(8)
            .width(400)
            .into(),

            // Required: rich-text label with a destructive asterisk.
            "required" => column![field(
                input("This field is required", &self.required_value)
                    .on_input(move |v| {
                        Message::Page(pi, page::Message::InputChanged(F_REQUIRED, v))
                    })
                    .width(Fill),
            )
            .label(Component::from(rich_text![
                span("Required Field ").size(text::size::SM).medium(),
                span("*").size(text::size::SM).color(Token::Destructive),
            ]))
            .description("This field must be filled out."),]
            .width(320)
            .into(),

            _ => unreachable!("unknown input preview id: {id}"),
        }
    }
}
