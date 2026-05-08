//! Breadcrumb component page — mirrors
//! <https://ui.shadcn.com/docs/components/radix/breadcrumb>.
//!
//! # Intentional skips
//!
//! - `breadcrumb-rtl` — RTL is not applicable to iced (per CLAUDE.md).
//! - `breadcrumb-dropdown` — requires DropdownMenu, not yet ported. Defer
//!   until Dropdown lands. Note: shadcn's `breadcrumb-demo.tsx` (used as the
//!   docs hero) wires the ellipsis as a `<DropdownMenu>` trigger with a
//!   ghost `<Button>` containing `<MoreHorizontalIcon />`. Our hero renders
//!   the same trail with a clickable `breadcrumb::ellipsis(Some(_))` — the
//!   visual is identical, just no menu opens.
//!
//! # API Reference (deferred)
//!
//! The live docs API Reference table lists 7 components, each with the props
//! shadcn exposes:
//!
//! - `Breadcrumb` — `className: string` (no default)
//! - `BreadcrumbList` — `className: string` (no default)
//! - `BreadcrumbItem` — `className: string` (no default)
//! - `BreadcrumbLink` — `className: string` (no default)
//! - `BreadcrumbPage` — `className: string` (no default)
//! - `BreadcrumbSeparator` — `children: React.ReactNode`, `className: string`
//!   (no defaults)
//! - `BreadcrumbEllipsis` — `className: string` (no default)
//!
//! In our port, `className` collapses to widget-level styling (delegated to
//! the (currently absent) breadcrumb Catalog), `children` for the separator
//! is the `Breadcrumb::separator(factory)` builder, and the `Item` enum
//! flattens the JSX nesting (no separate `BreadcrumbList`/`BreadcrumbItem`).

// TODO: API Reference section once a table widget lands — see module doc
// for the 7 props the live docs covers.

use iced::Task;

use icedxdy::theme;
use icedxdy::widget::{breadcrumb, text};
use icedxdy::{Component, Theme};

use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Breadcrumb",
    description: "Displays the path to the current resource using a hierarchy of links.",
};

pub static BLOCKS: &[page::Block] = &[
    // Hero example — first card, no title heading.
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/breadcrumb-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some(
                    "A basic breadcrumb with a home link and a components link.",
                ),
                snippet: "examples/browser/snippets/breadcrumb-basic.md",
            },
            page::Block::Example {
                id: "separator",
                title: "Custom separator",
                description: Some(
                    "Use a custom component as `children` for `<BreadcrumbSeparator />` to create a custom separator.",
                ),
                snippet: "examples/browser/snippets/breadcrumb-separator.md",
            },
            page::Block::Example {
                id: "ellipsis",
                title: "Collapsed",
                description: Some(
                    "We provide a `<BreadcrumbEllipsis />` component to show a collapsed state when the breadcrumb is too long.",
                ),
                snippet: "examples/browser/snippets/breadcrumb-ellipsis.md",
            },
            page::Block::Example {
                id: "link",
                title: "Link component",
                description: Some(
                    "To use a custom link component from your routing library, you can use the `asChild` prop on `<BreadcrumbLink />`.",
                ),
                snippet: "examples/browser/snippets/breadcrumb-link.md",
            },
        ],
    },
];

/// Per-page message — each link / ellipsis emits a no-op variant the page
/// folds into [`page::Message::Noop`]. The shadcn examples don't actually
/// wire navigation in iced (no router); the message dispatch only exists
/// to demonstrate the cursor + click affordance.
#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
}

pub struct BreadcrumbPage {
    state: page::State,
}

impl BreadcrumbPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// Build the lucide-icon separator factory used by every example. Mirrors
/// shadcn's default `<ChevronRightIcon className="size-3.5" />`. iced's
/// `Text::style` takes a `Fn(&Theme) -> Style` closure (no `Into<StyleFn>`
/// shorthand on the iced widget itself), so we explicitly thread the
/// muted-foreground palette color through a closure here.
fn chevron_separator<'a>() -> impl Fn() -> Component<'a, Message> + 'a {
    || {
        icon::chevron_right()
            .size(14)
            .style(|t: &Theme| iced_widget::text::Style {
                color: Some(t.palette().muted.foreground.into()),
            })
            .into()
    }
}

fn view(id: &str) -> Component<'static, Message> {
    match id {
        // Hero / `breadcrumb-demo.tsx`. shadcn's demo wires the ellipsis up
        // as a DropdownMenu trigger (Documentation / Themes / GitHub items).
        // Without a dropdown widget, we render the same trail with a
        // clickable ellipsis that dispatches `Message::Pressed` — the
        // visual is identical, just no menu opens.
        "demo" => breadcrumb([
            breadcrumb::link(text::muted("Home"), Message::Pressed),
            breadcrumb::ellipsis(Some(Message::Pressed)),
            breadcrumb::link(text::muted("Components"), Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(chevron_separator())
        .into(),

        // Basic — `breadcrumb-basic.tsx`. Three crumbs, default chevron.
        "basic" => breadcrumb([
            breadcrumb::link(text::muted("Home"), Message::Pressed),
            breadcrumb::link(text::muted("Components"), Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(chevron_separator())
        .into(),

        // Custom separator — `breadcrumb-separator.tsx`. Uses `<DotIcon />`
        // in place of the default chevron. Without a `dot` lucide icon in
        // our generated set, render a centered Unicode bullet `•` (U+2022)
        // at body size — visually equivalent to lucide's `dot` glyph.
        "separator" => breadcrumb([
            breadcrumb::link(text::muted("Home"), Message::Pressed),
            breadcrumb::link(text::muted("Components"), Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(|| {
            text::small("\u{2022}")
                .style(theme::text::Muted)
                .into()
        })
        .into(),

        // Collapsed — `breadcrumb-ellipsis.tsx`. Static ellipsis between
        // Home and Components (no dropdown wired — it's a visual indicator
        // that the trail has been collapsed).
        "ellipsis" => breadcrumb([
            breadcrumb::link(text::muted("Home"), Message::Pressed),
            breadcrumb::ellipsis(None),
            breadcrumb::link(text::muted("Components"), Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(chevron_separator())
        .into(),

        // Link component — `breadcrumb-link.tsx`. shadcn's example uses
        // `asChild` + Next.js `<Link>` for routing integration. In iced
        // there's no DOM routing primitive, so the visual collapses to the
        // basic example — included separately to mirror the docs page TOC.
        "link" => breadcrumb([
            breadcrumb::link(text::muted("Home"), Message::Pressed),
            breadcrumb::link(text::muted("Components"), Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(chevron_separator())
        .into(),

        _ => unreachable!("unknown breadcrumb preview id: {id}"),
    }
}

impl page::Page for BreadcrumbPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<crate::Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<crate::Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, crate::Message, Theme> {
        let pi = self.state.page_index;
        view(id)
            .map(move |_| crate::Message::Page(pi, page::Message::Noop))
            .into()
    }
}
