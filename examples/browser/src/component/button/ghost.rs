use icedxdy::Slot;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button::ghost("Ghost").on_press(Message::Pressed).into()
}
