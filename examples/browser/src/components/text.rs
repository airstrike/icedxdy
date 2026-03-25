use iced::widget::column;
use iced::Task;

use icedxdy::widget::text;
use icedxdy::Element;

#[derive(Debug, Clone)]
pub enum Message {}

pub struct State;

impl State {
    pub fn new() -> (Self, Task<Message>) {
        (Self, Task::none())
    }

    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text::h1("Heading 1"),
            text::h2("Heading 2"),
            text::h3("Heading 3"),
            text::h4("Heading 4"),
            text::p("Paragraph text - the quick brown fox jumps over the lazy dog."),
            text::lead("Lead text - introductory paragraph style."),
            text::large("Large text"),
            text::small("Small text"),
            text::muted("Muted text - secondary information."),
            text::xs("Extra small text - fine print and labels."),
        ]
        .spacing(12)
        .into()
    }
}
