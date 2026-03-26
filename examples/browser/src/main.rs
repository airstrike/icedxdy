mod components;
mod fonts;
mod icon;

use iced::widget::{column, combo_box, row, scrollable, space};
use iced::{Fill, Font, Pixels, Task};

use icedxdy::widget::{button, container, separator, text};
use icedxdy::Size::*;
use icedxdy::{theme, Theme};

fn is_dark_mode() -> bool {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "Dark")
            .unwrap_or(true) // default to dark
    }
    #[cfg(not(target_os = "macos"))]
    {
        true // default to dark on other platforms
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("icedxdy • components browser")
        .antialiasing(true)
        .theme(App::theme)
        .font(icon::FONT)
        .run()
}

/// Fonts to pin at the top of the font list
const PINNED_FONTS: &[&str] = &["Geist", "Inter"];

struct App {
    dark_mode: bool,
    section: Section,
    button: components::button::State,
    text: components::text::State,
    fount: fount::Fount,
    font_list: combo_box::State<String>,
    current_font: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Section {
    #[default]
    Button,
    Text,
}

impl Section {
    const ALL: &[Section] = &[Section::Button, Section::Text];

    fn title(&self) -> &'static str {
        match self {
            Section::Button => "Button",
            Section::Text => "Text",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Section::Button => "Displays a button or a component that looks like a button.",
            Section::Text => "Typography components for headings, paragraphs, and more.",
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,
    SelectSection(Section),
    FontSelected(String),
    Button(components::button::Message),
    Text(components::text::Message),
    Font(fonts::Message),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let (btn, button_task) = components::button::State::new();
        let (txt, text_task) = components::text::State::new();

        let default_font = "Geist".to_string();

        (
            Self {
                dark_mode: is_dark_mode(),
                section: Section::default(),
                button: btn,
                text: txt,
                fount: fount::Fount::new(),
                font_list: combo_box::State::new(vec![default_font.clone()]),
                current_font: default_font,
            },
            Task::batch([
                button_task.map(Message::Button),
                text_task.map(Message::Text),
                fonts::init().map(Message::Font),
            ]),
        )
    }

    fn rebuild_font_list(&mut self) {
        let mut names = self.fount.families();

        // Move pinned fonts to the top (in reverse so first pinned ends up first)
        for &pinned in PINNED_FONTS.iter().rev() {
            if let Some(pos) = names.iter().position(|n| n == pinned) {
                let name = names.remove(pos);
                names.insert(0, name);
            }
        }

        self.font_list = combo_box::State::new(names);
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                Task::none()
            }
            Message::SelectSection(section) => {
                self.section = section;
                Task::none()
            }
            Message::FontSelected(name) => {
                self.current_font = name.clone();

                if self.fount.system_families().contains(&name) {
                    // System font — already available
                    let name: &'static str = Box::leak(name.into_boxed_str());
                    iced::font::set_defaults(Font::new(name), Pixels(14.0))
                } else {
                    // Google Font — load it
                    fonts::load(name, self.fount.google_catalog()).map(Message::Font)
                }
            }
            Message::Button(msg) => self.button.update(msg).map(Message::Button),
            Message::Text(msg) => self.text.update(msg).map(Message::Text),
            Message::Font(msg) => match msg {
                fonts::Message::SystemFontsLoaded(families) => {
                    let names: Vec<String> = families
                        .into_iter()
                        .map(|f| f.to_string())
                        .filter(|name| !name.starts_with('.'))
                        .collect();
                    self.fount.set_system_families(names);
                    self.rebuild_font_list();
                    Task::none()
                }
                fonts::Message::CatalogLoaded(Ok(catalog)) => {
                    self.fount.set_google_catalog(catalog);
                    self.rebuild_font_list();
                    fonts::load(
                        self.current_font.clone(),
                        self.fount.google_catalog(),
                    )
                    .map(Message::Font)
                }
                fonts::Message::CatalogLoaded(Err(_)) => Task::none(),
                fonts::Message::Loaded(name, Err(e)) => {
                    eprintln!("Font loading failed ({name}): {e}");
                    Task::none()
                }
                fonts::Message::Loaded(name, Ok(())) => {
                    let name: &'static str = Box::leak(name.into_boxed_str());
                    iced::font::set_defaults(Font::new(name), Pixels(14.0))
                }
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Message, Theme> {
        let branding = text::h4("icedxdy");

        let font_selector = combo_box(
            &self.font_list,
            "Font…",
            Some(&self.current_font),
            Message::FontSelected,
        )
        .width(160)
        .size(12);

        let theme_icon = if self.dark_mode {
            icon::sun().size(16)
        } else {
            icon::moon().size(16)
        };
        let theme_toggle = button(theme_icon)
            .style(theme::button::ghost)
            .on_press(Message::ToggleTheme);

        let header = row![branding, space::horizontal(), font_selector, theme_toggle]
            .spacing(8)
            .padding(16)
            .align_y(iced::Center);

        // Sidebar
        let sidebar_header = container(text::muted("Components")).padding([0, 8]);

        let sidebar_items = column(Section::ALL.iter().map(|section| {
            let is_active = *section == self.section;
            let style: fn(&Theme, _) -> _ = if is_active {
                theme::button::sidebar_active
            } else {
                theme::button::sidebar
            };
            button(section.title())
                .size(Sm)
                .style(style)
                .on_press(Message::SelectSection(*section))
                .into()
        }))
        .spacing(2);

        let sidebar = scrollable(
            column![sidebar_header, sidebar_items]
                .spacing(16)
                .padding([24, 8])
                .width(220),
        )
        .height(Fill);

        // Main content
        let title = text::h1(self.section.title());
        let description = text::p(self.section.description())
            .style(theme::text::muted);

        let preview = container(match self.section {
            Section::Button => self.button.view().map(Message::Button),
            Section::Text => self.text.view().map(Message::Text),
        })
        .width(Fill);

        let content = column![title, description, preview]
            .spacing(24)
            .padding(32)
            .width(Fill);

        let thin_scrollbar = scrollable::Scrollbar::default()
            .width(1)
            .scroller_width(2);

        let mut main = scrollable(content)
            .direction(scrollable::Direction::Vertical(thin_scrollbar))
            .width(Fill)
            .height(Fill);

        // Attach scrollable id for Button section's scroll-to support
        if self.section == Section::Button {
            main = main.id(self.button.scrollable_id());
        }

        let body = match self.section {
            Section::Button => {
                let on_this_page = self.button.on_this_page().map(Message::Button);
                row![sidebar, main, on_this_page]
                    .width(Fill)
                    .height(Fill)
            }
            _ => {
                row![sidebar, main]
                    .width(Fill)
                    .height(Fill)
            }
        };

        column![header, separator::horizontal(), body]
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::dark()
        } else {
            Theme::light()
        }
    }
}
