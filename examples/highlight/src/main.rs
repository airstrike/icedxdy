//! Highlight playground — render the same Rust snippet under several
//! syntax-highlighting themes side by side.
//!
//! Lives in its own crate so iterating on themes / capture mappings
//! doesn't churn the components-browser API. Pick a theme from the row
//! at the top; the snippet re-renders below.
//!
//! Add a theme:
//! 1. Drop a TextMate-style `*.json` into `widget/src/markdown/themes/`
//!    (or anywhere — just adjust the `include_str!` path).
//! 2. Add a `(name, include_str!(...))` line to `THEMES`.

use iced::widget::{center, column, container, row, scrollable};
use iced::{Element, Fill, Font, Pixels, Task};
use iced_widget::button::StyleFn;
use icedxdy::Size::*;
use icedxdy::widget::markdown;
use icedxdy::widget::markdown::highlighter;
use icedxdy::widget::{button, text};
use icedxdy::{Theme, radius, theme};

const SAMPLE: &str = include_str!("sample.txt");

const THEMES: &[(&str, &str)] = &[
    (
        "github-light-default",
        include_str!("../themes/github-light-default.json"),
    ),
    ("github-dark", include_str!("../themes/github-dark.json")),
    ("tokyo-night", include_str!("../themes/tokyo-night.json")),
    ("monokai", include_str!("../themes/monokai.json")),
    ("dracula-soft", include_str!("../themes/dracula-soft.json")),
    ("dark-plus", include_str!("../themes/dark-plus.json")),
];

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("icedxdy \u{2022} highlight playground")
        .antialiasing(true)
        .theme(App::theme)
        .run()
}

enum App {
    Loading,
    Ready(Ready),
}

struct Ready {
    viewer: Viewer,
    items: Vec<markdown::Item>,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Vec<(String, highlighter::Map)>),
    Select(usize),
    Noop,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let load = Task::perform(
            async {
                THEMES
                    .iter()
                    .filter_map(|(name, json)| {
                        highlighter::Map::from_vscode_theme(json)
                            .ok()
                            .map(|map| (name.to_string(), map))
                    })
                    .collect()
            },
            Message::Loaded,
        );
        (App::Loading, load)
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Loaded(themes) => {
                let items: Vec<_> = markdown::parse(SAMPLE).collect();

                let selected = themes
                    .iter()
                    .position(|(_, m)| m.is_dark() == theme::is_dark_mode())
                    .unwrap_or(0);

                *self = App::Ready(Ready {
                    viewer: Viewer { themes, selected },
                    items,
                });
            }
            Message::Select(i) => {
                if let App::Ready(r) = self {
                    r.viewer.selected = i;
                }
            }
            Message::Noop => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        match self {
            App::Loading => center(
                column![text::h1("highlight"), text::muted("Parsing themes\u{2026}"),]
                    .spacing(8)
                    .align_x(iced::Center),
            )
            .into(),
            App::Ready(r) => r.view(),
        }
    }

    fn theme(&self) -> Theme {
        match self {
            App::Loading => Theme::light(),
            App::Ready(r) => {
                if r.viewer.themes[r.viewer.selected].1.is_dark() {
                    Theme::dark()
                } else {
                    Theme::light()
                }
            }
        }
    }
}

impl Ready {
    fn view(&self) -> Element<'_, Message, Theme> {
        let picker = row(self.viewer.themes.iter().enumerate().map(|(i, (name, _))| {
            let style: StyleFn<'_, Theme> = if i == self.viewer.selected {
                (theme::button::Default + radius(Lg)).into()
            } else {
                (theme::button::Outline + radius(Lg)).into()
            };
            button(name.as_str())
                .size(Sm)
                .style(style)
                .on_press(Message::Select(i))
                .into()
        }))
        .spacing(6);

        let mut style = markdown::Style::from(&iced::Theme::TokyoNight);
        style.code_block_font = Font::MONOSPACE;
        let settings = markdown::Settings {
            code_size: Pixels(13.0),
            ..markdown::Settings::with_style(style)
        };

        let active = &self.viewer.themes[self.viewer.selected];
        let label = text::muted(format!(
            "{}    \u{2022}    {} captures populated",
            active.0,
            active.1.populated_count(),
        ));

        let code = markdown::view_with(&self.items, settings, &self.viewer).map(|_| Message::Noop);

        let code_card = container(code)
            .padding(16)
            .width(Fill)
            .style(theme::container::code_block);

        let body = column![picker, label, code_card]
            .spacing(20)
            .padding(24)
            .max_width(900);

        scrollable(container(body).center_x(Fill))
            .height(Fill)
            .into()
    }
}

/// Holds every loaded theme + the active index. The `markdown::Viewer`
/// impl just returns the indexed `Map` — selecting a different theme is
/// a one-line `viewer.selected = i;` mutation, no re-parse, no realloc.
struct Viewer {
    themes: Vec<(String, highlighter::Map)>,
    selected: usize,
}

impl<'a> markdown::Viewer<'a, markdown::Uri, Theme, iced::Renderer> for Viewer {
    fn on_link_click(url: markdown::Uri) -> markdown::Uri {
        url
    }

    fn code_theme(&'a self) -> Option<&'a dyn highlighter::Theme> {
        Some(&self.themes[self.selected].1)
    }
}
