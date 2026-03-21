use iced::widget::{column, container, markdown, row};
use iced::{Element, Fill, Task};
use std::path::PathBuf;

use icedxdy::widget::{button, text, Size};
use icedxdy::{theme, wrap, Theme};

use crate::icon;

/// Height for preview containers
const PREVIEW_HEIGHT: f32 = 150.0;

#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
    SnippetLoaded(Example, Result<String, String>),
    LinkClicked(markdown::Uri),
}

/// Examples matching shadcn docs order exactly
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
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
            // Size: xs, sm, default, lg — each with arrow-up-right icon
            Example::Size => {
                row![
                    button(wrap(
                        row![text("Extra Small"), icon::arrow_up_right().size(12)]
                            .spacing(6)
                            .align_y(iced::Center),
                    ))
                    .size(Size::Xs)
                    .on_press(Message::Pressed),
                    button(wrap(
                        row![text("Small"), icon::arrow_up_right().size(14)]
                            .spacing(6)
                            .align_y(iced::Center),
                    ))
                    .size(Size::Sm)
                    .on_press(Message::Pressed),
                    button(wrap(
                        row![text("Default"), icon::arrow_up_right().size(14)]
                            .spacing(6)
                            .align_y(iced::Center),
                    ))
                    .on_press(Message::Pressed),
                    button(wrap(
                        row![text("Large"), icon::arrow_up_right().size(16)]
                            .spacing(8)
                            .align_y(iced::Center),
                    ))
                    .size(Size::Lg)
                    .on_press(Message::Pressed),
                ]
                .spacing(16)
                .align_y(iced::Center)
                .into()
            }

            Example::Default => button("Button").on_press(Message::Pressed).into(),

            Example::Outline => button::outline("Outline").on_press(Message::Pressed).into(),
            Example::Secondary => {
                button::secondary("Secondary").on_press(Message::Pressed).into()
            }
            Example::Ghost => button::ghost("Ghost").on_press(Message::Pressed).into(),
            Example::Destructive => {
                button::destructive("Destructive").on_press(Message::Pressed).into()
            }
            Example::Link => button::link("Link").on_press(Message::Pressed).into(),

            // Icon-only button
            Example::Icon => button::icon(wrap(icon::arrow_up_right().size(16)))
                .on_press(Message::Pressed)
                .into(),

            // Button with icon + text
            Example::WithIcon => button(wrap(
                row![icon::git_branch().size(16), text("New Branch")]
                    .spacing(8)
                    .align_y(iced::Center),
            ))
            .on_press(Message::Pressed)
            .into(),

            // Rounded icon button
            Example::Rounded => button::icon(wrap(icon::arrow_up().size(16)))
                .on_press(Message::Pressed)
                .into(),

            // Spinner / loading buttons
            Example::Spinner => row![
                button(wrap(
                    row![icon::loader_circle().size(16), text("Generating")]
                        .spacing(8)
                        .align_y(iced::Center),
                )),
                button(wrap(
                    row![text("Downloading"), icon::loader_circle().size(16)]
                        .spacing(8)
                        .align_y(iced::Center),
                )),
            ]
            .spacing(16)
            .into(),
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
        // Demo section - top showcase (matches shadcn hero: outline + icon)
        let demo = container(
            row![
                button::outline("Button").on_press(Message::Pressed),
                button::icon(wrap(icon::chevron_right().size(16)))
                    .on_press(Message::Pressed),
            ]
            .spacing(8),
        )
        .center_x(Fill)
        .center_y(Fill)
        .width(Fill)
        .height(PREVIEW_HEIGHT)
        .style(theme::container::card);

        let demo_code = text("button::outline(\"Button\")");

        let demo_section = column![demo, demo_code].spacing(12);

        // Examples header
        let examples_header = text::h2("Examples");

        // Individual examples
        let examples = column(self.snippets.iter().map(|(example, snippet)| {
            let header = text::h3(example.title());

            let mut section = column![header].spacing(4);

            if let Some(desc) = example.description() {
                section = section.push(text::muted(desc));
            }

            let preview = container(example.view())
                .center_x(Fill)
                .center_y(Fill)
                .width(Fill)
                .height(PREVIEW_HEIGHT)
                .style(theme::container::card);

            let code: Element<'_, Message, Theme> = snippet
                .as_ref()
                .map(|items| {
                    markdown::view(items, &iced::Theme::TokyoNight)
                        .map(Message::LinkClicked)
                        .into()
                })
                .unwrap_or_else(|| text("Loading...").into());

            section = section.push(preview).push(code);

            section.spacing(12).into()
        }))
        .spacing(32);

        column![demo_section, examples_header, examples]
            .spacing(32)
            .width(Fill)
            .into()
    }
}
