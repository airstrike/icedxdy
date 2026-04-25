use std::collections::BTreeSet;
use std::path::PathBuf;

use iced::Alignment;
use iced::widget::{Id, center, column, container, sensor, stack};
use iced::{Fill, Font, Function, Pixels, Task};

use icedxdy::Size::*;
use icedxdy::widget::markdown;
use icedxdy::widget::{button, text};
use icedxdy::{Component, Theme, theme};

use crate::example;
use crate::icon;

/// Height for preview containers (shadcn uses h-72 = 288px)
const PREVIEW_HEIGHT: f32 = 288.0;

/// Collapsed code-snippet height when not expanded by the user (px).
const COLLAPSED_CODE_HEIGHT: f32 = 160.0;

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
    ExpandCode(usize),
    CopyCode(usize),
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
    fn code_expanded(&self) -> &BTreeSet<usize>;

    fn init(&self) -> Task<super::Message>;
    fn update(&mut self, message: Message) -> Task<super::Message>;
    fn preview(&self, index: usize) -> iced::Element<'_, super::Message, Theme>;

    /// Reset state that should NOT persist across navigation away from this
    /// page (e.g., expanded-code dismissal). Default: clear nothing —
    /// pages with `page::State` should override to call `state.reset_volatile()`.
    fn reset_volatile(&mut self) {}
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

    let expanded_set = page.code_expanded();

    let items = meta.examples.iter().enumerate().flat_map(|(i, ex)| {
        let preview = center(page.preview(i))
            .height(PREVIEW_HEIGHT)
            .width(Fill)
            .padding(40)
            .style(theme::container::card_preview);

        let code: Option<iced::Element<'_, super::Message, Theme>> =
            (!ex.snippet.is_empty()).then(|| {
                let body = snippets[i]
                    .as_ref()
                    .map(snippet.with(pi))
                    .unwrap_or_else(|| text("Loading...").into());
                let inner = container(body)
                    .padding([12, 16])
                    .width(Fill)
                    .style(theme::container::code_block);

                let copy_btn: Component<'_, super::Message> = button::ghost(
                    icon::arrow_up_right().size(14),
                )
                .size(Sm)
                .square()
                .on_press(super::Message::Page(pi, Message::CopyCode(i)))
                .into();
                let copy_corner = container(copy_btn)
                    .align_x(Alignment::End)
                    .align_y(Alignment::Start)
                    .padding([8, 8])
                    .width(Fill)
                    .height(Fill);

                if expanded_set.contains(&i) {
                    stack![inner, copy_corner].into()
                } else {
                    let truncated = inner.max_height(COLLAPSED_CODE_HEIGHT).clip(true);
                    let view_code_btn: Component<'_, super::Message> = button("View Code")
                        .size(Sm)
                        .style(theme::button::outline)
                        .on_press(super::Message::Page(pi, Message::ExpandCode(i)))
                        .into();
                    let overlay = container(view_code_btn)
                        .style(theme::container::code_fade_overlay)
                        .height(COLLAPSED_CODE_HEIGHT)
                        .width(Fill)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center);
                    stack![truncated, overlay, copy_corner].into()
                }
            });

        let card = container(column![preview, code])
            .style(theme::container::card)
            .padding(1)
            .clip(true)
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
            .width(Fill)
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
    markdown::view_with(items, settings, &SnippetViewer)
        .map(move |url| super::Message::Page(page_index, Message::LinkClicked(url)))
        .into()
}

/// Custom markdown viewer for code-snippet rendering. Overrides
/// `code_block` to skip iced's default container chrome (which would
/// nest a second `theme::container::code_block` inside our outer
/// page-level one); we provide the bg + rounded corners + padding from
/// the page-level container instead, so the markdown widget just emits
/// the syntax-highlighted lines + horizontal scrollbar.
///
/// Note: token colors come from `iced_highlighter`'s hard-coded
/// `Base16Ocean` theme (see iced markdown source). To match shadcn's
/// `github-light`/`github-dark` requires bypassing `markdown::parse` and
/// using `iced_highlighter::Stream` directly with `InspiredGitHub` —
/// see follow-up plan.
struct SnippetViewer;

impl<'a> markdown::Viewer<'a, markdown::Uri, Theme, iced::Renderer> for SnippetViewer {
    fn on_link_click(url: markdown::Uri) -> markdown::Uri {
        url
    }

    fn code_block(
        &'a self,
        settings: markdown::Settings,
        _language: Option<&'a str>,
        _code: &'a str,
        lines: &'a [markdown::Text],
    ) -> iced::Element<'a, markdown::Uri, Theme, iced::Renderer> {
        use iced::widget::{rich_text, scrollable};

        let theme = self.code_theme();
        let body = column(lines.iter().map(|line| {
            rich_text(line.spans(settings.style, Some(theme)))
                .on_link_click(Self::on_link_click)
                .font(settings.style.code_block_font)
                .size(settings.code_size)
                .into()
        }));

        scrollable(body)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::default()
                    .width(settings.code_size / 2)
                    .scroller_width(settings.code_size / 2),
            ))
            .into()
    }
}

/// Common state that each page owns via composition.
pub struct State {
    pub page_index: usize,
    pub snippets: Vec<Option<Vec<markdown::Item>>>,
    /// Raw markdown source for each snippet — kept around so the copy
    /// button can extract the code without re-reading the file.
    pub snippets_raw: Vec<Option<String>>,
    pub visible: BTreeSet<usize>,
    pub example_ids: Vec<Id>,
    pub scrollable_id: Id,
    pub code_expanded: BTreeSet<usize>,
    meta: &'static Meta,
}

impl State {
    pub fn new(page_index: usize, meta: &'static Meta) -> Self {
        let count = meta.examples.len();
        Self {
            page_index,
            snippets: vec![None; count],
            snippets_raw: vec![None; count],
            visible: BTreeSet::new(),
            example_ids: (0..count).map(|_| Id::unique()).collect(),
            scrollable_id: Id::unique(),
            code_expanded: BTreeSet::new(),
            meta,
        }
    }

    /// Clear state that doesn't survive navigation away from this page.
    /// Currently: dismissed "View Code" overlays.
    pub fn reset_volatile(&mut self) {
        self.code_expanded.clear();
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
                if let Ok(content) = result
                    && let Some(slot) = self.snippets.get_mut(*index)
                {
                    // TODO: chunk B — pass a CodeTheme through the Viewer so colors
                    // are decided at draw time, then swap DefaultLight for a
                    // shadcn-faithful github-light/dark palette.
                    *slot = Some(markdown::parse(content).collect());
                    if let Some(raw_slot) = self.snippets_raw.get_mut(*index) {
                        *raw_slot = Some(content.clone());
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
            Message::ExpandCode(idx) => {
                self.code_expanded.insert(*idx);
                Some(Task::none())
            }
            Message::CopyCode(idx) => {
                let code = self
                    .snippets_raw
                    .get(*idx)
                    .and_then(|s| s.as_ref())
                    .map(|raw| extract_code_from_markdown(raw))
                    .unwrap_or_default();
                let pi = self.page_index;
                Some(
                    iced::clipboard::write(code)
                        .map(move |_| super::Message::Page(pi, Message::Noop)),
                )
            }
            Message::Noop => Some(Task::none()),
            _ => None,
        }
    }
}

/// Extract the contents of the FIRST fenced code block from a markdown
/// source — strips the opening ```lang fence and the closing ```. Falls
/// back to the full input if no fence is present.
fn extract_code_from_markdown(raw: &str) -> String {
    let mut inside = false;
    let mut out: Vec<&str> = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            if inside {
                break;
            }
            inside = true;
            continue;
        }
        if inside {
            out.push(line);
        }
    }
    if out.is_empty() {
        raw.to_string()
    } else {
        out.join("\n")
    }
}
