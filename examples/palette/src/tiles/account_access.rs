//! Preview-02 account-access card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/account-access.tsx`.

use iced::Fill;
use iced::Widget;
use icedxdy::widget::{button, column, field, input, item};
use icedxdy::{Theme, card};

use super::api_gap;
use crate::{Message, icon};

pub fn view() -> iced::Element<'static, Message, Theme> {
    let email = field::field(input("").default_value("artist@studio.inc").width(Fill))
        .label("Email Address")
        .width(Fill);

    let password = field::field(input("").default_value("***********").width(Fill))
        .label("Current Password")
        .width(Fill);

    let danger = item::item([
        item::media(icon::circle_help().size(16))
            .style(item::Icon)
            .into(),
        item::content([
            item::title("Danger Zone").into(),
            item::description("Archive account and remove catalog").into(),
        ])
        .into(),
        icon::arrow_right().size(16).into(),
    ])
    .style(item::Muted);

    card::card(
        column![
            card::header("Account Access")
                .description("Update your credentials or re-authenticate.")
                .action(api_gap::marker()),
            card::content(column![email, password].spacing(16).width(Fill)),
            card::footer(
                column![
                    button("Update Security")
                        .width(Fill)
                        .on_press(Message::Noop),
                    danger,
                ]
                .spacing(16)
                .width(Fill)
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}
