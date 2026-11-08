use icedxdy::Slot;
use icedxdy::widget::button;

use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button("Ghost").style(button::Ghost).into()
}
