use std::collections::BTreeSet;

use iced::Task;
use iced::widget::Id;
use icedxdy::widget::markdown;

use icedxdy::Theme;
use icedxdy::widget::{button, input, text};

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
            id: "with-button",
            title: "Inline",
            description: Some("Use an input with a button to create a search input."),
            snippet: "examples/browser/snippets/input-with-button.md",
        },
    ],
};

pub struct InputPage {
    state: page::State,
    demo_value: String,
    basic_value: String,
    field_value: String,
    with_button_value: String,
}

impl InputPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, &META),
            demo_value: String::new(),
            basic_value: String::new(),
            field_value: String::new(),
            with_button_value: String::new(),
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
                    3 => self.with_button_value = value,
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
            0 => icedxdy::widget::column([
                text::small("API Key").semibold().into(),
                input::input("sk-...", &self.demo_value)
                    .on_input(move |v| Message::Page(pi, page::Message::InputChanged(0, v)))
                    .into(),
                text::muted("Your API key is encrypted and stored securely.").into(),
            ])
            .spacing(8)
            .width(320)
            .into(),
            // Basic: simple input with placeholder
            1 => input::input("Enter text", &self.basic_value)
                .on_input(move |v| Message::Page(pi, page::Message::InputChanged(1, v)))
                .into(),
            // Field: input with label "Username" and description
            2 => icedxdy::widget::column([
                text::small("Username").semibold().into(),
                input::input("Enter your username", &self.field_value)
                    .on_input(move |v| Message::Page(pi, page::Message::InputChanged(2, v)))
                    .into(),
                text::muted("Choose a unique username for your account.").into(),
            ])
            .spacing(8)
            .width(320)
            .into(),
            // Disabled
            3 => icedxdy::widget::column([
                text::small("Email").semibold().into(),
                input::input("Email", "").disabled().into(),
                text::muted("This field is currently disabled.").into(),
            ])
            .spacing(8)
            .width(320)
            .into(),
            // Inline: input + button side by side
            4 => icedxdy::widget::column([icedxdy::widget::row()
                .push(
                    input::input("Search...", &self.with_button_value)
                        .on_input(move |v| Message::Page(pi, page::Message::InputChanged(3, v)))
                        .into(),
                )
                .push(
                    button("Search")
                        .on_press(Message::Page(pi, page::Message::Noop))
                        .into(),
                )
                .spacing(8)
                .into()])
            .max_width(400.0)
            .into(),
            _ => unreachable!(),
        }
    }
}
