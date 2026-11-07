//! Breadcrumb component page — mirrors
//! <https://ui.shadcn.com/docs/components/radix/breadcrumb>.
//!
//! # Intentional skips
//!
//! - `breadcrumb-rtl` — RTL is not applicable to iced (per CLAUDE.md).
//! - `breadcrumb-dropdown` — requires DropdownMenu, not yet ported. Defer
//!   until Dropdown lands. Note: shadcn's `breadcrumb-demo.tsx` (used as the
//!   docs hero) wires the ellipsis as a DropdownMenu trigger with a ghost
//!   icon button. Our hero renders the same trail with a clickable
//!   `breadcrumb::ellipsis(Some(_))` — the visual is identical, just no menu
//!   opens.
//!
//! # API Reference
//!
//! Rendered via the `breadcrumb-api-reference.md` snippet at the bottom of
//! BLOCKS. The live docs API Reference table lists 7 components; the
//! snippet captures the icedxdy-shaped builder API for `breadcrumb(...)`
//! plus the item constructors (`link`, `page`, `ellipsis`) and the
//! auto-interspersed separator.

use iced::Task;

use icedxdy::widget::breadcrumb;
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
                description: Some("A basic breadcrumb with a home link and a components link."),
                snippet: "examples/browser/snippets/breadcrumb-basic.md",
            },
            page::Block::Example {
                id: "separator",
                title: "Custom separator",
                description: Some(
                    "Pass a custom component to `.separator(...)` to override the default chevron.",
                ),
                snippet: "examples/browser/snippets/breadcrumb-separator.md",
            },
            page::Block::Example {
                id: "ellipsis",
                title: "Collapsed",
                description: Some(
                    "Use `breadcrumb::ellipsis(...)` to show a collapsed state when the breadcrumb is too long.",
                ),
                snippet: "examples/browser/snippets/breadcrumb-ellipsis.md",
            },
            page::Block::Example {
                id: "link",
                title: "Link component",
                description: Some(
                    "Pass a custom message to `breadcrumb::link(content, on_press)` to wire the crumb into your own routing layer.",
                ),
                snippet: "examples/browser/snippets/breadcrumb-link.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/breadcrumb-api-reference.md"),
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

fn chevron<'a>() -> Component<'a, Message> {
    icon::chevron_right().size(14).into()
}

fn view(id: &str) -> Component<'static, Message> {
    match id {
        // Hero / `breadcrumb-demo.tsx`. shadcn's demo wires the ellipsis up
        // as a DropdownMenu trigger (Documentation / Themes / GitHub items).
        // Without a dropdown widget, we render the same trail with a
        // clickable ellipsis that dispatches `Message::Pressed` — the
        // visual is identical, just no menu opens.
        "demo" => breadcrumb([
            breadcrumb::link("Home", Message::Pressed),
            breadcrumb::ellipsis(Some(Message::Pressed)),
            breadcrumb::link("Components", Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(chevron)
        .into(),

        // Basic — `breadcrumb-basic.tsx`. Three crumbs, default chevron.
        "basic" => breadcrumb([
            breadcrumb::link("Home", Message::Pressed),
            breadcrumb::link("Components", Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(chevron)
        .into(),

        // Custom separator — `breadcrumb-separator.tsx`. Uses lucide's `DotIcon`
        // in place of the default chevron. Without a `dot` lucide icon in
        // our generated set, render a centered Unicode bullet `•` (U+2022)
        // at body size — visually equivalent to lucide's `dot` glyph.
        "separator" => breadcrumb([
            breadcrumb::link("Home", Message::Pressed),
            breadcrumb::link("Components", Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(|| "\u{2022}".into())
        .into(),

        // Collapsed — `breadcrumb-ellipsis.tsx`. Static ellipsis between
        // Home and Components (no dropdown wired — it's a visual indicator
        // that the trail has been collapsed).
        "ellipsis" => breadcrumb([
            breadcrumb::link("Home", Message::Pressed),
            breadcrumb::ellipsis(None),
            breadcrumb::link("Components", Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(chevron)
        .into(),

        // Link component — `breadcrumb-link.tsx`. shadcn's example slots a
        // Next.js link via `asChild` for routing integration. In iced there's
        // no DOM routing primitive, so the visual collapses to the basic
        // example — included separately to mirror the docs page TOC.
        "link" => breadcrumb([
            breadcrumb::link("Home", Message::Pressed),
            breadcrumb::link("Components", Message::Pressed),
            breadcrumb::page("Breadcrumb"),
        ])
        .separator(chevron)
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
