use icedxdy::Component;
use icedxdy::Size::*;
use icedxdy::widget::button;
use icedxdy::theme;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    button![icon::git_branch(), "New Branch"]
        .size(Sm)
        .style(theme::button::Outline)
        .on_press(Message::Pressed)
        .into()
}
