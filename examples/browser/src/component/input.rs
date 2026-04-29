use std::collections::BTreeSet;

use iced::Task;
use iced::widget::Id;
use icedxdy::widget::markdown;

use icedxdy::Theme;
use icedxdy::theme;
use icedxdy::theme::tweak;
use icedxdy::widget::{button, column, input, row, text};

use crate::Message;
use crate::example;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Input",
    description: "A text input component for forms and user data entry with built-in styling and accessibility features.",
    examples: &[
        example::Meta {
            id: "demo",
            title: "Input",
            description: None,
            snippet: "examples/browser/snippets/input-demo.md",
        },
        example::Meta {
            id: "basic",
            title: "Basic",
            description: Some("A simple input with a placeholder."),
            snippet: "examples/browser/snippets/input-basic.md",
        },
        example::Meta {
            id: "field",
            title: "Field",
            description: Some("An input with a label and description."),
            snippet: "examples/browser/snippets/input-field.md",
        },
        example::Meta {
            id: "disabled",
            title: "Disabled",
            description: Some("A disabled input field."),
            snippet: "examples/browser/snippets/input-disabled.md",
        },
        example::Meta {
            id: "inline",
            title: "Inline",
            description: Some("Use an input with a button to create a search input."),
            snippet: "examples/browser/snippets/input-inline.md",
        },
        example::Meta {
            id: "invalid",
            title: "Invalid",
            description: Some("An input with validation errors, styled with a destructive border and focus ring."),
            snippet: "examples/browser/snippets/input-invalid.md",
        },
        example::Meta {
            id: "required",
            title: "Required",
            description: Some("An input with a required label, marked by a destructive asterisk suffix."),
            snippet: "examples/browser/snippets/input-required.md",
        },
    ],
};

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
            state: page::State::new(page_index, &META),
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
    fn page_index(&self) -> usize { self.state.page_index }
    fn meta(&self) -> &'static page::Meta {
        &META
    }

    fn snippets(&self) -> &[Option<Vec<markdown::Item>>] {
        &self.state.snippets
    }

    fn visible(&self) -> &BTreeSet<usize> {
        &self.state.visible
    }

    fn example_ids(&self) -> &[Id] {
        &self.state.example_ids
    }

    fn scrollable_id(&self) -> Id {
        self.state.scrollable_id.clone()
    }

    fn code_expanded(&self) -> &BTreeSet<usize> {
        &self.state.code_expanded
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
                    0 => self.demo_value = value,
                    1 => self.basic_value = value,
                    2 => self.field_value = value,
                    3 => self.inline_value = value,
                    4 => self.invalid_value = value,
                    5 => self.required_value = value,
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

    fn preview(&self, index: usize) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match index {
            // Demo: field with label "API Key", placeholder "sk-...", description
            0 => column([
                text::small("API Key").semibold().into(),
                input("sk-...", &self.demo_value)
                    .on_input(move |v| Message::Page(pi, page::Message::InputChanged(0, v)))
                    .into(),
                text::muted("Your API key is encrypted and stored securely.").into(),
            ])
            .spacing(8)
            .width(320)
            .into(),
            // Basic: simple input with placeholder
            1 => input("Enter text", &self.basic_value)
                .on_input(move |v| Message::Page(pi, page::Message::InputChanged(1, v)))
                .into(),
            // Field: input with label "Username" and description
            2 => column([
                text::small("Username").semibold().into(),
                input("Enter your username", &self.field_value)
                    .on_input(move |v| Message::Page(pi, page::Message::InputChanged(2, v)))
                    .into(),
                text::muted("Choose a unique username for your account.").into(),
            ])
            .spacing(8)
            .width(320)
            .into(),
            // Disabled — no `.on_input(...)` call ⇒ widget is disabled.
            3 => column([
                text::small("Email").semibold().into(),
                input("Email", "").into(),
                text::muted("This field is currently disabled.").into(),
            ])
            .spacing(8)
            .width(320)
            .into(),
            // Inline: input + button side by side
            4 => column([
                row![
                    input("Search...", &self.inline_value)
                        .on_input(move |v| Message::Page(pi, page::Message::InputChanged(3, v))),
                    button("Search").on_press(Message::Page(pi, page::Message::Noop)),
                ]
                .spacing(8)
                .into()
            ])
            .max_width(400.0)
            .into(),
            // Invalid: destructive border + destructive focus ring (aria-invalid).
            // The nova classlist uses `aria-invalid:ring-destructive/20` (20% alpha)
            // but our `tweak::Ring` resolves a `Token` to a full-alpha Color with
            // no per-atom alpha override. Full-alpha destructive demonstrates the
            // visual concept; matching the 0.2 alpha is a follow-up once an
            // alpha-override mechanism lands. See report.
            5 => column([
                text::small("Invalid Input").semibold().into(),
                input("Error", &self.invalid_value)
                    .on_input(move |v| Message::Page(pi, page::Message::InputChanged(4, v)))
                    .style(
                        theme::text_input::Default
                            + tweak::border::Color(tweak::Token::Destructive)
                            + tweak::focus(
                                tweak::border::Color(tweak::Token::Destructive)
                                    + tweak::Ring {
                                        color: tweak::Token::Destructive,
                                        width: 3.0,
                                        offset: 0.0,
                                    },
                            ),
                    )
                    .into(),
                text::muted("This field contains validation errors.").into(),
            ])
            .spacing(8)
            .width(320)
            .into(),
            // Required: label has a destructive `*` suffix; input itself is default style.
            6 => column([
                row![
                    text::small("Required Field").semibold(),
                    text::small(" *").style(theme::text::Destructive),
                ]
                .into(),
                input("This field is required", &self.required_value)
                    .on_input(move |v| Message::Page(pi, page::Message::InputChanged(5, v)))
                    .into(),
                text::muted("This field must be filled out.").into(),
            ])
            .spacing(8)
            .width(320)
            .into(),
            _ => unreachable!(),
        }
    }
}
