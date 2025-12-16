use iced::Element;
use iced::widget::{button, column, row, text};

use icedxdy::Theme;
use icedxdy::theme::button as btn;

#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
}

pub fn view<'a>() -> Element<'a, Message, Theme> {
    let variants = column![
        text("Variants").size(16),
        row![
            button(text("Default")).on_press(Message::Pressed),
            button(text("Secondary"))
                .on_press(Message::Pressed)
                .style(btn::secondary),
            button(text("Outline"))
                .on_press(Message::Pressed)
                .style(btn::outline),
            button(text("Ghost"))
                .on_press(Message::Pressed)
                .style(btn::ghost),
            button(text("Destructive"))
                .on_press(Message::Pressed)
                .style(btn::destructive),
            button(text("Link"))
                .on_press(Message::Pressed)
                .style(btn::link),
        ]
        .spacing(8),
    ]
    .spacing(8);

    let disabled = column![
        text("Disabled").size(16),
        row![
            button(text("Default")),
            button(text("Secondary")).style(btn::secondary),
            button(text("Outline")).style(btn::outline),
            button(text("Ghost")).style(btn::ghost),
            button(text("Destructive")).style(btn::destructive),
            button(text("Link")).style(btn::link),
        ]
        .spacing(8),
    ]
    .spacing(8);

    column![variants, disabled].spacing(24).into()
}
