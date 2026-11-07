use icedxdy::Size::*;
use icedxdy::Slot;
use icedxdy::widget::{button, row};

use super::Message;
use crate::icon;

pub fn view<'a>() -> Slot<'a, Message> {
    row![
        row![
            button("Extra Small")
                .style(button::Outline)
                .size(Xs)
                .on_press(Message::Pressed),
            button(icon::arrow_up_right())
                .square()
                .style(button::Outline)
                .size(Xs)
                .on_press(Message::Pressed),
        ]
        .spacing(8),
        row![
            button("Small")
                .style(button::Outline)
                .size(Sm)
                .on_press(Message::Pressed),
            button(icon::arrow_up_right())
                .square()
                .style(button::Outline)
                .size(Sm)
                .on_press(Message::Pressed),
        ]
        .spacing(8),
        row![
            button("Default")
                .style(button::Outline)
                .on_press(Message::Pressed),
            button(icon::arrow_up_right())
                .square()
                .style(button::Outline)
                .on_press(Message::Pressed),
        ]
        .spacing(8),
        row![
            button("Large")
                .style(button::Outline)
                .size(Lg)
                .on_press(Message::Pressed),
            button(icon::arrow_up_right())
                .square()
                .style(button::Outline)
                .size(Lg)
                .on_press(Message::Pressed),
        ]
        .spacing(8),
    ]
    .spacing(16)
    .into()
}
