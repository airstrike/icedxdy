use std::path::PathBuf;

use iced::widget::{column, markdown};
use iced::{Fill, Font, Pixels, Task};

use icedxdy::widget::{button, container, input, text};
use icedxdy::{Component, theme};

/// Height for preview containers (shadcn uses h-72 = 288px)
const PREVIEW_HEIGHT: f32 = 288.0;

#[derive(Debug, Clone)]
pub enum Message {
    DefaultChanged(String),
    EmailChanged(String),
    WithButtonChanged(String),
    SnippetLoaded(Example, Result<String, String>),
    LinkClicked(markdown::Uri),
    Submit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
    Default,
    Disabled,
    WithLabel,
    WithButton,
}

impl Example {
    const ALL: &[Example] = &[
        Example::Default,
        Example::Disabled,
        Example::WithLabel,
        Example::WithButton,
    ];

    fn snippet_path(&self) -> PathBuf {
        let name = match self {
            Example::Default => "input-default",
            Example::Disabled => "input-disabled",
            Example::WithLabel => "input-with-label",
            Example::WithButton => "input-with-button",
        };
        PathBuf::from(format!("examples/browser/snippets/{}.md", name))
    }

    fn title(&self) -> &'static str {
        match self {
            Example::Default => "Input",
            Example::Disabled => "Disabled",
            Example::WithLabel => "With Label",
            Example::WithButton => "With Button",
        }
    }

    fn description(&self) -> Option<&'static str> {
        match self {
            Example::Disabled => Some("A disabled input field."),
            Example::WithLabel => Some("An input with a label above it."),
            Example::WithButton => Some("An input paired with a button."),
            _ => None,
        }
    }

    fn view<'a>(&self, state: &'a State) -> Component<'a, Message> {
        match self {
            Example::Default => {
                input::input("Type something...", &state.default_value)
                    .on_input(Message::DefaultChanged)
                    .into()
            }
            Example::Disabled => {
                input::input("Disabled input", "")
                    .disabled()
                    .into()
            }
            Example::WithLabel => {
                icedxdy::widget::column([
                    text::small("Email").into(),
                    input::input("Email", &state.email_value)
                        .on_input(Message::EmailChanged)
                        .into(),
                ])
                .spacing(8)
                .width(320)
                .into()
            }
            Example::WithButton => {
                icedxdy::widget::column([
                    icedxdy::widget::row()
                        .push(
                            input::input("Email", &state.with_button_value)
                                .on_input(Message::WithButtonChanged)
                                .into(),
                        )
                        .push(button("Subscribe").on_press(Message::Submit).into())
                        .spacing(8)
                        .into(),
                ])
                .max_width(400.0)
                .into()
            }
        }
    }
}

pub struct State {
    default_value: String,
    email_value: String,
    with_button_value: String,
    snippets: Vec<(Example, Option<Vec<markdown::Item>>)>,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let snippets: Vec<_> = Example::ALL.iter().map(|v| (*v, None)).collect();

        let tasks = Example::ALL.iter().map(|example| {
            let example = *example;
            let path = example.snippet_path();
            Task::perform(
                async move {
                    tokio::fs::read_to_string(path)
                        .await
                        .map_err(|e| e.to_string())
                },
                move |result| Message::SnippetLoaded(example, result),
            )
        });

        (
            Self {
                default_value: String::new(),
                email_value: String::new(),
                with_button_value: String::new(),
                snippets,
            },
            Task::batch(tasks),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DefaultChanged(value) => {
                self.default_value = value;
                Task::none()
            }
            Message::EmailChanged(value) => {
                self.email_value = value;
                Task::none()
            }
            Message::WithButtonChanged(value) => {
                self.with_button_value = value;
                Task::none()
            }
            Message::Submit => Task::none(),
            Message::SnippetLoaded(example, result) => {
                if let Some((_, snippet)) = self.snippets.iter_mut().find(|(e, _)| *e == example)
                    && let Ok(content) = result
                {
                    *snippet = Some(markdown::parse(&content).collect());
                }
                Task::none()
            }
            Message::LinkClicked(url) => {
                let _ = url;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Component<'_, Message> {
        let mut items: Vec<Component<'_, Message>> = Vec::new();

        for (i, (example, snippet)) in self.snippets.iter().enumerate() {
            let preview = container(example.view(self))
                .center_x(Fill)
                .center_y(Fill)
                .width(Fill)
                .height(PREVIEW_HEIGHT)
                .padding(40)
                .style(theme::container::card_preview);

            let code: Component<'_, Message> = snippet
                .as_ref()
                .map(|s| {
                    let mut style = markdown::Style::from(&iced::Theme::TokyoNight);
                    style.code_block_font = Font::MONOSPACE;

                    let settings = markdown::Settings {
                        code_size: Pixels(13.0),
                        ..markdown::Settings::with_style(style)
                    };

                    markdown::view(s, settings).map(Message::LinkClicked).into()
                })
                .unwrap_or_else(|| text::muted("Loading...").into());

            let code = container(code)
                .padding([12, 16])
                .style(theme::container::code_block);

            let card = container(column![preview, code])
                .style(theme::container::card_muted)
                .width(Fill);

            if i == 0 {
                items.push(column![card].into());
            } else {
                let mut section = column![text::h3(example.title())].spacing(16);

                if let Some(desc) = example.description() {
                    section = section.push(text::p(desc).style(theme::text::muted));
                }

                let section_card: Component<'_, Message> = section.push(card).spacing(16).into();
                items.push(section_card);
            }
        }

        icedxdy::widget::column(items)
            .spacing(48)
            .max_width(640.0)
            .width(Fill)
            .into()
    }
}
