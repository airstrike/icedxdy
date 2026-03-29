use icedxdy::Component;
use icedxdy::widget::{button, row, spinner};
use icedxdy::theme;

use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    row![
        button![spinner(), "Generating"]
            .style(theme::button::outline),
        button!["Downloading", spinner()]
            .style(theme::button::secondary),
    ]
    .spacing(16)
    .into()
}
