use icedxdy::Component;
use icedxdy::theme;
use icedxdy::widget::{button, row, spinner};

use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    row![
        button![spinner(), "Generating"].style(theme::button::Outline),
        button!["Downloading", spinner()].style(theme::button::Secondary),
    ]
    .spacing(16)
    .into()
}
