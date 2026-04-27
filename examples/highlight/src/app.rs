//! The Ready-state app — combo-box picker, code panel, loading overlay,
//! keyboard nav. Constructed by the loader in `main.rs` once the first
//! theme arrives; from that point on, the rest of the program never has
//! to test "are we ready yet?".
//!
//! Owns its own [`Message`] enum for everything user-driven (picker,
//! keyboard nav, no-op). Lifecycle messages (catalog/theme loaded) live
//! on the loader side and arrive here only as construction inputs.

use iced::widget::{combo_box, scrollable};
use iced::{Element, Fill, Font, Pixels, Subscription, Task, keyboard};
use iced_widget::stack;
use icedxdy::widget::markdown::highlighter;
use icedxdy::widget::text;
use icedxdy::widget::{center, container, markdown};
use icedxdy::{Theme, theme};

use crate::registry;

const SAMPLE: &str = include_str!("sample.txt");

/// User-driven app messages. Picker selection, keyboard nav, and the
/// markdown viewer's no-op. The loader's lifecycle messages
/// (catalog/theme loaded) live in `crate::Message` and don't appear here.
#[derive(Debug, Clone)]
pub enum Message {
    Selected(registry::Entry),
    Switch(Direction),
    Noop,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Next,
    Previous,
    First,
    Last,
}

pub struct App {
    entries: Vec<registry::Entry>,
    picker_state: combo_box::State<registry::Entry>,
    selected: registry::Entry,
    map: highlighter::Map,
    items: Vec<markdown::Item>,
    pending: Option<String>,
}

impl App {
    pub fn new(
        entries: Vec<registry::Entry>,
        slug: &str,
        map: highlighter::Map,
    ) -> Self {
        let selected = entry_for(&entries, slug);
        let picker_state = combo_box::State::new(entries.clone());
        let items = markdown::parse(SAMPLE).collect();
        Self {
            entries,
            picker_state,
            selected,
            map,
            items,
            pending: None,
        }
    }

    /// Swap in a freshly-loaded theme. Called by the loader on
    /// `ThemeLoaded(Ok)`. The picker selection is reaffirmed against
    /// the catalog so we never desync on edge cases.
    pub fn theme_loaded(&mut self, slug: &str, map: highlighter::Map) {
        self.selected = entry_for(&self.entries, slug);
        self.map = map;
        self.pending = None;
    }

    /// Drop the pending flag without changing the active theme.
    pub fn theme_load_failed(&mut self) {
        self.pending = None;
    }

    pub fn is_dark(&self) -> bool {
        self.map.is_dark()
    }

    /// Returns a `Task<crate::Message>` because the fetch this triggers
    /// produces a `ThemeLoaded` lifecycle event handled by the loader.
    pub fn update(&mut self, msg: Message) -> Task<crate::Message> {
        match msg {
            Message::Selected(entry) => {
                let slug = entry.slug.clone();
                self.pending = Some(slug.clone());
                self.selected = entry;
                fetch_theme_task(slug)
            }
            Message::Switch(direction) => {
                let Some(index) = entry_index(&self.entries, &self.selected.slug)
                else {
                    return Task::none();
                };
                let len = self.entries.len();
                let next_index = match direction {
                    Direction::Next => (index + 1) % len,
                    Direction::Previous => (index + len - 1) % len,
                    Direction::First => 0,
                    Direction::Last => len - 1,
                };
                let entry = self.entries[next_index].clone();
                let slug = entry.slug.clone();
                self.pending = Some(slug.clone());
                self.selected = entry;
                fetch_theme_task(slug)
            }
            Message::Noop => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        let picker = combo_box(
            &self.picker_state,
            "Pick a theme\u{2026}",
            Some(&self.selected),
            Message::Selected,
        );

        let label = self.pending.as_ref().map(|slug| {
            // Mirrors the browser's "View Code" structure: an opaque inner
            // card (analog of the View Code button) centered inside a
            // gradient overlay (analog of the code-fade overlay). The
            // gradient lets the previous theme show through up top; the
            // card makes the "Loading" text readable in the middle.
            use icedxdy::Size::Lg;
            use icedxdy::theme::tweak::{Opaque, radius};
            let card = container(text::muted(format!("Loading {slug}\u{2026}")))
                .padding([8, 16])
                .style(theme::container::Card + Opaque + radius(Lg));
            center(card).style(theme::container::CodeFadeOverlay)
        });

        let mut style = markdown::Style::from(&iced::Theme::TokyoNight);
        style.code_block_font = Font::MONOSPACE;
        let settings = markdown::Settings {
            code_size: Pixels(13.0),
            ..markdown::Settings::with_style(style)
        };

        let code = markdown::view_with(&self.items, settings, self).map(|_| Message::Noop);

        let code_card = stack![
            container(code)
                .padding(16)
                .width(Fill)
                .style(theme::container::CodeBlock),
            label
        ];

        let count = text::muted(self.map.populated_count());

        let body = iced::widget::column![picker, code_card, count]
            .spacing(20)
            .padding(24)
            .max_width(900);

        scrollable(container(body).center_x(Fill))
            .height(Fill)
            .into()
    }

    pub fn subscription() -> Subscription<Message> {
        use keyboard::key::{Key, Named};

        keyboard::listen().filter_map(|event| {
            let keyboard::Event::KeyPressed { key, .. } = event else {
                return None;
            };
            match key {
                Key::Named(Named::PageDown)
                | Key::Named(Named::ArrowRight)
                | Key::Named(Named::ArrowDown) => Some(Message::Switch(Direction::Next)),
                Key::Named(Named::PageUp)
                | Key::Named(Named::ArrowLeft)
                | Key::Named(Named::ArrowUp) => Some(Message::Switch(Direction::Previous)),
                Key::Named(Named::Home) => Some(Message::Switch(Direction::First)),
                Key::Named(Named::End) => Some(Message::Switch(Direction::Last)),
                _ => None,
            }
        })
    }
}

impl<'a> markdown::Viewer<'a, markdown::Uri, Theme, iced::Renderer> for App {
    fn on_link_click(url: markdown::Uri) -> markdown::Uri {
        url
    }

    fn code_theme(&'a self) -> Option<&'a dyn highlighter::Theme> {
        Some(&self.map)
    }
}

/// Spawn an async fetch for a theme by slug. Used by both the loader
/// (initial fetch on catalog-loaded) and the App (picker + keyboard nav).
/// Produces a `ThemeLoaded` lifecycle message handled by the loader.
pub fn fetch_theme_task(slug: String) -> Task<crate::Message> {
    let report_slug = slug.clone();
    Task::perform(
        async move {
            let body = registry::fetch_theme(slug).await?;
            highlighter::Map::from_vscode_theme(&body).map_err(|e| e.to_string())
        },
        move |result| crate::Message::ThemeLoaded(report_slug.clone(), result),
    )
}

fn entry_for(entries: &[registry::Entry], slug: &str) -> registry::Entry {
    entries
        .iter()
        .find(|e| e.slug == slug)
        .cloned()
        .unwrap_or_else(|| registry::Entry {
            slug: slug.to_owned(),
            display_name: slug.to_owned(),
        })
}

fn entry_index(entries: &[registry::Entry], slug: &str) -> Option<usize> {
    entries.iter().position(|e| e.slug == slug)
}
