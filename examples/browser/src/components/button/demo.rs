use icedxdy::Element;
use icedxdy::widget::{button, row};

use crate::icon;
use super::Message;

pub fn view<'a>() -> Element<'a, Message> {
    row![
        button::outline("Button").on_press(Message::Pressed),
        button::icon(icon::arrow_up())
            .on_press(Message::Pressed),
    ]
    .spacing(8)
    .into()
}
