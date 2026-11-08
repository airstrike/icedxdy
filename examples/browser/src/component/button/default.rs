use icedxdy::Slot;
use icedxdy::widget::button;

use crate::Message;

pub fn view<'a>() -> Slot<'a, Message> {
    button!["Button"].into()
}
