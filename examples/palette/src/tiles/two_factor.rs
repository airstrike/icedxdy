//! Two-factor authentication tile — switch + descriptive copy.

use iced::Fill;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::{column, row, space, switch, text};

use crate::{Message, State};

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let label = column([
        text::small("Authenticator app").medium().into(),
        text::xs("Use a TOTP app like 1Password or Authy to generate codes.")
            .style(theme::text::Muted)
            .into(),
    ])
    .spacing(4)
    .width(Fill);

    let body = row![
        label,
        space::horizontal(),
        switch::switch(state.connected).on_toggle(Message::ToggleConnection),
    ]
    .spacing(16)
    .width(Fill);

    card::card(
        column([
            card::header("Two-factor authentication")
                .description(
                    "Add an extra layer of security to your account by enabling \
                     two-factor authentication.",
                )
                .into(),
            card::content(body).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .into()
}
