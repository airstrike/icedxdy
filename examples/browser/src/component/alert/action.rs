use icedxdy::Component;
use icedxdy::Size;
use icedxdy::widget::{alert, button};

use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    alert::alert()
        .title("Dark mode is now available")
        .description("Enable it under your profile settings to get started.")
        .action(button::button("Enable").size(Size::Xs).on_press(Message::Pressed))
        .into()
}
