use iced::Length;
use icedxdy::Component;
use icedxdy::widget::alert;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    icedxdy::widget::column([
        alert::alert()
            .icon(icon::circle_check_big())
            .title("Payment successful")
            .description("Your payment of $29.99 has been processed. A receipt has been sent to your email address.")
            .into(),
        alert::alert()
            .icon(icon::info())
            .title("New feature available")
            .description("We've added dark mode support. You can enable it in your account settings.")
            .into(),
    ])
    .spacing(16)
    .width(Length::Fill)
    .into()
}
