use iced::Element;
use icedxdy::Theme;
use icedxdy::core::Palette;
use icedxdy::core::style::{Status, Style, StyleFn, destructive, outline};
use icedxdy::theme;
use icedxdy::widget::{Column, button, text};

use crate::Message;

pub fn view(armed: bool) -> Element<'static, Message, Theme> {
    let btn = if armed {
        let style: StyleFn<fn(Palette, Status) -> Style> = theme::button::Default.into();
        button("Confirm Delete")
            .width(160)
            .style(style + destructive)
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
