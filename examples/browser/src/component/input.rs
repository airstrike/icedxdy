use std::collections::BTreeSet;

use iced::widget::{Id, markdown};
use iced::Task;

use icedxdy::widget::{button, input, text};
use icedxdy::Theme;

use crate::example;
use crate::page;
use crate::Message;

pub static META: page::Meta = page::Meta {
    title: "Input",
    description: "Displays a form input field or a component that looks like an input.",
    examples: &[
        example::Meta { id: "demo", title: "Input", description: None, snippet: "examples/browser/snippets/input-default.md" },
        example::Meta { id: "disabled", title: "Disabled", description: Some("A disabled input field."), snippet: "examples/browser/snippets/input-disabled.md" },
        example::Meta { id: "with-label", title: "With Label", description: Some("An input with a label above it."), snippet: "examples/browser/snippets/input-with-label.md" },
        example::Meta { id: "with-button", title: "With Button", description: Some("An input paired with a button."), snippet: "examples/browser/snippets/input-with-button.md" },
    ],
};

pub struct InputPage {
    state: page::State,
    default_value: String,
    email_value: String,
    with_button_value: String,
}

impl InputPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, &META),
            default_value: String::new(),
            email_value: String::new(),
            with_button_value: String::new(),
        })
    }
}

impl page::Page for InputPage {
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

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Event) -> Task<Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        match event {
            page::Event::InputChanged(field, value) => {
                match field {
                    0 => self.default_value = value,
                    1 => self.email_value = value,
                    2 => self.with_button_value = value,
                    _ => {}
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn preview(&self, index: usize) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match index {
            0 => {
                input::input("Type something...", &self.default_value)
                    .on_input(move |v| Message::Page(pi, page::Event::InputChanged(0, v)))
                    .into()
            }
            1 => {
                input::input("Disabled input", "")
                    .disabled()
                    .into()
            }
            2 => {
                icedxdy::widget::column([
                    text::small("Email").into(),
                    input::input("Email", &self.email_value)
                        .on_input(move |v| Message::Page(pi, page::Event::InputChanged(1, v)))
                        .into(),
                ])
                .spacing(8)
                .width(320)
                .into()
            }
            3 => {
                icedxdy::widget::column([
                    icedxdy::widget::row()
                        .push(
                            input::input("Email", &self.with_button_value)
                                .on_input(move |v| Message::Page(pi, page::Event::InputChanged(2, v)))
                                .into(),
                        )
                        .push(button("Subscribe").on_press(Message::Page(pi, page::Event::Noop)).into())
                        .spacing(8)
                        .into(),
                ])
                .max_width(400.0)
                .into()
            }
            _ => unreachable!(),
        }
    }
}
