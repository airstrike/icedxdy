use icedxdy::Size::*;
use icedxdy::Slot;
use icedxdy::widget::button;

use super::Message;
use crate::icon;

pub fn view<'a>() -> Slot<'a, Message> {
    button![git_branch, "New Branch"]
        .size(Sm)
        .style(button::Outline)
        .on_press(Message::Pressed)
        .into()
}
