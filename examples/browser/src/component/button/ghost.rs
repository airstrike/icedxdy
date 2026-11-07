use icedxdy::Slot;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button("Ghost")
        .style(button::Ghost)
        .on_press(Message::Pressed)
        .into()
}
