use icedxdy::Slot;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button("Secondary")
        .style(button::Secondary)
        .on_press(Message::Pressed)
        .into()
}
