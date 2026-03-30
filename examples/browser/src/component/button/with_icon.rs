use icedxdy::Component;
use icedxdy::widget::{button, row};
use icedxdy::theme;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    row![
        button![icon::git_branch(), "New Branch"]
            .style(theme::button::outline)
            .on_press(Message::Pressed),
        button!["Fork", icon::git_branch()]
            .style(theme::button::outline)
            .on_press(Message::Pressed),
    ]
    .spacing(8)
    .into()
}
