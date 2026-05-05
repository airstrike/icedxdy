use icedxdy::Component;
use icedxdy::Size::*;
use icedxdy::widget::{button, row};

use super::Message;
use crate::icon;

pub fn view<'a>() -> Component<'a, Message> {
    row![
        row![
            button::outline("Extra Small")
                .size(Xs)
                .on_press(Message::Pressed),
            button::icon(icon::arrow_up_right())
                .size(Xs)
                .on_press(Message::Pressed),
        ]
        .spacing(8),
        row![
            button::outline("Small").size(Sm).on_press(Message::Pressed),
            button::icon(icon::arrow_up_right())
                .size(Sm)
                .on_press(Message::Pressed),
        ]
        .spacing(8),
        row![
            button::outline("Default").on_press(Message::Pressed),
            button::icon(icon::arrow_up_right()).on_press(Message::Pressed),
        ]
        .spacing(8),
        row![
            button::outline("Large").size(Lg).on_press(Message::Pressed),
            button::icon(icon::arrow_up_right())
                .size(Lg)
                .on_press(Message::Pressed),
        ]
        .spacing(8),
    ]
    .spacing(16)
    .into()
}
