use icedxdy::Slot;
use icedxdy::widget::{button, row, spinner};

use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    row![
        button![spinner(), "Generating"]
            .style(button::Outline)
            .disabled(true),
        button!["Downloading", spinner()]
            .style(button::Secondary)
            .disabled(true),
    ]
    .spacing(16)
    .into()
}
