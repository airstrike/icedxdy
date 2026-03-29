use icedxdy::Component;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    button::destructive("Destructive").on_press(Message::Pressed).into()
}
