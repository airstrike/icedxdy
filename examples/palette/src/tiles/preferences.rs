//! Preview-02 preferences card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/preferences.tsx`.

use iced::widget::space;
use iced::{Center, Fill};
use icedxdy::widget::{button, column, field, row, switch};
use icedxdy::{Space, Theme, card};

use super::{api_gap, select};
use crate::{Message, State};

const CURRENCIES: &[&str] = &[
    "USD — United States Dollar",
    "EUR — Euro",
    "GBP — British Pound",
    "JPY — Japanese Yen",
];

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let currency = field::field(select::trigger("USD — United States Dollar", CURRENCIES))
        .label("Default Currency")
        .width(Fill);

    let public = field::field(
        switch(state.preferences_public_stats).on_toggle(Message::PreferencesPublicStatsChanged),
    )
    .orientation(field::Horizontal)
    .label("Public Statistics")
    .description("Allow others to see your total stream count and listening activity");

    let email = field::field(
        switch(state.preferences_email_notifications)
            .on_toggle(Message::PreferencesEmailNotificationsChanged),
    )
    .orientation(field::Horizontal)
    .label("Email Notifications")
    .description("Monthly royalty reports and distribution updates");

    card::card(
        column![
            card::header("Preferences")
                .description("Manage your account settings and notifications.")
                .action(
                    row![
                        api_gap::marker(),
                        button::ghost("x").on_press(Message::Noop),
                    ]
                    .spacing(Space::U2)
                ),
            card::content(column![currency, public, email].spacing(16).width(Fill)),
            card::footer(
                row![
                    button::outline("Reset").on_press(Message::Noop),
                    space::horizontal(),
                    button("Save Preferences").on_press(Message::Noop),
                ]
                .align_y(Center)
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
