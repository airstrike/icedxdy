//! Preview-02 account-access card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/account-access.tsx`.

use iced::Fill;
use icedxdy::widget::{button, column, field, input, item};
use icedxdy::{Theme, card};

use super::api_gap;
use crate::{Message, State, icon};

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let email = field::field(
        input("", &state.account_email)
            .on_input(Message::AccountEmailChanged)
            .width(Fill),
    )
    .label("Email Address")
    .width(Fill);

    let password = field::field(
        input("", &state.account_password)
            .on_input(Message::AccountPasswordChanged)
            .width(Fill),
    )
    .label("Current Password")
    .width(Fill);

    let danger = item::muted([
        item::media_icon(icon::circle_help().size(16)).into(),
        item::content([
            item::title("Danger Zone").into(),
            item::description("Archive account and remove catalog").into(),
        ])
        .into(),
        icon::arrow_right().size(16).into(),
    ]);

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
    .into()
}
