mod components;
mod fonts;
mod icon;

use iced::widget::{column, container, mouse_area, row, scrollable, space};
use iced::{Element, Fill, Font, Task};
use iced_widget::button::Button;

use icedxdy::{separator, text, Theme};

const INTER_REGULAR: &[u8] = include_bytes!("../../../fonts/assets/InterTight-Regular.ttf");
const INTER_MEDIUM: &[u8] = include_bytes!("../../../fonts/assets/InterTight-Medium.ttf");
const INTER_SEMIBOLD: &[u8] = include_bytes!("../../../fonts/assets/InterTight-SemiBold.ttf");
const INTER_BOLD: &[u8] = include_bytes!("../../../fonts/assets/InterTight-Bold.ttf");

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("icedxdy • components browser")
        .theme(App::theme)
        .font(icon::FONT)
        .font(INTER_REGULAR)
        .font(INTER_MEDIUM)
        .font(INTER_SEMIBOLD)
        .font(INTER_BOLD)
        .default_font(Font::new("Inter Tight"))
        .run()
}

struct App {
    dark_mode: bool,
    section: Section,
    button: components::button::State,
    text: components::text::State,
    fount: fount::Fount,
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
    Button(components::button::Message),
    Text(components::text::Message),
    Font(fonts::Message),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let (btn, button_task) = components::button::State::new();
        let (txt, text_task) = components::text::State::new();

        (
            Self {
                dark_mode: false,
                section: Section::default(),
                button: btn,
                text: txt,
                fount: fount::Fount::new(),
            },
            Task::batch([
                button_task.map(Message::Button),
                text_task.map(Message::Text),
                fonts::init().map(Message::Font),
            ]),
        )
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
                    Task::none()
                }
                fonts::Message::CatalogLoaded(Ok(catalog)) => {
                    self.fount.set_google_catalog(catalog);
                    Task::none()
                }
                fonts::Message::CatalogLoaded(Err(_)) => Task::none(),
                fonts::Message::Loaded(name, Err(e)) => {
                    eprintln!("Font loading failed ({name}): {e}");
                    Task::none()
                }
                fonts::Message::Loaded(_, Ok(())) => Task::none(),
            },
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        let branding = text::h4("icedxdy");
        let theme_icon = if self.dark_mode {
            icon::sun().size(16)
        } else {
            icon::moon().size(16)
        };
        let theme_toggle: Element<'_, Message, Theme> = Button::new(theme_icon)
            .padding([4, 8])
            .style(icedxdy::theme::button::ghost)
            .on_press(Message::ToggleTheme)
            .into();
        let header = row![branding, space::horizontal(), theme_toggle]
            .padding(16)
            .align_y(iced::Center);

        // Sidebar
        let sidebar_header = text::muted("Components");

        let sidebar_items = column(Section::ALL.iter().map(|section| {
            let is_active = *section == self.section;
            let label = if is_active {
                text::small(section.title())
            } else {
                text::muted(section.title())
            };
            mouse_area(label)
                .on_press(Message::SelectSection(*section))
                .into()
        }))
        .spacing(8);

        let sidebar = scrollable(
            column![sidebar_header, sidebar_items]
                .spacing(16)
                .padding(24)
                .width(200),
        )
        .height(Fill);

        // Main content
        let title = text::h1(self.section.title());
        let description = text::muted(self.section.description());

        let preview = container(match self.section {
            Section::Button => self.button.view().map(Message::Button),
            Section::Text => self.text.view().map(Message::Text),
        })
        .width(Fill);

        let content = column![title, description, preview]
            .spacing(16)
            .padding(32)
            .width(Fill);

        let main = scrollable(content).width(Fill).height(Fill);

        let body = row![sidebar, separator::vertical(), main]
            .width(Fill)
            .height(Fill);

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
