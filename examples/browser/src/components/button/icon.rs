use icedxdy::Element;
use icedxdy::widget::button;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Element<'a, Message> {
    button::icon(icon::arrow_up_right())
        .on_press(Message::Pressed)
        .into()
}
