//! Highlight playground — render the same Rust snippet under any theme
//! from the shiki `tm-themes` catalog (~60 VSCode-format themes).
//!
//! Cold start fetches the catalog from jsdelivr (cached 24h) then loads a
//! default theme matching the system light/dark preference. The combo_box
//! picker lets you switch — each pick triggers a fetch (cached forever
//! per slug). Network is required: there is no bundled fallback.
//!
//! `main.rs` owns the loader state machine + splash screens. Once a theme
//! arrives we hand off to `app::App`, which owns its own `Message` enum
//! for everything user-driven.

use iced::widget::column;
use iced::{Element, Subscription, Task};
use icedxdy::widget::markdown::highlighter;
use icedxdy::widget::{center, text};
use icedxdy::{Theme, theme};

mod app;
mod registry;

use app::{App, fetch_theme_task};

fn main() -> iced::Result {
    iced::application(Highlights::new, Highlights::update, Highlights::view)
        .title("icedxdy \u{2022} highlight playground")
        .antialiasing(true)
        .subscription(Highlights::subscription)
        .theme(Highlights::theme)
        .run()
}

/// Loader state machine. The actual UI lives in `App`; everything else
/// here is splash screens + the catalog/theme fetch handshake.
enum Highlights {
    LoadingCatalog,
    LoadingTheme {
        entries: Vec<registry::Entry>,
        slug: String,
    },
    Ready(App),
    Failed(String),
}

/// Lifecycle messages owned by the loader. App-driven events
/// (combo-box picks, keyboard nav) live in `app::Message` and flow
/// through the `App` wrapper variant.
#[derive(Debug, Clone)]
pub enum Message {
    CatalogLoaded(Result<Vec<registry::Entry>, String>),
    ThemeLoaded(String, Result<highlighter::Map, String>),
    App(app::Message),
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
                        Highlights::Ready(App::new(entries, &slug, map))
                    }
                    Highlights::Ready(mut app) => {
                        app.theme_loaded(&slug, map);
                        Highlights::Ready(app)
                    }
                    other => other,
                };
                Task::none()
            }
            Message::ThemeLoaded(slug, Err(e)) => {
                if let Highlights::Ready(app) = self {
                    app.theme_load_failed();
                }
                eprintln!("theme {slug} failed: {e}");
                Task::none()
            }
            Message::App(app_msg) => match self {
                Highlights::Ready(app) => app.update(app_msg),
                _ => Task::none(),
            },
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        match self {
            Highlights::LoadingCatalog => splash("Loading theme catalog\u{2026}"),
            Highlights::LoadingTheme { slug, .. } => {
                splash(&format!("Loading theme '{slug}'\u{2026}"))
            }
            Highlights::Failed(msg) => splash(msg),
            Highlights::Ready(app) => app.view().map(Message::App),
        }
    }

    fn theme(&self) -> Theme {
        let prefer_dark = match self {
            Highlights::Ready(app) => app.is_dark(),
            _ => theme::is_dark_mode(),
        };
        if prefer_dark {
            Theme::dark()
        } else {
            Theme::light()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self {
            Highlights::Ready(_) => App::subscription().map(Message::App),
            _ => Subscription::none(),
        }
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
