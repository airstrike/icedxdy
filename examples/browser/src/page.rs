use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

use tokio::fs::read_to_string;

use iced::border;
use iced::mouse::Interaction;
use iced::widget::{Id, column, sensor, stack};
use iced::{Alignment, Background, Fill, Font, Pixels, Subscription, Task};

use icedxdy::Size::*;
use icedxdy::theme::container::{
    card, card_preview, code_block, code_block_panel, code_fade_overlay,
};
use icedxdy::theme::tweak::Opaque;
use icedxdy::widget::container::Container;
use icedxdy::widget::markdown;
use icedxdy::widget::markdown::widget::Highlight;
use icedxdy::widget::{button, rule, text};
use icedxdy::{Component, Palette, Theme, Weight, radius, theme};

use crate::font;
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
/// Four flavors:
///
/// - [`Block::Md`] is text-shaped content (Installation, Usage, API
///   Reference, Cursor, free-standing prose) loaded asynchronously from a
///   markdown file. The first `# `, `## `, or `### ` heading in the file
///   is extracted as the section title for the right-side TOC and
///   rendered with the depth-aware heading style.
/// - [`Block::Example`] is a live preview rendered by `Page::preview(id)`
///   wrapped in shadcn's card chrome (288px-tall preview area + collapsible
///   code block underneath). Empty `title` skips the heading + TOC entry —
///   used for the page hero.
/// - [`Block::Inline`] is a live preview rendered into the page flow with
///   no card chrome. Used by typography for the joke-tax article.
/// - [`Block::Group`] wraps a section heading + nested children. The
///   heading's depth is derived from the group's nesting level: depth 0
///   uses `text::h2`, depth >= 1 uses `text::h3`. Children inherit
///   `depth + 1` for THEIR own headings, and the right-side TOC indents
///   them by 12px per level with a 0.85x alpha multiplier on each level
///   below the top.
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
        /// Heading rendered above the card; also drives the TOC anchor.
        /// `""` means no heading and no TOC entry — used for the page hero.
        title: &'static str,
        /// Inline description rendered between the heading and the preview card.
        description: Option<&'static str>,
        /// Path to a `.md` file containing the code snippet. Empty string
        /// means no code block under the preview.
        snippet: &'static str,
    },

    /// An interactive `preview(id)` rendered as page-level prose (no card
    /// chrome). For typography demos and similar inline content.
    #[allow(dead_code)]
    Inline { id: &'static str },

    /// A grouping section with an explicit heading and nested children.
    /// Replaces the auto-h2-injection trick the older renderer used to
    /// drop an "Examples" heading before the first non-hero example.
    /// Children render at the parent's indentation; only the heading
    /// level + TOC indent change with depth.
    #[allow(dead_code)]
    Group {
        /// Section heading rendered at the group's depth.
        title: &'static str,
        /// Nested blocks, rendered after the group heading.
        children: &'static [Block],
    },
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
            Block::Example { title, .. } => {
                if title.is_empty() {
                    None
                } else {
                    Some(*title)
                }
            }
            Block::Group { title, .. } => {
                if title.is_empty() {
                    None
                } else {
                    Some(*title)
                }
            }
            Block::Inline { .. } => None,
        }
    }

    /// Path to the markdown source file loaded asynchronously, if any.
    /// For [`Block::Group`], children expose their own paths via the
    /// recursive walk in [`for_each_path`].
    fn path(&self) -> &'static str {
        match self {
            Block::Md(p) => p,
            Block::Example { snippet, .. } => snippet,
            Block::Inline { .. } => "",
            Block::Group { .. } => "",
        }
    }
}

/// Recursively visit every block and invoke `f(path)` for every non-empty
/// markdown source path — used by `State::init` to schedule snippet/markdown
/// loads across nested groups.
fn for_each_path(blocks: &[Block], f: &mut impl FnMut(&'static str)) {
    for b in blocks {
        let p = b.path();
        if !p.is_empty() {
            f(p);
        }
        if let Block::Group { children, .. } = b {
            for_each_path(children, f);
        }
    }
}

/// Flatten the recursive block tree into a depth-tagged visit order.
/// Each yielded entry corresponds to ONE addressable position in the
/// page (sensor + scroll target + TOC entry), keyed by a flat `usize`
/// index in visit order. Groups precede their children — a group's
/// flat index points at its heading, and each nested child gets its
/// own subsequent flat index.
fn flatten(blocks: &'static [Block]) -> Vec<(&'static Block, u8)> {
    fn walk(out: &mut Vec<(&'static Block, u8)>, blocks: &'static [Block], depth: u8) {
        for b in blocks {
            out.push((b, depth));
            if let Block::Group { children, .. } = b {
                walk(out, children, depth + 1);
            }
        }
    }
    let mut out = Vec::new();
    walk(&mut out, blocks, 0);
    out
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
pub trait Page: Send {
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
    let st = page.state();
    let toc_count = flatten(page.blocks())
        .into_iter()
        .filter(|(b, _)| b.toc_title(st).is_some())
        .count();
    let nav = (toc_count > 1).then(|| toc(page));
    (body, nav)
}

/// Heading widget for a TOC-bearing section. Depth 0 → `h2`, depth >= 1
/// → `h3`. Centralized so render-side and any future depth-aware sizing
/// changes update in one place.
fn heading_for<'a>(
    depth: u8,
    title: impl Into<String>,
) -> iced::Element<'a, super::Message, Theme> {
    let title = title.into();
    if depth == 0 {
        text::h2(title).into()
    } else {
        text::h3(title).into()
    }
}

fn render_blocks<'a>(
    page: &'a dyn Page,
    viewer: &'a SnippetViewer,
) -> Component<'a, super::Message> {
    let pi = page.page_index();
    let st = page.state();
    let mut items: Vec<iced::Element<'_, super::Message, Theme>> = Vec::new();
    let mut flat_idx: usize = 0;
    walk_blocks(
        page.blocks(),
        0,
        page,
        viewer,
        st,
        pi,
        &mut flat_idx,
        &mut items,
    );
    column(items).spacing(48).width(Fill).into()
}

#[allow(clippy::too_many_arguments)]
fn walk_blocks<'a>(
    blocks: &'static [Block],
    depth: u8,
    page: &'a dyn Page,
    viewer: &'a SnippetViewer,
    st: &'a State,
    pi: usize,
    flat_idx: &mut usize,
    items: &mut Vec<iced::Element<'a, super::Message, Theme>>,
) {
    for block in blocks {
        let i = *flat_idx;
        *flat_idx += 1;
        let block_id = st.block_ids[i].clone();

        let body: iced::Element<'_, super::Message, Theme> = match block {
            Block::Md(path) => render_md(pi, depth, *path, st, viewer),
            Block::Example {
                id,
                title,
                description,
                snippet,
            } => render_example(
                pi,
                i,
                depth,
                id,
                title,
                *description,
                snippet,
                st,
                viewer,
                page,
            ),
            Block::Inline { id } => page.preview(id),
            Block::Group { title, .. } => heading_for(depth, *title),
        };

        // Wrap each block with a sensor for viewport tracking + a stable Id
        // for `scroll_into_view`. The sensor is the outermost widget so it
        // covers the entire block (heading + description + card or md body).
        // For Block::Group, the sensor wraps ONLY the heading — children
        // get their own subsequent flat indices and their own sensors.
        let sensored = sensor(Container::new(body).id(block_id).width(Fill))
            .on_show(move |_| super::Message::Page(pi, Message::BlockShown(i)))
            .on_hide(super::Message::Page(pi, Message::BlockHidden(i)))
            .anticipate(50);
        items.push(sensored.into());

        if let Block::Group { children, .. } = block {
            walk_blocks(children, depth + 1, page, viewer, st, pi, flat_idx, items);
        }
    }
}

fn render_md<'a>(
    page_index: usize,
    depth: u8,
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
        col = col.push(heading_for(depth, t.to_owned()));
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
    depth: u8,
    id: &'static str,
    title: &'static str,
    description: Option<&'static str>,
    snippet_path: &'static str,
    st: &'a State,
    viewer: &'a SnippetViewer,
    page: &'a dyn Page,
) -> iced::Element<'a, super::Message, Theme> {
    // Hero (empty-title) example uses min_height so taller content (e.g.
    // inline article previews if any) can grow; non-hero examples pin
    // height to PREVIEW_HEIGHT.
    let preview_el: iced::Element<'_, super::Message, Theme> = if title.is_empty() {
        Container::new(page.preview(id))
            .center(Fill)
            .min_height(PREVIEW_HEIGHT)
            .width(Fill)
            .padding(PREVIEW_PADDING)
            .style(card_preview)
            .into()
    } else {
        Container::new(page.preview(id))
            .center(Fill)
            .height(PREVIEW_HEIGHT)
            .width(Fill)
            .padding(PREVIEW_PADDING)
            .style(card_preview)
            .into()
    };

    let code: Option<iced::Element<'_, super::Message, Theme>> =
        (!snippet_path.is_empty()).then(|| {
            let body = st
                .md_items
                .get(snippet_path)
                .and_then(|opt| opt.as_ref())
                .map(|items| snippet(page_index, items, viewer))
                .unwrap_or_else(|| text("Loading...").into());

            let inner = Container::new(body)
                .padding([12, 16])
                .width(Fill)
                .style(code_block_panel);

            if st.code_expanded.contains(&block_pos) {
                let copy_btn: Component<'_, super::Message> = button::ghost(icon::copy().size(14))
                    .size(Sm)
                    .square()
                    .on_press(super::Message::Page(
                        page_index,
                        Message::CopyCode(block_pos),
                    ))
                    .into();

                let copy_corner = Container::new(copy_btn)
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
                    .on_press(super::Message::Page(
                        page_index,
                        Message::ExpandCode(block_pos),
                    ))
                    .into();

                let overlay = Container::new(view_code_btn)
                    .center(Fill)
                    .style(code_fade_overlay)
                    .height(COLLAPSED_CODE_HEIGHT);
                stack![truncated, overlay].into()
            }
        });

    let divider: Option<iced::Element<'_, super::Message, Theme>> =
        code.as_ref().map(|_| rule::horizontal(1).into());

    let card_el: iced::Element<'_, super::Message, Theme> =
        Container::new(column![preview_el, divider, code])
            .style(card)
            .padding(1)
            .clip(true)
            .width(Fill)
            .into();

    // Hero: no h3, no description, just the card.
    if title.is_empty() {
        return card_el;
    }

    let desc: Option<iced::Element<'_, super::Message, Theme>> =
        description.map(|d| text::p(d).style(theme::text::Muted).into());

    let mut col = column![].spacing(16).width(Fill);
    col = col.push(heading_for(depth, title));
    if let Some(d) = desc {
        col = col.push(d);
    }
    col = col.push(card_el);
    col.into()
}

/// Right-side "On This Page" navigation.
///
/// Per `.claude/research/SIDEBARS.md`: NOT a `SidebarMenuButton`. shadcn's
/// docs-toc uses plain `<a>` tags with `text-muted-foreground` (inactive),
/// `hover:text-foreground` (hover), and
/// `data-[active=true]:font-medium data-[active=true]:text-foreground` for
/// the currently-viewed section. No backgrounds at any state — text color
/// only.
///
/// The browser's gloss on top of that spec is a viewport-driven alpha
/// gradient: items already in the visible set fade down toward the edges
/// of the visible set; items not visible at all get a baseline 0.25 alpha
/// so they stay legible-but-recessive. The rank-center item (alpha == 1.0)
/// is the closest analogue to the docs-toc "active" entry, so it gets the
/// `Weight::Medium` bump.
fn toc<'a>(page: &'a dyn Page) -> Component<'a, super::Message> {
    let pi = page.page_index();
    let st = page.state();

    // Walk the block tree in flat-visit order, capturing each TOC-bearing
    // entry's flat index, depth, and title.
    let entries: Vec<(usize, u8, String)> = flatten(page.blocks())
        .into_iter()
        .enumerate()
        .filter_map(|(flat, (b, depth))| b.toc_title(st).map(|t| (flat, depth, t.to_owned())))
        .collect();

    let visible_in_toc: Vec<usize> = st
        .visible
        .iter()
        .copied()
        .filter(|i| entries.iter().any(|(j, _, _)| j == i))
        .collect();

    let toc_count = visible_in_toc.len();
    let alpha_for = |i: usize| -> f32 {
        if toc_count == 0 || !visible_in_toc.contains(&i) {
            return 0.25;
        }
        let Some(rank) = visible_in_toc.iter().position(|&v| v == i) else {
            return 0.25;
        };
        let center = (toc_count - 1) as f32 / 2.0;
        let distance = (rank as f32 - center).abs() / center.max(1.0);
        1.0 - 0.5 * distance
    };

    // The "active" TOC entry mirrors docs-toc: whichever entry currently
    // sits at rank-center of the visible set (alpha == 1.0) is the one
    // the reader is reading. Empty-visible-set pages have no active entry.
    let active_index: Option<usize> = if visible_in_toc.is_empty() {
        None
    } else {
        let center = (toc_count - 1) / 2;
        visible_in_toc.get(center).copied()
    };

    use iced_widget::button::StyleFn;
    use theme::button::{Status, Style};
    let nav_items = entries.into_iter().map(|(i, depth, title)| {
        // Sub-items render slightly desaturated (0.85x) on top of the
        // existing rank-based alpha gradient. Indent grows 12px per level.
        let level_mul = if depth == 0 { 1.0 } else { 0.85 };
        let alpha = alpha_for(i) * level_mul;
        let indent = 8.0 + f32::from(depth) * 12.0;
        let is_active = Some(i) == active_index;
        let toc_style: StyleFn<'_, Theme> = Box::new(move |theme, status| {
            let p = theme.palette();
            // Inactive base = `muted_foreground` per docs-toc spec; the
            // rank-based alpha then fades distant entries further. The
            // active entry and any hovered/pressed entry promote to the
            // brighter `foreground`. No backgrounds at any state.
            let base = match status {
                Status::Active if is_active => p.foreground.alpha(alpha),
                Status::Active => p.muted.foreground.alpha(alpha),
                Status::Hovered | Status::Pressed | Status::Disabled => p.foreground,
            };
            Style {
                background: None,
                text_color: base,
                ..Style::default()
            }
            .into()
        });
        let weight = if is_active {
            Weight::Medium
        } else {
            Weight::Normal
        };
        let entry: Component<'_, super::Message> = button(title)
            .size(Sm)
            .weight(weight)
            .translate(false)
            .style(toc_style)
            .inset([1, 0])
            .interaction(Interaction::Pointer)
            .on_press(super::Message::Page(pi, Message::ScrollTo(i)))
            .into();
        let row: iced::Element<'_, super::Message, Theme> =
            Container::new(entry).padding([0.0, indent]).into();
        row
    });

    let header = Container::new(text::muted("On This Page")).padding([0, 8]);

    column![header]
        .push(column(nav_items))
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
    let mut settings = markdown::Settings {
        code_size: Pixels(13.0),
        // shadcn's `prose` body uses `leading-7` (28px on 16px body) — match
        // for nova fidelity.
        line_height: markdown::LineHeight::Relative(1.75),
        ..markdown::Settings::with_style(viewer.style)
    };
    // `Settings::with_style` auto-derives inline_code_size as text_size *
    // 0.875 (= 14 for body 16). Geist Mono's cap-height reads ~1px taller
    // than Inter Tight at the same nominal size, so 14 still looks bigger
    // than body. Pin inline at 13 to land visually-balanced with the
    // surrounding prose.
    settings.style.inline_code_size = Pixels(13.0);
    markdown::view_with(items, settings, viewer)
        .map(move |url| super::Message::Page(page_index, Message::LinkClicked(url)))
        .into()
}

/// Build a [`markdown::Style`] whose inline-code chip pulls from the
/// active [`Palette`]. Mirrors shadcn's `--color-code` / `--color-code-foreground`
/// pair: light-mode chip is a near-white bg with body fg; dark-mode chip
/// is a dark gray bg with a desaturated fg. Both flavors hand off via the
/// same field and stay in sync with the page's `code_block` container,
/// since [`Palette::code`] is the same `SemanticColor` that drives that
/// container's bg/fg.
fn inline_style_from_palette(palette: &Palette) -> markdown::Style {
    markdown::Style {
        font: Font::default(),
        // shadcn's inline `<code>` uses `bg-muted` — there's no dedicated
        // `--color-code` token in nova; the muted surface is the canonical
        // inline-chip background, with foreground text inheriting body color.
        inline_code_color: palette.foreground.into(),
        inline_code_highlight: Highlight {
            background: Background::Color(palette.muted.base.into()),
            border: border::rounded(4),
        },
        // shadcn squeezes inline code with `px-[0.3rem] py-[0.2rem]`. iced
        // expresses chip padding in pixels of font-size; 1px on the sides
        // stays compact without overlapping adjacent words.
        inline_code_padding: iced::Padding::default().left(2).right(2),
        // Bundled Geist Mono — matches shadcn nova's monospace surface
        // on the live docs site. Loaded at startup in `main.rs`; falls
        // back to cosmic-text's default monospace if the registration
        // fails for any reason.
        inline_code_font: Font::new(font::GEIST_MONO_NAME),
        // Baseline matching body=16; `markdown::Settings::with_text_size`
        // overwrites this to `text_size * 0.875` when the snippet renders,
        // so this only matters if a caller wires up `Settings` manually.
        inline_code_size: Pixels(14.0),
        code_block_font: Font::new(font::GEIST_MONO_NAME),
        // Snippet bodies rarely contain links, but when they do the
        // primary base reads as a "clickable" accent in either theme
        // (dark text on light, light text on dark).
        link_color: palette.primary.base.into(),
    }
}

/// Markdown viewer for code-snippet rendering. Holds an owned
/// [`markdown::highlighter::Map`] derived from the current app theme
/// (or supplied directly by the app) plus a [`markdown::Style`] whose
/// inline-code chip is tinted to the active [`Palette`]. Both fields
/// are baked at construction time and the App rebuilds the viewer
/// whenever the theme toggles, so view-time rendering stays a pure
/// read.
///
/// Overrides `code_block` to skip iced's default container chrome —
/// bg + radius + padding come from the page-level container, the
/// markdown widget just emits the highlighted rich-text lines +
/// horizontal scrollbar.
pub struct SnippetViewer {
    map: markdown::highlighter::Map,
    /// Palette-derived markdown style. Inline-code fields (color +
    /// highlight bg) are driven by `palette.code`; everything else
    /// is the icedxdy default. Private because only [`snippet`] in
    /// this module reads it; the App swaps the whole viewer on theme
    /// toggle rather than mutating in place.
    style: markdown::Style,
}

impl SnippetViewer {
    /// Build a viewer with an explicit code-theme map (typically a
    /// VS Code theme parsed by `Map::from_vscode_theme`) and a
    /// palette-derived [`markdown::Style`]. App-level construction site
    /// recomputes both whenever the user toggles light/dark, mirroring
    /// the existing `Map` rebuild path.
    pub fn new(map: markdown::highlighter::Map, palette: &Palette) -> Self {
        Self {
            map,
            style: inline_style_from_palette(palette),
        }
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

        let scroll = scrollable(body)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::default()
                    .width(settings.code_size / 2)
                    .scroller_width(settings.code_size / 2),
            ))
            .spacing(2);

        Container::new(scroll)
            .padding([12, 16])
            .width(Fill)
            .style(code_block)
            .into()
    }
}

/// Common state that each page owns via composition.
///
/// Sensor / scroll / code-expansion bookkeeping uses a FLAT index space
/// keyed by `flatten(blocks)` visit order. Top-level blocks AND every
/// nested child of a `Block::Group` each get their own flat index, their
/// own widget [`Id`], and their own sensor — clicking a TOC entry for a
/// nested example scrolls to the example, not its parent group, and
/// expanding a child example's code block doesn't ripple to siblings.
pub struct State {
    pub page_index: usize,
    /// Parsed markdown items keyed by path. For `Block::Example`, this is
    /// the code-snippet body. For `Block::Md`, this is the body with the
    /// first heading peeled off (rendered separately as a depth-aware heading).
    pub md_items: HashMap<&'static str, Option<Vec<markdown::Item>>>,
    /// Raw markdown source keyed by path — kept for the copy button.
    pub md_raw: HashMap<&'static str, String>,
    /// Extracted first-heading title for `Block::Md` paths. `None` if not
    /// loaded yet; `Some("")` if the file had no heading.
    pub md_titles: HashMap<&'static str, Option<String>>,
    /// Sensor-driven viewport tracking, indexed by FLAT block position.
    pub visible: BTreeSet<usize>,
    /// Per-flat-block widget ids for `scroll_into_view` and sensors.
    /// `block_ids.len() == flatten(blocks).len()`.
    pub block_ids: Vec<Id>,
    pub scrollable_id: Id,
    /// FLAT block positions whose code block is expanded.
    pub code_expanded: BTreeSet<usize>,
    blocks: &'static [Block],
}

impl State {
    pub fn new(page_index: usize, blocks: &'static [Block]) -> Self {
        let flat_count = flatten(blocks).len();
        Self {
            page_index,
            md_items: HashMap::new(),
            md_raw: HashMap::new(),
            md_titles: HashMap::new(),
            visible: BTreeSet::new(),
            block_ids: (0..flat_count).map(|_| Id::unique()).collect(),
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
        let mut paths: Vec<&'static str> = Vec::new();
        for_each_path(self.blocks, &mut |p| paths.push(p));
        paths.sort();
        paths.dedup();

        let tasks: Vec<Task<super::Message>> = paths
            .into_iter()
            .map(|path| {
                let buf = PathBuf::from(path);
                Task::perform(
                    async move { read_to_string(buf).await.map_err(|e| e.to_string()) },
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
                    let is_md = flatten(self.blocks)
                        .iter()
                        .any(|(b, _)| matches!(b, Block::Md(p) if *p == *path));
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
            Message::ScrollTo(idx) => self
                .block_ids
                .get(*idx)
                .cloned()
                .map(|id| scroll::scroll_into_view(self.scrollable_id.clone(), id)),
            Message::ExpandCode(idx) => {
                self.code_expanded.insert(*idx);
                Some(Task::none())
            }
            Message::CopyCode(idx) => {
                let code = flatten(self.blocks)
                    .get(*idx)
                    .and_then(|(b, _)| match b {
                        Block::Example { snippet, .. } if !snippet.is_empty() => Some(*snippet),
                        Block::Example { .. }
                        | Block::Md(_)
                        | Block::Inline { .. }
                        | Block::Group { .. } => None,
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
