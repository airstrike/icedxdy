mod components;

use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill, Task};

use icedxdy::Theme;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("icedxdy • components browser")
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {
    section: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Button,
}

impl Section {
    const ALL: &[Section] = &[Section::Button];

    fn title(&self) -> &'static str {
        match self {
            Section::Button => "Button",
        }
    }

    fn view(&self) -> Element<Message, Theme> {
        match self {
            Section::Button => components::button::view().map(Message::Button),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectSection(usize),
    Button(components::button::Message),
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectSection(index) => {
                self.section = index;
            }
            Message::Button(_msg) => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<Message, Theme> {
        let sidebar = column(Section::ALL.iter().enumerate().map(|(i, section)| {
            let is_active = i == self.section;
            button(text(section.title()))
                .on_press(Message::SelectSection(i))
                .style(if is_active {
                    icedxdy::theme::button::secondary
                } else {
                    icedxdy::theme::button::ghost
                })
                .into()
        }))
        .spacing(4)
        .padding(16)
        .width(200);

        let content = Section::ALL
            .get(self.section)
            .map(|s| s.view())
            .unwrap_or_else(|| text("Select a section").into());

        let main = container(content)
            .width(Fill)
            .height(Fill)
            .padding(32);

        row![sidebar, main]
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}
