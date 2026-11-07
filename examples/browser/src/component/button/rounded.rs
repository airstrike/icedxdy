use icedxdy::Slot;
use icedxdy::widget::button;

use super::Message;
use crate::icon;

pub fn view<'a>() -> Slot<'a, Message> {
    button(icon::arrow_up())
        .square()
        .style(button::Outline)
        .rounded()
        .on_press(Message::Pressed)
        .into()
}
