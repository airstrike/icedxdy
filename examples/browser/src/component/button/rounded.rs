use icedxdy::Component;
use icedxdy::widget::{button, row};

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    row![
        button!["Get Started"].rounded().on_press(Message::Pressed),
        button::icon(icon::arrow_up())
            .rounded()
            .on_press(Message::Pressed),
    ]
    .spacing(8)
    .into()
}
