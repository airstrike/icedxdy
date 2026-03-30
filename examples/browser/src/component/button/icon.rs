use icedxdy::Component;
use icedxdy::widget::button;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    button::icon(icon::arrow_up_right())
        .on_press(Message::Pressed)
        .into()
}
