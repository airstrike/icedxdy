use iced::Element;
use icedxdy::Theme;
use icedxdy::core::style::{destructive, outline};
use icedxdy::widget::{Column, button, text};

use crate::Message;

pub fn view(armed: bool) -> Element<'static, Message, Theme> {
    let btn = if armed {
        button("Confirm Delete")
            .width(160)
            .style(destructive)
            .on_press(Message::ToggleArmed)
    } else {
        button("Delete")
            .width(160)
            .style(outline + destructive)
            .on_press(Message::ToggleArmed)
    };

    let body: Column<'static, Message> = Column::new()
        .push(text::label("Click to arm. Click again to fire.").into())
        .push(btn.into())
        .spacing(10);
    body.into()
}
