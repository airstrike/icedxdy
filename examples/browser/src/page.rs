use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

use tokio::fs::read_to_string;

use iced::Alignment;
use iced::widget::{Id, center, column, container, sensor, stack};
use iced::{Fill, Font, Pixels, Subscription, Task};

use icedxdy::Size::*;
use icedxdy::theme::container::{card, card_preview, code_block, code_fade_overlay};
use icedxdy::theme::tweak::Opaque;
use icedxdy::widget::markdown;
use icedxdy::widget::container::Container;
use icedxdy::widget::{button, rule, text};
use icedxdy::{Component, Theme, radius, theme};

use crate::icon;

/// Height for preview containers (shadcn uses h-72 = 288px). Per-example
/// cards stay at this fixed height; tall content (the typography
/// article) lives in a `Block::Inline` instead, which has no card chrome
/// and grows naturally.
const PREVIEW_HEIGHT: f32 = 288.0;

/// Card preview padding — shadcn uses `p-6` = 24px.
const PREVIEW_PADDING: f32 = 24.0;

/// Collapsed code-snippet height when not expanded by the user (px).
const COLLAPSED_CODE_HEIGHT: f32 = 160.0;

/// Static metadata for a component page.
pub struct Meta {
    pub title: &'static str,
    pub description: &'static str,
}

/// Composition primitive for a component page. A page is a `&'static [Block]`
/// rendered in order between the page title/description and the bottom of
/// the scroll area.
///
/// Three flavors:
///
/// - [`Block::Md`] is text-shaped content (Installation, Usage, API
///   Reference, Cursor, free-standing prose) loaded asynchronously from a
///   markdown file. The first `# `, `## `, or `### ` heading in the file
///   is extracted as the section title for the right-side TOC and
///   rendered via the same `text::h3` helper as `Block::Example`'s title
///   — visual parity is the point.
/// - [`Block::Example`] is a live preview rendered by `Page::preview(id)`
///   wrapped in shadcn's card chrome (288px-tall preview area + collapsible
///   code block underneath). Empty `title` skips the h3 + TOC entry — used
///   for the page hero.
/// - [`Block::Inline`] is a live preview rendered into the page flow with
///   no card chrome. Used by typography for the joke-tax article.
pub enum Block {
    /// A pre-rendered markdown section. Path is relative to the repo root,
    /// loaded asynchronously via `page::State::init`.
    #[allow(dead_code)]
    Md(&'static str),

    /// A live interactive example with a `preview()` view, optional
    /// description, and a code snippet.
    Example {
        /// Stable id; `Page::preview(id)` dispatches by this string.
        id: &'static str,
        /// h3 heading rendered above the card; also drives the TOC anchor.
        /// `""` means no h3 and no TOC entry — used for the page hero.
        title: &'static str,
        /// Inline description rendered between the h3 and the preview card.
        description: Option<&'static str>,
        /// Path to a `.md` file containing the code snippet. Empty string
        /// means no code block under the preview.
        snippet: &'static str,
    },

    /// An interactive `preview(id)` rendered as page-level prose (no card
    /// chrome). For typography demos and similar inline content.
    #[allow(dead_code)]
    Inline { id: &'static str },
}

impl Block {
    /// Title used for the right-side TOC. `None` skips the entry.
    fn toc_title<'a>(&'a self, state: &'a State) -> Option<&'a str> {
        match self {
            Block::Md(p) => state
                .md_titles
                .get(*p)
                .and_then(|t| t.as_deref())
                .filter(|s| !s.is_empty()),
            Block::Example { title, .. } if !title.is_empty() => Some(*title),
            _ => None,
        }
    }

    /// Path to the markdown source file loaded asynchronously, if any.
    fn path(&self) -> &'static str {
        match self {
            Block::Md(p) => p,
            Block::Example { snippet, .. } => snippet,
            Block::Inline { .. } => "",
        }
    }
}

/// Messages produced by page infrastructure and component interactions.
#[derive(Debug, Clone)]
pub enum Message {
    /// Snippet/markdown body finished loading. Keyed by path so blocks
    /// can share content if needed, and so reordering blocks doesn't
    /// break dispatch.
    SnippetLoaded(&'static str, Result<String, String>),
    LinkClicked(markdown::Uri),
    BlockShown(usize),
    BlockHidden(usize),
    ScrollTo(usize),
    ExpandCode(usize),
    CopyCode(usize),
    InputChanged(usize, String),
    /// Float-valued counterpart of `InputChanged`. For sliders and other
    /// continuous-value pages.
    ValueChanged(usize, f32),
    Noop,
}

/// Object-safe trait for a component page.
pub trait Page {
    fn meta(&self) -> &'static Meta;
    fn blocks(&self) -> &'static [Block];
    fn page_index(&self) -> usize;
    fn state(&self) -> &State;
    fn scrollable_id(&self) -> Id;

    fn init(&self) -> Task<super::Message>;
    fn update(&mut self, message: Message) -> Task<super::Message>;
    fn preview(&self, id: &str) -> iced::Element<'_, super::Message, Theme>;

    /// Subscription for pages that drive animations or timers. Default:
    /// `Subscription::none()`. Only the currently active page's subscription
    /// is wired by `App`, so pages don't waste cycles when the user is on
    /// a different tab.
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    /// Reset state that should NOT persist across navigation away from this
    /// page (e.g., expanded-code dismissal). Default: clear nothing —
    /// pages with `page::State` should override to call `state.reset_volatile()`.
    fn reset_volatile(&mut self) {}

    /// Called by `App` whenever the user navigates TO this page. Use it for
    /// "first-paint" triggers that iced's `sensor` can't reliably catch when
    /// the example is already in the viewport at mount time. Default: no-op.
    fn on_visible(&mut self) -> Task<super::Message> {
        Task::none()
    }
}

/// Render a page's body and optional "On This Page" nav. The `viewer`
/// owns the active code-theme `Map`; the caller (App) keeps it in
/// state and rebuilds it on theme toggle so the viewer reference here
/// outlives the returned Element.
pub fn view<'a>(
    page: &'a dyn Page,
    viewer: &'a SnippetViewer,
) -> (
    Component<'a, super::Message>,
    Option<Component<'a, super::Message>>,
) {
    let body = render_blocks(page, viewer);
    let toc_count = page
        .blocks()
        .iter()
        .filter(|b| b.toc_title(page.state()).is_some())
        .count();
    let nav = (toc_count > 1).then(|| sidebar(page));
    (body, nav)
}

fn render_blocks<'a>(
    page: &'a dyn Page,
    viewer: &'a SnippetViewer,
) -> Component<'a, super::Message> {
    let pi = page.page_index();
    let st = page.state();
    let blocks = page.blocks();

    // Track whether we've emitted the "Examples" h2 yet — appears once,
    // immediately before the first non-hero `Block::Example`.
    let mut emitted_examples_h2 = false;

    let mut items: Vec<iced::Element<'_, super::Message, Theme>> = Vec::new();

    for (i, block) in blocks.iter().enumerate() {
        let block_id = st.block_ids[i].clone();

        let body: iced::Element<'_, super::Message, Theme> = match block {
            Block::Md(path) => {
                if !emitted_examples_h2
                    && st
                        .md_titles
                        .get(*path)
                        .and_then(|t| t.as_deref())
                        .map(|t| t.eq_ignore_ascii_case("examples"))
                        .unwrap_or(false)
                {
                    emitted_examples_h2 = true;
                }
                render_md(pi, *path, st, viewer)
            }
            Block::Example {
                id,
                title,
                description,
                snippet,
            } => {
                if !title.is_empty() && !emitted_examples_h2 {
                    items.push(text::h2("Examples").into());
                    emitted_examples_h2 = true;
                }
                render_example(
                    pi, i, id, title, *description, snippet, st, viewer, page,
                )
            }
            Block::Inline { id } => page.preview(id),
        };

        // Wrap each block with a sensor for viewport tracking + a stable Id
        // for `scroll_into_view`. The sensor is the outermost widget so
        // it covers the entire block (h3 + description + card or md body).
        let sensored = sensor(container(body).id(block_id))
            .on_show(move |_| super::Message::Page(pi, Message::BlockShown(i)))
            .on_hide(super::Message::Page(pi, Message::BlockHidden(i)))
            .anticipate(50);
        items.push(sensored.into());
    }

    column(items).spacing(48).width(Fill).into()
}

fn render_md<'a>(
    page_index: usize,
    path: &'static str,
    st: &'a State,
    viewer: &'a SnippetViewer,
) -> iced::Element<'a, super::Message, Theme> {
    let title = st
        .md_titles
        .get(path)
        .and_then(|t| t.as_deref())
        .filter(|s| !s.is_empty());

    let mut col = column![].spacing(16).width(Fill);
    if let Some(t) = title {
        col = col.push(text::h3(t.to_owned()));
    }
    match st.md_items.get(path) {
        Some(Some(items)) if !items.is_empty() => {
            col = col.push(snippet(page_index, items, viewer).into_iced());
        }
        Some(Some(_)) => {
            // Body is empty (heading-only file) — nothing to render below.
        }
        _ => {
            let loading: iced::Element<'_, super::Message, Theme> = text("Loading...").into();
            col = col.push(loading);
        }
    };
    col.into()
}

#[allow(clippy::too_many_arguments)]
fn render_example<'a>(
    page_index: usize,
    block_pos: usize,
    id: &'static str,
    title: &'static str,
    description: Option<&'static str>,
    snippet_path: &'static str,
    st: &'a State,
    viewer: &'a SnippetViewer,
    page: &'a dyn Page,
) -> iced::Element<'a, super::Message, Theme> {
    // Hero (empty-title) example uses the vendored Container with min_height
    // so taller content (e.g. inline article previews if any) can grow.
    let preview_el: iced::Element<'_, super::Message, Theme> = if title.is_empty() {
        let preview_style: iced_widget::container::StyleFn<'static, Theme> =
            Box::new(|t| card_preview(t).into());
        Container::new(page.preview(id))
            .center(Fill)
            .min_height(PREVIEW_HEIGHT)
            .width(Fill)
            .padding(PREVIEW_PADDING)
            .style(preview_style)
            .into()
    } else {
        center(page.preview(id))
            .height(PREVIEW_HEIGHT)
            .width(Fill)
            .padding(PREVIEW_PADDING)
            .style(|t| card_preview(t).into())
            .into()
    };

    let code: Option<iced::Element<'_, super::Message, Theme>> = (!snippet_path.is_empty())
        .then(|| {
            let body = st
                .md_items
                .get(snippet_path)
                .and_then(|opt| opt.as_ref())
                .map(|items| snippet(page_index, items, viewer))
                .unwrap_or_else(|| text("Loading...").into());

            let inner = container(body)
                .padding([12, 16])
                .width(Fill)
                .style(|t| code_block(t).into());

            if st.code_expanded.contains(&block_pos) {
                let copy_btn: Component<'_, super::Message> =
                    button::ghost(icon::copy().size(14))
                        .size(Sm)
                        .square()
                        .on_press(super::Message::Page(page_index, Message::CopyCode(block_pos)))
                        .into();

                let copy_corner = container(copy_btn)
                    .align_x(Alignment::End)
                    .align_y(Alignment::Start)
                    .padding([8, 8])
                    .width(Fill)
                    .height(Fill);

                stack![inner, copy_corner].into()
            } else {
                let truncated = inner.max_height(COLLAPSED_CODE_HEIGHT).clip(true);

                let view_code_btn: Component<'_, super::Message> = button("View Code")
                    .style(theme::button::Outline + Opaque + radius(Lg))
                    .on_press(super::Message::Page(page_index, Message::ExpandCode(block_pos)))
                    .into();

                let overlay = center(view_code_btn)
                    .style(|t| code_fade_overlay(t).into())
                    .height(COLLAPSED_CODE_HEIGHT);
                stack![truncated, overlay].into()
            }
        });

    let divider: Option<iced::Element<'_, super::Message, Theme>> =
        code.as_ref().map(|_| rule::horizontal(1).into());

    let card: iced::Element<'_, super::Message, Theme> =
        container(column![preview_el, divider, code])
            .style(|t| card(t).into())
            .padding(1)
            .clip(true)
            .width(Fill)
            .into();

    // Hero: no h3, no description, just the card.
    if title.is_empty() {
        return card;
    }

    let desc: Option<iced::Element<'_, super::Message, Theme>> = description
        .map(|d| text::p(d).style(theme::text::Muted).into());

    let mut col = column![].spacing(16).width(Fill);
    col = col.push(text::h3(title));
    if let Some(d) = desc {
        col = col.push(d);
    }
    col = col.push(card);
    col.into()
}

fn sidebar<'a>(page: &'a dyn Page) -> Component<'a, super::Message> {
    let pi = page.page_index();
    let st = page.state();
    let blocks = page.blocks();

    let entries: Vec<(usize, String)> = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.toc_title(st).map(|t| (i, t.to_owned())))
        .collect();

    let visible_in_toc: Vec<usize> = st
        .visible
        .iter()
        .copied()
        .filter(|i| entries.iter().any(|(j, _)| j == i))
        .collect();

    let toc_count = visible_in_toc.len();
    let alpha_for = |i: usize| -> f32 {
        if toc_count == 0 || !visible_in_toc.contains(&i) {
            return 0.25;
        }
        let rank = visible_in_toc.iter().position(|&v| v == i).unwrap();
        let center = (toc_count - 1) as f32 / 2.0;
        let distance = (rank as f32 - center).abs() / center.max(1.0);
        1.0 - 0.5 * distance
    };

    use iced_widget::button::StyleFn;
    use theme::button::{Status, Style};
    let nav_items = entries.into_iter().map(|(i, title)| {
        let alpha = alpha_for(i);
        let toc_style: StyleFn<'_, Theme> = Box::new(move |theme, status| {
            let p = theme.palette();
            let base_color = match status {
                Status::Active => p.foreground.alpha(alpha),
                _ => p.foreground,
            };
            Style {
                background: None,
                text_color: base_color,
                ..Style::default()
            }
            .into()
        });
        button(title)
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

fn snippet<'a>(
    page_index: usize,
    items: &'a Vec<markdown::Item>,
    viewer: &'a SnippetViewer,
) -> Component<'a, super::Message> {
    let mut style = markdown::Style::from(&iced::Theme::TokyoNight);
    style.code_block_font = Font::MONOSPACE;
    let settings = markdown::Settings {
        code_size: Pixels(13.0),
        ..markdown::Settings::with_style(style)
    };
    markdown::view_with(items, settings, viewer)
        .map(move |url| super::Message::Page(page_index, Message::LinkClicked(url)))
        .into()
}

/// Markdown viewer for code-snippet rendering. Holds an owned
/// [`markdown::highlighter::Map`] derived from the current app theme
/// (or supplied directly by the app) — the active palette gets injected
/// at view time without re-parsing the markdown items.
///
/// Overrides `code_block` to skip iced's default container chrome —
/// bg + radius + padding come from the page-level container, the
/// markdown widget just emits the highlighted rich-text lines +
/// horizontal scrollbar.
pub struct SnippetViewer {
    map: markdown::highlighter::Map,
}

impl SnippetViewer {
    /// Build a viewer with an explicit code-theme map — typically a
    /// VS Code theme parsed by `Map::from_vscode_theme`.
    pub fn with_map(map: markdown::highlighter::Map) -> Self {
        Self { map }
    }
}

impl<'a> markdown::Viewer<'a, markdown::Uri, Theme, iced::Renderer> for SnippetViewer {
    fn on_link_click(url: markdown::Uri) -> markdown::Uri {
        url
    }

    fn code_theme(&'a self) -> Option<&'a dyn markdown::highlighter::Theme> {
        Some(&self.map)
    }

    fn code_block(
        &'a self,
        settings: markdown::Settings,
        _language: Option<&'a str>,
        _code: &'a str,
        lines: &'a [markdown::Text],
    ) -> iced::Element<'a, markdown::Uri, Theme, iced::Renderer> {
        use iced::widget::{rich_text, scrollable};

        let body = column(lines.iter().map(|line| {
            rich_text(line.spans(settings.style, Some(&self.map)))
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
            .spacing(2)
            .into()
    }
}

/// Common state that each page owns via composition.
pub struct State {
    pub page_index: usize,
    /// Parsed markdown items keyed by path. For `Block::Example`, this is
    /// the code-snippet body. For `Block::Md`, this is the body with the
    /// first heading peeled off (rendered separately as h3).
    pub md_items: HashMap<&'static str, Option<Vec<markdown::Item>>>,
    /// Raw markdown source keyed by path — kept for the copy button.
    pub md_raw: HashMap<&'static str, String>,
    /// Extracted first-heading title for `Block::Md` paths. `None` if not
    /// loaded yet; `Some("")` if the file had no heading.
    pub md_titles: HashMap<&'static str, Option<String>>,
    /// Sensor-driven viewport tracking, indexed by block position.
    pub visible: BTreeSet<usize>,
    /// Per-block widget ids for `scroll_into_view` and sensors.
    pub block_ids: Vec<Id>,
    pub scrollable_id: Id,
    /// Block positions whose code block is expanded.
    pub code_expanded: BTreeSet<usize>,
    blocks: &'static [Block],
}

impl State {
    pub fn new(page_index: usize, blocks: &'static [Block]) -> Self {
        Self {
            page_index,
            md_items: HashMap::new(),
            md_raw: HashMap::new(),
            md_titles: HashMap::new(),
            visible: BTreeSet::new(),
            block_ids: (0..blocks.len()).map(|_| Id::unique()).collect(),
            scrollable_id: Id::unique(),
            code_expanded: BTreeSet::new(),
            blocks,
        }
    }

    /// Clear state that doesn't survive navigation away from this page.
    /// Currently: dismissed "View Code" overlays.
    pub fn reset_volatile(&mut self) {
        self.code_expanded.clear();
    }

    pub fn init(&self) -> Task<super::Message> {
        let pi = self.page_index;
        let mut paths: Vec<&'static str> = self
            .blocks
            .iter()
            .filter_map(|b| {
                let p = b.path();
                if p.is_empty() { None } else { Some(p) }
            })
            .collect();
        paths.sort();
        paths.dedup();

        let tasks: Vec<Task<super::Message>> = paths
            .into_iter()
            .map(|path| {
                let buf = PathBuf::from(path);
                Task::perform(
                    async move {
                        read_to_string(buf)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    move |result| super::Message::Page(pi, Message::SnippetLoaded(path, result)),
                )
            })
            .collect();
        Task::batch(tasks)
    }

    pub fn update(&mut self, event: &Message) -> Option<Task<super::Message>> {
        use icedxdy::widget::operation::scroll;

        match event {
            Message::SnippetLoaded(path, result) => {
                if let Ok(content) = result {
                    let is_md = self
                        .blocks
                        .iter()
                        .any(|b| matches!(b, Block::Md(p) if *p == *path));
                    let (title, body) = if is_md {
                        peel_first_heading(content)
                    } else {
                        (None, content.clone())
                    };
                    self.md_titles.insert(*path, title);
                    self.md_raw.insert(*path, content.clone());
                    self.md_items
                        .insert(*path, Some(markdown::parse(&body).collect()));
                }
                Some(Task::none())
            }
            Message::LinkClicked(_) => Some(Task::none()),
            Message::BlockShown(idx) => {
                self.visible.insert(*idx);
                Some(Task::none())
            }
            Message::BlockHidden(idx) => {
                self.visible.remove(idx);
                Some(Task::none())
            }
            Message::ScrollTo(idx) => Some(scroll::scroll_into_view(
                self.scrollable_id.clone(),
                self.block_ids[*idx].clone(),
            )),
            Message::ExpandCode(idx) => {
                self.code_expanded.insert(*idx);
                Some(Task::none())
            }
            Message::CopyCode(idx) => {
                let code = self
                    .blocks
                    .get(*idx)
                    .and_then(|b| match b {
                        Block::Example { snippet, .. } if !snippet.is_empty() => Some(*snippet),
                        _ => None,
                    })
                    .and_then(|p| self.md_raw.get(p))
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

/// Strip the FIRST ATX heading (`# foo`, `## foo`, or `### foo`) from a
/// markdown source and return `(heading_text, remaining_body)`. The
/// heading line is consumed; subsequent lines pass through verbatim.
/// Used to extract a section title for the right-side TOC and render it
/// via `text::h3` for visual parity with `Block::Example.title`.
///
/// Skips leading blank lines before the heading. Returns `(None, original)`
/// if the first non-blank line isn't a heading.
fn peel_first_heading(src: &str) -> (Option<String>, String) {
    let lines: Vec<&str> = src.lines().collect();
    let mut found: Option<String> = None;
    let mut rest_index: usize = lines.len();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed
            .strip_prefix("### ")
            .or_else(|| trimmed.strip_prefix("## "))
            .or_else(|| trimmed.strip_prefix("# "))
        {
            found = Some(rest.trim().to_string());
            rest_index = i + 1;
        }
        break;
    }

    let body = if found.is_some() {
        lines[rest_index..].join("\n")
    } else {
        src.to_string()
    };
    (found, body)
}
