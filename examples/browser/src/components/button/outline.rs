use icedxdy::Component;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    button::outline("Outline").on_press(Message::Pressed).into()
}
