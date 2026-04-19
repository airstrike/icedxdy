use std::collections::BTreeSet;
use std::path::PathBuf;

use iced::widget::{Id, center, column, container, markdown, sensor};
use iced::{Fill, Font, Function, Pixels, Task};

use icedxdy::Size::*;
use icedxdy::widget::{button, text};
use icedxdy::{Component, Theme, theme};

use crate::example;

/// Height for preview containers (shadcn uses h-72 = 288px)
const PREVIEW_HEIGHT: f32 = 288.0;

/// Static metadata for a component page.
pub struct Meta {
    pub title: &'static str,
    pub description: &'static str,
    pub examples: &'static [example::Meta],
}

/// Messages produced by page infrastructure and component interactions.
#[derive(Debug, Clone)]
pub enum Message {
    SnippetLoaded(usize, Result<String, String>),
    LinkClicked(markdown::Uri),
    ExampleShown(usize),
    ExampleHidden(usize),
    ScrollTo(usize),
    InputChanged(usize, String),
    Noop,
}

/// Object-safe trait for a component page.
pub trait Page {
    fn meta(&self) -> &'static Meta;
    fn page_index(&self) -> usize;
    fn snippets(&self) -> &[Option<Vec<markdown::Item>>];
    fn visible(&self) -> &BTreeSet<usize>;
    fn example_ids(&self) -> &[Id];
    fn scrollable_id(&self) -> Id;

    fn init(&self) -> Task<super::Message>;
    fn update(&mut self, message: Message) -> Task<super::Message>;
    fn preview(&self, index: usize) -> iced::Element<'_, super::Message, Theme>;
}

/// Render a page's body and optional "On This Page" nav.
pub fn view(
    page: &dyn Page,
) -> (
    Component<'_, super::Message>,
    Option<Component<'_, super::Message>>,
) {
    let body = examples(page);
    let nav = (page.meta().examples.len() > 1).then(|| sidebar(page));
    (body, nav)
}

fn examples<'a>(page: &'a dyn Page) -> Component<'a, super::Message> {
    let pi = page.page_index();
    let meta = page.meta();
    let snippets = page.snippets();
    let example_ids = page.example_ids();

    let items = meta.examples.iter().enumerate().flat_map(|(i, ex)| {
        let preview = center(page.preview(i))
            .height(PREVIEW_HEIGHT)
            .padding(40)
            .style(theme::container::card_preview);

        let code = (!ex.snippet.is_empty()).then_some(
            container(
                snippets[i]
                    .as_ref()
                    .map(snippet.with(pi))
                    .unwrap_or_else(|| text("Loading...").into()),
            )
            .padding([12, 16])
            .style(theme::container::code_block),
        );

        let card = container(column![preview, code])
            .style(theme::container::card_muted)
            .width(Fill)
            .into();

        macro_rules! sensored {
            ($content:expr) => {
                sensor(container($content).id(example_ids[i].clone()))
                    .on_show(move |_| super::Message::Page(pi, Message::ExampleShown(i)))
                    .on_hide(super::Message::Page(pi, Message::ExampleHidden(i)))
                    .anticipate(50)
            };
        }

        if i == 0 {
            vec![sensored!(card).into()]
        } else {
            let desc: Option<Component<'_, super::Message>> = ex
                .description
                .map(|d| text::p(d).style(theme::text::muted).into());

            let section: Component<'_, super::Message> = icedxdy::widget::column(
                [Some(text::h3(ex.title).into()), desc, Some(card)]
                    .into_iter()
                    .flatten(),
            )
            .spacing(16)
            .into();

            let mut items = Vec::with_capacity(2);
            if i == 1 {
                items.push(text::h2("Examples").into());
            }
            items.push(sensored!(section).into());
            items
        }
    });

    icedxdy::widget::column(items)
        .spacing(48)
        .max_width(640.0)
        .width(Fill)
        .into()
}

fn sidebar<'a>(page: &'a dyn Page) -> Component<'a, super::Message> {
    let pi = page.page_index();
    let meta = page.meta();
    let visible = page.visible();

    let count = visible.len();
    let alpha_for = |i: usize| -> f32 {
        if count == 0 || !visible.contains(&i) {
            return 0.25;
        }
        let rank = visible.iter().position(|&v| v == i).unwrap();
        let center = (count - 1) as f32 / 2.0;
        let distance = (rank as f32 - center).abs() / center.max(1.0);
        1.0 - 0.5 * distance
    };

    use theme::button::{Status, Style};
    let nav_items = meta
        .examples
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, example)| {
            let alpha = alpha_for(i);
            let toc_style = move |theme: &Theme, status: Status| {
                let p = theme.palette();
                let base_color = match status {
                    Status::Active => p.foreground.alpha(alpha),
                    _ => p.foreground,
                };
                Style {
                    background: None,
                    text_color: base_color.into(),
                    ..Style::default()
                }
            };
            button(example.title)
                .size(Sm)
                .translate(false)
                .style(toc_style)
                .on_press(super::Message::Page(pi, Message::ScrollTo(i)))
                .into()
        });

    let header = container(text::muted("On This Page")).padding([0, 8]);

    column![header]
        .push(column(nav_items).spacing(2))
        .spacing(16)
        .padding([24, 8])
        .width(200)
        .into()
}

fn snippet<'a>(page_index: usize, items: &'a Vec<markdown::Item>) -> Component<'a, super::Message> {
    let mut style = markdown::Style::from(&iced::Theme::TokyoNight);
    style.code_block_font = Font::MONOSPACE;
    let settings = markdown::Settings {
        code_size: Pixels(13.0),
        ..markdown::Settings::with_style(style)
    };
    markdown::view(items, settings)
        .map(move |url| super::Message::Page(page_index, Message::LinkClicked(url)))
        .into()
}

/// Common state that each page owns via composition.
pub struct State {
    pub page_index: usize,
    pub snippets: Vec<Option<Vec<markdown::Item>>>,
    pub visible: BTreeSet<usize>,
    pub example_ids: Vec<Id>,
    pub scrollable_id: Id,
    meta: &'static Meta,
}

impl State {
    pub fn new(page_index: usize, meta: &'static Meta) -> Self {
        let count = meta.examples.len();
        Self {
            page_index,
            snippets: vec![None; count],
            visible: BTreeSet::new(),
            example_ids: (0..count).map(|_| Id::unique()).collect(),
            scrollable_id: Id::unique(),
            meta,
        }
    }

    pub fn init(&self) -> Task<super::Message> {
        let pi = self.page_index;
        let tasks: Vec<Task<super::Message>> = self
            .meta
            .examples
            .iter()
            .enumerate()
            .filter(|(_, ex)| !ex.snippet.is_empty())
            .map(|(i, ex)| {
                let path = PathBuf::from(ex.snippet);
                Task::perform(
                    async move {
                        tokio::fs::read_to_string(path)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    move |result| super::Message::Page(pi, Message::SnippetLoaded(i, result)),
                )
            })
            .collect();
        Task::batch(tasks)
    }

    pub fn update(&mut self, event: &Message) -> Option<Task<super::Message>> {
        use icedxdy::widget::operation::scroll;

        match event {
            Message::SnippetLoaded(index, result) => {
                if let Some(slot) = self.snippets.get_mut(*index) {
                    if let Ok(content) = result {
                        *slot = Some(markdown::parse(content).collect());
                    }
                }
                Some(Task::none())
            }
            Message::LinkClicked(_) => Some(Task::none()),
            Message::ExampleShown(idx) => {
                self.visible.insert(*idx);
                Some(Task::none())
            }
            Message::ExampleHidden(idx) => {
                self.visible.remove(idx);
                Some(Task::none())
            }
            Message::ScrollTo(idx) => Some(scroll::scroll_into_view(
                self.scrollable_id.clone(),
                self.example_ids[*idx].clone(),
            )),
            Message::Noop => Some(Task::none()),
            _ => None,
        }
    }
}
