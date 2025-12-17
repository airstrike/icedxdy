mod components;

use iced::widget::{column, container, mouse_area, row, scrollable, space};
use iced::{Element, Fill, Task};

use icedxdy::{button, fonts, separator, text, Theme};

fn main() -> iced::Result {
    // Initialize font names before use
    fonts::set();

    let settings = iced::Settings {
        default_font: fonts::INTER.get(),
        fonts: fonts::load(),
        ..Default::default()
    };

    iced::application(App::new, App::update, App::view)
        .title("icedxdy • components browser")
        .theme(App::theme)
        .settings(settings)
        .run()
}

struct App {
    dark_mode: bool,
    section: Section,
    button: components::button::State,
    text: components::text::State,
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
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let (button, button_task) = components::button::State::new();
        let (text, text_task) = components::text::State::new();

        (
            Self {
                dark_mode: false,
                section: Section::default(),
                button,
                text,
            },
            Task::batch([
                button_task.map(Message::Button),
                text_task.map(Message::Text),
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
        }
    }

    fn view(&self) -> Element<Message, Theme> {
        // Header with branding and theme toggle
        let branding = text::h4("icedxdy");
        let theme_label = if self.dark_mode { "Light" } else { "Dark" };
        let theme_toggle = button::ghost(theme_label).on_press(Message::ToggleTheme);
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
