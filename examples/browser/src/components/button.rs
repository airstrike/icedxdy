mod default;
mod demo;
mod destructive;
mod ghost;
mod icon;
mod link;
mod outline;
mod rounded;
mod secondary;
mod size;
mod spinner;
mod with_icon;

use std::collections::HashSet;
use std::path::PathBuf;

use iced::widget::{column, container, markdown, operation, scrollable, Id};
use iced::{Fill, Font, Pixels, Task};

use icedxdy::widget::operation::position;
use icedxdy::widget::text;
use icedxdy::{theme, Element, Theme};

/// Height for preview containers (shadcn uses h-72 = 288px)
const PREVIEW_HEIGHT: f32 = 288.0;

#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
    SnippetLoaded(Example, Result<String, String>),
    LinkClicked(markdown::Uri),
    ExampleShown(usize),
    ExampleHidden(usize),
    ScrollTo(usize),
    ScrollResult(Option<iced::Rectangle>),
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

    fn id(&self) -> &'static str {
        match self {
            Example::Demo => "demo",
            Example::Size => "size",
            Example::Default => "default",
            Example::Outline => "outline",
            Example::Secondary => "secondary",
            Example::Ghost => "ghost",
            Example::Destructive => "destructive",
            Example::Link => "link",
            Example::Icon => "icon",
            Example::WithIcon => "with-icon",
            Example::Rounded => "rounded",
            Example::Spinner => "spinner",
        }
    }

    fn view<'a>(&self) -> Element<'a, Message> {
        match self {
            Example::Demo => demo::view(),
            Example::Size => size::view(),
            Example::Default => default::view(),
            Example::Outline => outline::view(),
            Example::Secondary => secondary::view(),
            Example::Ghost => ghost::view(),
            Example::Destructive => destructive::view(),
            Example::Link => link::view(),
            Example::Icon => icon::view(),
            Example::WithIcon => with_icon::view(),
            Example::Rounded => rounded::view(),
            Example::Spinner => spinner::view(),
        }
    }
}

pub struct State {
    snippets: Vec<(Example, Option<Vec<markdown::Item>>)>,
    visible: HashSet<usize>,
    column_id: position::Id,
    scrollable_id: Id,
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

        (
            Self {
                snippets,
                visible: HashSet::new(),
                column_id: position::Id::unique(),
                scrollable_id: Id::unique(),
            },
            Task::batch(tasks),
        )
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
            Message::ExampleShown(idx) => {
                self.visible.insert(idx);
                Task::none()
            }
            Message::ExampleHidden(idx) => {
                self.visible.remove(&idx);
                Task::none()
            }
            Message::ScrollTo(idx) => {
                position::find_position(
                    self.column_id.clone(),
                    idx,
                )
                .map(Message::ScrollResult)
            }
            Message::ScrollResult(Some(bounds)) => {
                operation::scroll_to(
                    self.scrollable_id.clone(),
                    scrollable::AbsoluteOffset {
                        x: 0.0,
                        y: bounds.y,
                    },
                )
            }
            Message::ScrollResult(None) => Task::none(),
        }
    }

    pub fn scrollable_id(&self) -> Id {
        self.scrollable_id.clone()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut items: Vec<icedxdy::widget::Element<'_, Message>> = Vec::new();

        for (i, (example, snippet)) in self.snippets.iter().enumerate() {
            let preview = container(example.view())
                .center_x(Fill)
                .center_y(Fill)
                .width(Fill)
                .height(PREVIEW_HEIGHT)
                .padding(40)
                .style(theme::container::card_preview);

            let code: Element<'_, Message> = snippet
                .as_ref()
                .map(|s| {
                    let mut style = markdown::Style::from(&iced::Theme::TokyoNight);
                    style.code_block_font = Font::MONOSPACE;

                    let settings = markdown::Settings {
                        code_size: Pixels(13.0),
                        ..markdown::Settings::with_style(style)
                    };

                    markdown::view(s, settings)
                        .map(Message::LinkClicked)
                        .into()
                })
                .unwrap_or_else(|| text("Loading...").into());

            let code = container(code)
                .padding([12, 16])
                .style(theme::container::code_block);

            let card = container(
                column![preview, code]
            )
            .style(theme::container::card_muted)
            .width(Fill);

            if *example == Example::Demo {
                let sensored: Element<'_, Message> =
                    iced::widget::sensor(card)
                        .on_show(move |_| Message::ExampleShown(i))
                        .on_hide(Message::ExampleHidden(i))
                        .into();
                items.push(sensored.into());
            } else {
                if i == 1 {
                    let heading: Element<'_, Message> =
                        text::h2("Examples").into();
                    items.push(heading.into());
                }

                let mut section = column![text::h3(example.title())].spacing(16);

                if let Some(desc) = example.description() {
                    section = section.push(text::p(desc).style(theme::text::muted));
                }

                let section_card: Element<'_, Message> =
                    section.push(card).spacing(16).into();

                let sensored: Element<'_, Message> =
                    iced::widget::sensor(section_card)
                        .on_show(move |_| Message::ExampleShown(i))
                        .on_hide(Message::ExampleHidden(i))
                        .into();
                items.push(sensored.into());
            }
        }

        icedxdy::widget::column(items)
            .spacing(48)
            .max_width(640.0)
            .width(Fill)
            .id(self.column_id.clone())
            .into()
    }

    pub fn on_this_page(&self) -> Element<'_, Message> {
        use icedxdy::widget::button;
        use theme::button::{Status, Style};

        // App-level style: shadcn TOC uses plain anchors with
        // text-muted-foreground / hover:text-foreground, no backgrounds
        let toc_style = move |theme: &Theme, status: Status| {
            let p = theme.palette();
            let text_color = match status {
                Status::Active => p.muted.foreground,
                _ => p.foreground,
            };
            Style {
                background: None,
                text_color: text_color.into(),
                ..Default::default()
            }
        };

        let nav_items = Example::ALL.iter().enumerate().skip(1).map(|(i, example)| {
            let is_visible = self.visible.contains(&i);
            let label = if is_visible {
                text::small(example.title()).medium()
            } else {
                text::small(example.title())
            };
            button(label)
                .style(toc_style)
                .on_press(Message::ScrollTo(i))
                .into()
        });

        let header = text::muted("On This Page");

        column![header]
            .push(column(nav_items).spacing(2))
            .spacing(16)
            .padding([24, 8])
            .width(200)
            .into()
    }
}
