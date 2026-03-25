use icedxdy::Element;
use icedxdy::widget::{button, row, spinner};
use icedxdy::theme;

use super::Message;

pub fn view<'a>() -> Element<'a, Message> {
    row![
        button![spinner(), "Generating"]
            .style(theme::button::outline),
        button!["Downloading", spinner()]
            .style(theme::button::secondary),
    ]
    .spacing(16)
    .into()
}
