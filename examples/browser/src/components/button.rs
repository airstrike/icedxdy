use iced::widget::{column, container, markdown, row, text};
use iced::{Element, Fill, Task};
use std::path::PathBuf;

use icedxdy::{button, theme, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
    SnippetLoaded(Variant, Result<String, String>),
    LinkClicked(markdown::Uri),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Default,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

impl Variant {
    const ALL: &[Variant] = &[
        Variant::Default,
        Variant::Secondary,
        Variant::Outline,
        Variant::Ghost,
        Variant::Destructive,
        Variant::Link,
    ];

    fn snippet_path(&self) -> PathBuf {
        let name = match self {
            Variant::Default => "button-default",
            Variant::Secondary => "button-secondary",
            Variant::Outline => "button-outline",
            Variant::Ghost => "button-ghost",
            Variant::Destructive => "button-destructive",
            Variant::Link => "button-link",
        };
        PathBuf::from(format!("examples/browser/snippets/{}.md", name))
    }

    fn label(&self) -> &'static str {
        match self {
            Variant::Default => "Button",
            Variant::Secondary => "Secondary",
            Variant::Outline => "Outline",
            Variant::Ghost => "Ghost",
            Variant::Destructive => "Destructive",
            Variant::Link => "Link",
        }
    }

    fn view<'a>(&self) -> Element<'a, Message, Theme> {
        match self {
            Variant::Default => button::primary(self.label()).on_press(Message::Pressed).into(),
            Variant::Secondary => button::secondary(self.label()).on_press(Message::Pressed).into(),
            Variant::Outline => button::outline(self.label()).on_press(Message::Pressed).into(),
            Variant::Ghost => button::ghost(self.label()).on_press(Message::Pressed).into(),
            Variant::Destructive => button::destructive(self.label()).on_press(Message::Pressed).into(),
            Variant::Link => button::link(self.label()).on_press(Message::Pressed).into(),
        }
    }
}

pub struct State {
    snippets: Vec<(Variant, Option<Vec<markdown::Item>>)>,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let snippets: Vec<_> = Variant::ALL.iter().map(|v| (*v, None)).collect();

        let tasks: Vec<_> = Variant::ALL
            .iter()
            .map(|variant| {
                let variant = *variant;
                let path = variant.snippet_path();
                Task::perform(
                    async move { tokio::fs::read_to_string(path).await.map_err(|e| e.to_string()) },
                    move |result| Message::SnippetLoaded(variant, result),
                )
            })
            .collect();

        (Self { snippets }, Task::batch(tasks))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Pressed => Task::none(),
            Message::SnippetLoaded(variant, result) => {
                if let Some((_, snippet)) = self.snippets.iter_mut().find(|(v, _)| *v == variant) {
                    if let Ok(content) = result {
                        *snippet = Some(markdown::parse(&content).collect());
                    }
                }
                Task::none()
            }
            Message::LinkClicked(_) => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        let mut left_items = Vec::new();
        let mut right_items = Vec::new();

        for (i, (variant, snippet)) in self.snippets.iter().enumerate() {
            let button_view = container(variant.view())
                .center_x(Fill)
                .center_y(Fill)
                .width(Fill)
                .height(150)
                .style(theme::container::card);

            let code_view: Element<'_, Message, Theme> = snippet
                .as_ref()
                .map(|items| {
                    markdown::view(items, &iced::Theme::TokyoNight)
                        .map(Message::LinkClicked)
                        .into()
                })
                .unwrap_or_else(|| text("Loading...").into());

            let item = column![button_view, code_view].spacing(8).into();

            if i % 2 == 0 {
                left_items.push(item);
            } else {
                right_items.push(item);
            }
        }

        row![
            column(left_items).spacing(24).width(Fill),
            column(right_items).spacing(24).width(Fill),
        ]
        .spacing(24)
        .into()
    }
}
