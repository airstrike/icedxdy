use icedxdy::Component;
use icedxdy::Size::*;
use icedxdy::theme;
use icedxdy::widget::button;

use super::Message;
use crate::icon;

pub fn view<'a>() -> Component<'a, Message> {
    button![git_branch, "New Branch"]
        .size(Sm)
        .style(theme::button::Outline)
        .on_press(Message::Pressed)
        .into()
}
