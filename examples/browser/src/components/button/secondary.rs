use icedxdy::Element;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Element<'a, Message> {
    button::secondary("Secondary").on_press(Message::Pressed).into()
}
