use icedxdy::Component;
use icedxdy::widget::button;

use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    button::secondary("Secondary")
        .on_press(Message::Pressed)
        .into()
}
