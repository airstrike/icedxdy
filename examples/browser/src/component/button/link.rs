use icedxdy::Slot;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button("Link")
        .style(button::Link)
        .on_press(Message::Pressed)
        .into()
}
