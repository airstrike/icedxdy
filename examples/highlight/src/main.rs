//! Highlight playground — render the same Rust snippet under any theme
//! from the shiki `tm-themes` catalog (~60 VSCode-format themes).
//!
//! Cold start fetches the catalog from jsdelivr (cached 24h) then loads a
//! default theme matching the system light/dark preference. The combo_box
//! picker lets you switch — each pick triggers a fetch (cached forever
//! per slug). Network is required: there is no bundled fallback.

use iced::widget::{column, combo_box, scrollable};
use iced::{Element, Fill, Font, Pixels, Subscription, Task, keyboard};
use iced_widget::stack;
use icedxdy::widget::markdown::highlighter;
use icedxdy::widget::text;
use icedxdy::widget::{center, container, markdown};
use icedxdy::{Theme, theme};

mod registry;

const SAMPLE: &str = include_str!("sample.txt");

fn main() -> iced::Result {
    iced::application(Highlights::new, Highlights::update, Highlights::view)
        .title("icedxdy \u{2022} highlight playground")
        .antialiasing(true)
        .subscription(Highlights::subscription)
        .theme(Highlights::theme)
        .run()
}

enum Highlights {
    LoadingCatalog,
    LoadingTheme {
        entries: Vec<registry::Entry>,
        slug: String,
    },
    Ready(App),
    Failed(String),
}

struct App {
    entries: Vec<registry::Entry>,
    picker_state: combo_box::State<registry::Entry>,
    selected: registry::Entry,
    map: highlighter::Map,
    items: Vec<markdown::Item>,
    pending: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    CatalogLoaded(Result<Vec<registry::Entry>, String>),
    ThemeLoaded(String, Result<highlighter::Map, String>),
    Selected(registry::Entry),
    Noop,

    Switch(Direction),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Next,
    Previous,
    First,
    Last,
}

impl Highlights {
    fn new() -> (Self, Task<Message>) {
        let load = Task::perform(registry::fetch_catalog(), Message::CatalogLoaded);
        (Highlights::LoadingCatalog, load)
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::CatalogLoaded(Ok(entries)) => {
                let slug = pick_default(&entries);
                *self = Highlights::LoadingTheme {
                    entries,
                    slug: slug.clone(),
                };
                fetch_theme_task(slug)
            }
            Message::CatalogLoaded(Err(e)) => {
                *self = Highlights::Failed(format!("Failed to load catalog: {e}"));
                Task::none()
            }
            Message::ThemeLoaded(slug, Ok(map)) => {
                let placeholder = std::mem::replace(self, Highlights::LoadingCatalog);
                *self = match placeholder {
                    Highlights::LoadingTheme { entries, .. } => {
                        let selected = entry_for(&entries, &slug);
                        let picker_state = combo_box::State::new(entries.clone());
                        let items = markdown::parse(SAMPLE).collect();
                        Highlights::Ready(App {
                            entries,
                            picker_state,
                            selected,
                            map,
                            items,
                            pending: None,
                        })
                    }
                    Highlights::Ready(mut r) => {
                        r.selected = entry_for(&r.entries, &slug);
                        r.map = map;
                        r.pending = None;
                        Highlights::Ready(r)
                    }
                    other => other,
                };
                Task::none()
            }
            Message::ThemeLoaded(slug, Err(e)) => {
                if let Highlights::Ready(r) = self {
                    r.pending = None;
                }
                eprintln!("theme {slug} failed: {e}");
                Task::none()
            }
            Message::Selected(entry) => {
                if let Highlights::Ready(r) = self {
                    let slug = entry.slug.clone();
                    r.pending = Some(slug.clone());
                    r.selected = entry;
                    fetch_theme_task(slug)
                } else {
                    Task::none()
                }
            }

            Message::Switch(direction) => {
                let Highlights::Ready(r) = self else {
                    return Task::none();
                };
                let Some(index) = entry_index(&r.entries, &r.selected.slug) else {
                    return Task::none();
                };
                let next_index = match direction {
                    Direction::Next => (index + 1) % r.entries.len(),
                    Direction::Previous => (index + r.entries.len() - 1) % r.entries.len(),
                    Direction::First => 0,
                    Direction::Last => r.entries.len() - 1,
                };
                let entry = r.entries[next_index].clone();
                let slug = entry.slug.clone();
                r.pending = Some(slug.clone());
                r.selected = entry;
                fetch_theme_task(slug)
            }

            Message::Noop => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        match self {
            Highlights::LoadingCatalog => splash("Loading theme catalog\u{2026}"),
            Highlights::LoadingTheme { slug, .. } => {
                splash(&format!("Loading theme '{slug}'\u{2026}"))
            }
            Highlights::Failed(msg) => splash(msg),
            Highlights::Ready(r) => r.view(),
        }
    }

    fn theme(&self) -> Theme {
        let prefer_dark = match self {
            Highlights::Ready(r) => r.map.is_dark(),
            _ => theme::is_dark_mode(),
        };
        if prefer_dark {
            Theme::dark()
        } else {
            Theme::light()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
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

fn splash<'a>(message: &str) -> Element<'a, Message, Theme> {
    center(
        column![text::h1("highlight"), text::muted(message.to_owned())]
            .spacing(8)
            .align_x(iced::Center),
    )
    .into()
}

fn fetch_theme_task(slug: String) -> Task<Message> {
    let report_slug = slug.clone();
    Task::perform(
        async move {
            let body = registry::fetch_theme(slug).await?;
            highlighter::Map::from_vscode_theme(&body).map_err(|e| e.to_string())
        },
        move |result| Message::ThemeLoaded(report_slug.clone(), result),
    )
}

fn pick_default(entries: &[registry::Entry]) -> String {
    let preferred = if theme::is_dark_mode() {
        "github-dark"
    } else {
        "github-light"
    };
    if entries.iter().any(|e| e.slug == preferred) {
        return preferred.to_owned();
    }
    entries
        .first()
        .map(|e| e.slug.clone())
        .unwrap_or_else(|| preferred.to_owned())
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

impl App {
    fn view(&self) -> Element<'_, Message, Theme> {
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

        let body = column![picker, code_card, count]
            .spacing(20)
            .padding(24)
            .max_width(900);

        scrollable(container(body).center_x(Fill))
            .height(Fill)
            .into()
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
