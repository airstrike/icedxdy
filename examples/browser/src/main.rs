//! Components-browser entry point.
//!
//! Mirrors the loader-state-machine pattern in `examples/highlight`: a
//! [`Browser`] state machine handles the splash screen while we parse the
//! vendored shadcn code themes and kick off the font catalog fetch, then
//! hands off to [`app::App`] once the first font message arrives.
//!
//! `app::App` owns the actual UI. Everything else lives in `main.rs` only
//! to keep startup state out of the runtime path.

mod app;
mod component;
mod example;
mod fonts;
mod icon;
mod page;

use iced::widget::column;
use iced::{Element, Subscription, Task};
use icedxdy::widget::markdown::highlighter::Map;
use icedxdy::widget::{center, text};
use icedxdy::Theme;

use app::App;

/// Vendored shadcn-flavour code themes (Shiki ships these in shadcn's docs
/// site). Used verbatim — no overrides — so example renderings match the
/// shadcn documentation snippets exactly. Loaded synchronously via
/// `include_str!` and parsed once at startup before [`App`] is constructed.
const GH_LIGHT: &str = include_str!("../themes/github-light-default.json");
const GH_DARK: &str = include_str!("../themes/github-dark.json");

fn parse_code_map(json: &str) -> Map {
    Map::from_vscode_theme(json).expect("vendored github theme parses")
}

fn main() -> iced::Result {
    iced::application(Browser::new, Browser::update, Browser::view)
        .title("icedxdy \u{2022} components browser")
        .antialiasing(true)
        .subscription(Browser::subscription)
        .theme(Browser::theme)
        .font(icon::FONT)
        .run()
}

/// Re-export so pages and other callers continue to use `crate::Message`.
pub use app::Message;

/// Loader state. The [`App`] is constructed at startup so it can absorb
/// font/catalog events from the moment they arrive; the splash overlay
/// stays visible until the default font binary actually finishes loading.
struct Browser {
    app: App,
    font_loaded: bool,
    /// Visual hint for splash progression. Bumped by phase as font-loading
    /// events arrive (system list -> Google catalog -> font bytes). Clamped
    /// to never decrease, so out-of-order events still progress visually.
    load_progress: f32,
}

impl Browser {
    fn new() -> (Self, Task<Message>) {
        let gh_light = parse_code_map(GH_LIGHT);
        let gh_dark = parse_code_map(GH_DARK);
        let (app, app_init) = App::new(gh_light, gh_dark);
        let fonts_init = fonts::init().map(Message::Font);
        (
            Browser {
                app,
                font_loaded: false,
                load_progress: 0.0,
            },
            Task::batch([app_init, fonts_init]),
        )
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        // Phase-based splash progress. Each terminal stage clamps the bar
        // upward; the `.max()` keeps reordered events monotone.
        if let Message::Font(font_msg) = &msg {
            let bump = match font_msg {
                fonts::Message::SystemFontsLoaded(_) => 25.0,
                fonts::Message::CatalogLoaded(_) => 50.0,
                fonts::Message::Loaded(_, _) => 75.0,
            };
            self.load_progress = self.load_progress.max(bump);
        }
        // Dismiss the splash on any terminal font event — success
        // (`Loaded(_, Ok(_))`), per-font failure (`Loaded(_, Err(_))`), or
        // catalog failure (`CatalogLoaded(Err(_))`). Without the failure
        // arms we'd splash forever offline.
        if matches!(
            &msg,
            Message::Font(fonts::Message::Loaded(_, _))
                | Message::Font(fonts::Message::CatalogLoaded(Err(_)))
        ) {
            self.font_loaded = true;
        }
        self.app.update(msg)
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        if self.font_loaded {
            self.app.view()
        } else {
            splash("Loading\u{2026}", self.load_progress)
        }
    }

    fn theme(&self) -> Theme {
        if self.app.is_dark() {
            Theme::dark()
        } else {
            Theme::light()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.font_loaded {
            self.app.subscription()
        } else {
            Subscription::none()
        }
    }
}

fn splash<'a>(message: &str, progress_value: f32) -> Element<'a, Message, Theme> {
    center(
        column![
            text::h1("icedxdy"),
            text::muted(message.to_owned()),
            icedxdy::widget::progress(progress_value).width(140),
        ]
        .spacing(12)
        .align_x(iced::Center),
    )
    .into()
}
