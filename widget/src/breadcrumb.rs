//! Breadcrumb navigation — list-of-items API with auto separator interspersing.
//!
//! Mirrors shadcn's `<Breadcrumb><BreadcrumbList>...</BreadcrumbList></Breadcrumb>`
//! composition. The shadcn API exposes seven primitives (`Breadcrumb`,
//! `BreadcrumbList`, `BreadcrumbItem`, `BreadcrumbLink`, `BreadcrumbPage`,
//! `BreadcrumbSeparator`, `BreadcrumbEllipsis`); we collapse the JSX nesting
//! into a flat list of [`Item`] values plus an auto-interspersed separator.
//!
//! ```ignore
//! use icedxdy::widget::{breadcrumb, text};
//!
//! breadcrumb([
//!     breadcrumb::link(text::muted("Home"), Message::GoHome),
//!     breadcrumb::link(text::muted("Components"), Message::GoComponents),
//!     breadcrumb::page("Breadcrumb"),
//! ])
//! ```
//!
//! Item kinds:
//!
//! - [`link`] — clickable. Cursor changes to `Pointer`; emits the supplied
//!   message on left-button release. Mirrors `<BreadcrumbLink>`.
//! - [`page`] — current page. Renders content as-is, no click affordance.
//!   Mirrors `<BreadcrumbPage>`.
//! - [`ellipsis`] — collapsed-items indicator (a `…` glyph). Pass
//!   `Some(message)` to make it clickable (e.g. to open a dropdown of the
//!   elided crumbs); pass `None` for a purely visual placeholder.
//!   Mirrors `<BreadcrumbEllipsis>`.
//!
//! # Color cascade
//!
//! shadcn's `<BreadcrumbList>` applies `text-muted-foreground` once and
//! `<BreadcrumbPage>` overrides with `text-foreground`. We don't yet have a
//! breadcrumb [`Catalog`](icedxdy_core::theme) so the muted/foreground
//! distinction is the **caller's** responsibility — wrap link content in
//! [`text::muted`](crate::text::muted) for the canonical visual, and pass
//! the page content unstyled (default foreground). A real Catalog would
//! make this automatic.
//!
//! Animation gap: shadcn's `<BreadcrumbLink>` carries
//! `transition-colors hover:text-foreground`, so a link's color fades from
//! muted to foreground on hover. Because color is caller-driven (no Catalog
//! yet), our impl snaps — there is no hover-color animation. A future
//! Catalog pass would surface link state and restore the fade.
//!
//! # Separator
//!
//! Default separator is a `›` glyph in muted text — the widget crate has no
//! icon-font dependency, so callers pass `icon::chevron_right()` (or any
//! widget) via [`Breadcrumb::separator`] for lucide-icon parity.

use iced_core::mouse::Interaction;
use iced_widget::{Renderer, mouse_area};
use icedxdy_core::Theme;
use icedxdy_core::theme;

use crate::component::{Component, IntoElement};
use crate::row::Row;
use crate::text;
use crate::text::Text;

/// One entry in a breadcrumb trail.
///
/// Public via the `breadcrumb::Item` namespace path (per RUST_STYLE
/// no-composite-names: callers write `breadcrumb::Item`, never
/// `BreadcrumbItem`). Construct via the [`link`], [`page`], and [`ellipsis`]
/// helpers rather than the variants directly.
pub enum Item<'a, Message> {
    /// A clickable crumb. Mirrors `<BreadcrumbLink>`.
    Link(Component<'a, Message>, Message),
    /// The current page. Mirrors `<BreadcrumbPage>`.
    Page(Component<'a, Message>),
    /// An "..." indicator standing in for elided intermediate crumbs.
    /// Mirrors `<BreadcrumbEllipsis>`.
    Ellipsis(Option<Message>),
}

/// Build a clickable [`Item::Link`] crumb.
pub fn link<'a, Message: 'a>(
    text: impl Into<Component<'a, Message>>,
    on_press: Message,
) -> Item<'a, Message> {
    Item::Link(text.into(), on_press)
}

/// Build a non-clickable [`Item::Page`] crumb (the current page).
pub fn page<'a, Message: 'a>(text: impl Into<Component<'a, Message>>) -> Item<'a, Message> {
    Item::Page(text.into())
}

/// Build an [`Item::Ellipsis`] crumb. Pass `Some(message)` to make it
/// clickable, `None` for a static visual placeholder.
pub fn ellipsis<'a, Message: 'a>(on_press: Option<Message>) -> Item<'a, Message> {
    Item::Ellipsis(on_press)
}

/// Build a [`Breadcrumb`] from a list of items. The separator (default = a
/// muted `›` glyph) is auto-interspersed between adjacent items.
pub fn breadcrumb<'a, Message: 'a>(
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> Breadcrumb<'a, Message> {
    Breadcrumb::new(items)
}

/// Lazily-evaluated separator factory.
///
/// `Component` is not `Clone`, so the separator must be re-built once per
/// gap rather than stored once and copied. The factory closure is invoked
/// `items.len() - 1` times at `into_element` time.
type SeparatorFn<'a, Message> = Box<dyn Fn() -> Component<'a, Message> + 'a>;

/// A breadcrumb trail. Build via [`breadcrumb`].
pub struct Breadcrumb<'a, Message> {
    items: Vec<Item<'a, Message>>,
    separator: Option<SeparatorFn<'a, Message>>,
}

impl<'a, Message: 'a> Breadcrumb<'a, Message> {
    /// Build a breadcrumb from an iterator of items. Prefer [`breadcrumb`]
    /// at call sites for symmetry with other helpers.
    pub fn new(items: impl IntoIterator<Item = Item<'a, Message>>) -> Self {
        Self {
            items: items.into_iter().collect(),
            separator: None,
        }
    }

    /// Override the default separator. The closure is called once per gap
    /// between adjacent items (Component values are not `Clone`, so a fresh
    /// instance is built for each insertion).
    ///
    /// ```ignore
    /// breadcrumb([...])
    ///     .separator(|| icon::chevron_right().size(14).into())
    /// ```
    pub fn separator<F>(mut self, factory: F) -> Self
    where
        F: Fn() -> Component<'a, Message> + 'a,
    {
        self.separator = Some(Box::new(factory));
        self
    }
}

impl<'a, Message: Clone + 'a> From<Breadcrumb<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(b: Breadcrumb<'a, Message>) -> Self {
        Box::new(b).into_element()
    }
}

impl<'a, Message: Clone + 'a> From<Breadcrumb<'a, Message>> for Component<'a, Message> {
    fn from(b: Breadcrumb<'a, Message>) -> Self {
        Component::new(b)
    }
}

impl<'a, Message: Clone + 'a> IntoElement<'a, Message> for Breadcrumb<'a, Message> {
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        let Breadcrumb { items, separator } = *self;

        // Default separator: a 14px `›` glyph in muted-foreground text.
        // Mirrors shadcn's `<ChevronRightIcon className="size-3.5" />`
        // visual without requiring an icon font dependency in the widget
        // crate. Callers (e.g. the browser example) override with
        // `icon::chevron_right()` for true lucide-icon parity.
        let make_separator: SeparatorFn<'a, Message> =
            separator.unwrap_or_else(|| Box::new(default_separator));

        let item_count = items.len();
        // shadcn `<BreadcrumbList>` uses `gap-1.5` (6px) between adjacent
        // children (items + separators).
        let mut row = Row::new().spacing(6.0);

        for (idx, item) in items.into_iter().enumerate() {
            row = row.push(render_item(item));
            if idx + 1 < item_count {
                row = row.push(make_separator());
            }
        }

        Box::new(row).into_element()
    }
}

/// Render a single [`Item`] to a [`Component`]. Extracted so the dispatch
/// reads as a flat match in one place.
fn render_item<'a, Message: Clone + 'a>(item: Item<'a, Message>) -> Component<'a, Message> {
    match item {
        Item::Link(content, on_press) => render_link(content, on_press),
        Item::Page(content) => content,
        Item::Ellipsis(on_press) => render_ellipsis(on_press),
    }
}

/// Render a clickable link crumb. Wraps the content in `mouse_area` for
/// the Pointer cursor + click dispatch — matches the sidebar nav precedent
/// and shadcn's `<a>` semantics.
fn render_link<'a, Message: Clone + 'a>(
    content: Component<'a, Message>,
    on_press: Message,
) -> Component<'a, Message> {
    mouse_area(content.into_iced())
        .interaction(Interaction::Pointer)
        .on_press(on_press)
        .into()
}

/// Render an ellipsis indicator. `on_press = Some(_)` makes it clickable.
///
/// shadcn renders `<MoreHorizontalIcon />` inside a 20px-square
/// `flex size-5 items-center justify-center` box. Using Unicode `…`
/// (U+2026) keeps the widget crate font-free while reading as "more" at
/// body sizes.
fn render_ellipsis<'a, Message: Clone + 'a>(
    on_press: Option<Message>,
) -> Component<'a, Message> {
    let glyph: Component<'a, Message> = Text::new("\u{2026}")
        .size(text::size::SM)
        .style(theme::text::Muted)
        .into();

    match on_press {
        Some(message) => mouse_area(glyph.into_iced())
            .interaction(Interaction::Pointer)
            .on_press(message)
            .into(),
        None => glyph,
    }
}

/// Default separator factory: a muted `›` glyph at body size. Override with
/// [`Breadcrumb::separator`] for icon-based parity (e.g. `icon::chevron_right`).
fn default_separator<'a, Message: 'a>() -> Component<'a, Message> {
    Text::new("\u{203A}")
        .size(text::size::SM)
        .style(theme::text::Muted)
        .into()
}
