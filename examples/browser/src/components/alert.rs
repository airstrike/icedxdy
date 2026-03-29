mod demo;
mod default;
mod destructive;

use std::path::PathBuf;

use iced::widget::{column, container, markdown};
use iced::{Fill, Font, Pixels, Task};

use icedxdy::widget::text;
use icedxdy::{Component, theme};

/// Height for preview containers
const PREVIEW_HEIGHT: f32 = 288.0;

#[derive(Debug, Clone)]
pub enum Message {
    SnippetLoaded(Example, Result<String, String>),
    LinkClicked(markdown::Uri),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
    Demo,
    Default,
    Destructive,
}

impl Example {
    const ALL: &[Example] = &[
        Example::Demo,
        Example::Default,
        Example::Destructive,
    ];

    fn snippet_path(&self) -> PathBuf {
        let name = match self {
            Example::Demo => "alert-demo",
            Example::Default => "alert-default",
            Example::Destructive => "alert-destructive",
        };
        PathBuf::from(format!("examples/browser/snippets/{}.md", name))
    }

    fn title(&self) -> &'static str {
        match self {
            Example::Demo => "Alert",
            Example::Default => "Default",
            Example::Destructive => "Destructive",
        }
    }

    fn view<'a>(&self) -> Component<'a, Message> {
        match self {
            Example::Demo => demo::view(),
            Example::Default => default::view(),
            Example::Destructive => destructive::view(),
        }
    }
}

pub struct State {
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

        (Self { snippets }, Task::batch(tasks))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
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
        let mut items: Vec<icedxdy::widget::Component<'_, Message>> = Vec::new();

        for (i, (example, snippet)) in self.snippets.iter().enumerate() {
            let preview = container(example.view())
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
                .unwrap_or_else(|| text("Loading...").into());

            let code = container(code)
                .padding([12, 16])
                .style(theme::container::code_block);

            let card = container(column![preview, code])
                .style(theme::container::card_muted)
                .width(Fill);

            if *example == Example::Demo {
                items.push(card.into());
            } else {
                if i == 1 {
                    let heading: Component<'_, Message> = text::h2("Examples").into();
                    items.push(heading);
                }

                let section = icedxdy::widget::column([
                    text::h3(example.title()).into(),
                    card.into(),
                ]).spacing(16);
                items.push(section.into());
            }
        }

        icedxdy::widget::column(items)
            .spacing(48)
            .max_width(640.0)
            .width(Fill)
            .into()
    }
}
