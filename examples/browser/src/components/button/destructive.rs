use icedxdy::Element;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Element<'a, Message> {
    button::destructive("Destructive").on_press(Message::Pressed).into()
}
