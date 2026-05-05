use icedxdy::Component;
use icedxdy::widget::button;

use super::Message;
use crate::icon;

pub fn view<'a>() -> Component<'a, Message> {
    button::icon![arrow_up]
        .rounded()
        .on_press(Message::Pressed)
        .into()
}
