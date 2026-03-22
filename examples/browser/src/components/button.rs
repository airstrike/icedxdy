use iced::widget::{column, container, markdown};
use iced::{Element, Fill, Task};
use std::path::PathBuf;

use icedxdy::widget::{button, row, text};
use icedxdy::Size::*;
use icedxdy::{theme, Theme};

use crate::icon;

/// Height for preview containers (shadcn uses h-72 = 288px)
const PREVIEW_HEIGHT: f32 = 288.0;

#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
    SnippetLoaded(Example, Result<String, String>),
    LinkClicked(markdown::Uri),
}

/// Examples matching shadcn docs order exactly
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
    Demo,
    Size,
    Default,
    Outline,
    Secondary,
    Ghost,
    Destructive,
    Link,
    Icon,
    WithIcon,
    Rounded,
    Spinner,
}

impl Example {
    const ALL: &[Example] = &[
        Example::Demo,
        Example::Size,
        Example::Default,
        Example::Outline,
        Example::Secondary,
        Example::Ghost,
        Example::Destructive,
        Example::Link,
        Example::Icon,
        Example::WithIcon,
        Example::Rounded,
        Example::Spinner,
    ];

    fn snippet_path(&self) -> PathBuf {
        let name = match self {
            Example::Demo => "button-demo",
            Example::Size => "button-size",
            Example::Default => "button-default",
            Example::Outline => "button-outline",
            Example::Secondary => "button-secondary",
            Example::Ghost => "button-ghost",
            Example::Destructive => "button-destructive",
            Example::Link => "button-link",
            Example::Icon => "button-icon",
            Example::WithIcon => "button-with-icon",
            Example::Rounded => "button-rounded",
            Example::Spinner => "button-spinner",
        };
        PathBuf::from(format!("examples/browser/snippets/{}.md", name))
    }

    fn title(&self) -> &'static str {
        match self {
            Example::Demo => "Button",
            Example::Size => "Size",
            Example::Default => "Default",
            Example::Outline => "Outline",
            Example::Secondary => "Secondary",
            Example::Ghost => "Ghost",
            Example::Destructive => "Destructive",
            Example::Link => "Link",
            Example::Icon => "Icon",
            Example::WithIcon => "With Icon",
            Example::Rounded => "Rounded",
            Example::Spinner => "Spinner",
        }
    }

    fn description(&self) -> Option<&'static str> {
        match self {
            Example::Size => Some("Use the size prop to change the size of the button."),
            Example::Rounded => Some("Use rounded styling to make the button circular."),
            Example::Spinner => Some("Show a loading spinner inside the button."),
            _ => None,
        }
    }

    fn view<'a>(&self) -> Element<'a, Message, Theme> {
        match self {
            Example::Demo => {
                row![
                    button::outline("Button").on_press(Message::Pressed),
                    button::icon(icon::arrow_up())
                        .on_press(Message::Pressed),
                ]
                .spacing(8)
                .into()
            }

            Example::Size => {
                row![
                    row![
                        button::outline("Extra Small")
                            .size(Xs)
                            .on_press(Message::Pressed),
                        button::icon(icon::arrow_up_right())
                            .size(Xs)
                            .on_press(Message::Pressed),
                    ]
                    .spacing(8)
,
                    row![
                        button::outline("Small")
                            .size(Sm)
                            .on_press(Message::Pressed),
                        button::icon(icon::arrow_up_right())
                            .size(Sm)
                            .on_press(Message::Pressed),
                    ]
                    .spacing(8)
,
                    row![
                        button::outline("Default")
                            .on_press(Message::Pressed),
                        button::icon(icon::arrow_up_right())
                            .on_press(Message::Pressed),
                    ]
                    .spacing(8)
,
                    row![
                        button::outline("Large")
                            .size(Lg)
                            .on_press(Message::Pressed),
                        button::icon(icon::arrow_up_right())
                            .size(Lg)
                            .on_press(Message::Pressed),
                    ]
                    .spacing(8)
,
                ]
                .spacing(16)
                .into()
            }

            Example::Default => button!["Button"].on_press(Message::Pressed).into(),
            Example::Outline => button::outline("Outline").on_press(Message::Pressed).into(),
            Example::Secondary => {
                button::secondary("Secondary").on_press(Message::Pressed).into()
            }
            Example::Ghost => button::ghost("Ghost").on_press(Message::Pressed).into(),
            Example::Destructive => {
                button::destructive("Destructive").on_press(Message::Pressed).into()
            }
            Example::Link => button::link("Link").on_press(Message::Pressed).into(),

            Example::Icon => button::icon(icon::arrow_up_right())
                .on_press(Message::Pressed)
                .into(),

            Example::WithIcon => {
                button![icon::git_branch(), "New Branch"]
                    .on_press(Message::Pressed)
                    .into()
            }

            Example::Rounded => button::icon(icon::arrow_up())
                .rounded()
                .on_press(Message::Pressed)
                .into(),

            Example::Spinner => {
                use icedxdy::widget::spinner;
                row![
                    button![spinner(), "Generating"]
                        .style(theme::button::outline),
                    button!["Downloading", spinner()]
                        .style(theme::button::secondary),
                ]
                .spacing(16)
                .into()
            }
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
                async move { tokio::fs::read_to_string(path).await.map_err(|e| e.to_string()) },
                move |result| Message::SnippetLoaded(example, result),
            )
        });

        (Self { snippets }, Task::batch(tasks))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Pressed => Task::none(),
            Message::SnippetLoaded(example, result) => {
                if let Some((_, snippet)) = self.snippets.iter_mut().find(|(e, _)| *e == example) {
                    if let Ok(content) = result {
                        *snippet = Some(markdown::parse(&content).collect());
                    }
                }
                Task::none()
            }
            Message::LinkClicked(url) => {
                let _ = url;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        let sections = column(self.snippets.iter().map(|(example, snippet)| {
            let is_demo = *example == Example::Demo;

            let preview = container(example.view())
                .center_x(Fill)
                .center_y(Fill)
                .width(Fill)
                .height(PREVIEW_HEIGHT)
                .padding(40)
                .style(theme::container::card);

            let code: Element<'_, Message, Theme> = snippet
                .as_ref()
                .map(|items| {
                    markdown::view(items, &iced::Theme::TokyoNight)
                        .map(Message::LinkClicked)
                        .into()
                })
                .unwrap_or_else(|| text("Loading...").into());

            if is_demo {
                column![preview, code].spacing(16).into()
            } else {
                let mut section = column![text::h3(example.title())].spacing(16);

                if let Some(desc) = example.description() {
                    section = section.push(text::p(desc).style(theme::text::muted));
                }

                section.push(preview).push(code).spacing(16).into()
            }
        }))
        .spacing(48);

        // "Examples" header between demo and the rest
        let demo = &sections;
        let _ = demo; // sections includes Demo as first item

        // Split: first item is demo, rest are examples
        // Actually, let's just render with the h2 between demo and examples
        let mut items: Vec<Element<'_, Message, Theme>> = Vec::new();

        for (i, (example, snippet)) in self.snippets.iter().enumerate() {
            let preview = container(example.view())
                .center_x(Fill)
                .center_y(Fill)
                .width(Fill)
                .height(PREVIEW_HEIGHT)
                .padding(40)
                .style(theme::container::card);

            let code: Element<'_, Message, Theme> = snippet
                .as_ref()
                .map(|s| {
                    markdown::view(s, &iced::Theme::TokyoNight)
                        .map(Message::LinkClicked)
                        .into()
                })
                .unwrap_or_else(|| text("Loading...").into());

            if *example == Example::Demo {
                items.push(column![preview, code].spacing(16).into());
            } else {
                if i == 1 {
                    // Insert "Examples" header before first non-demo example
                    items.push(text::h2("Examples").into());
                }

                let mut section = column![text::h3(example.title())].spacing(16);

                if let Some(desc) = example.description() {
                    section = section.push(text::p(desc).style(theme::text::muted));
                }

                items.push(section.push(preview).push(code).spacing(16).into());
            }
        }

        column(items)
            .spacing(48)
            .max_width(640)
            .into()
    }
}
