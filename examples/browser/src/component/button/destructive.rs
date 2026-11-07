use icedxdy::Slot;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button("Destructive")
        .style(button::Destructive)
        .on_press(Message::Pressed)
        .into()
}
