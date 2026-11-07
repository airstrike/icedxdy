//! Environment variables — pinned KEY chip + value, with Edit/Deploy footer.

use iced::Fill;
use iced::Widget;
use iced::widget::space;
use icedxdy::card;
use icedxdy::widget::{button, column, container, input, row};
use icedxdy::{Component, Theme};

use crate::{Message, State};

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let rows: Vec<Component<'_, Message>> = vec![
        env_row(
            "DATABASE_URL",
            &state.env_database,
            Message::EnvDatabaseChanged,
        ),
        env_row("NEXT_PUBLIC_API", &state.env_api, Message::EnvApiChanged),
        env_row(
            "STRIPE_SECRET",
            &state.env_stripe,
            Message::EnvStripeChanged,
        ),
    ];

    let footer = row![
        button::outline("Edit").on_press(Message::Noop),
        space::horizontal(),
        button::secondary("Deploy").on_press(Message::Noop),
    ]
    .spacing(8)
    .width(Fill);

    let body = column(rows.into_iter().chain([footer.into()]))
        .spacing(10)
        .width(Fill);

    card::card(
        column([
            card::header("Environment Variables")
                .description("Production \u{00b7} 8 variables")
                .into(),
            card::content(container(body).padding([0, 16]).width(Fill)).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .height(273.0)
    .boxed()
}

fn env_row<'a>(
    key: &'static str,
    value: &'a str,
    on_input: fn(String) -> Message,
) -> Component<'a, Message> {
    row![readonly_input(key), active_input(value, on_input),]
        .spacing(8)
        .width(Fill)
        .into()
}

fn readonly_input(value: &'static str) -> Component<'static, Message> {
    input("", value).width(Fill).into()
}

fn active_input<'a>(value: &'a str, on_input: fn(String) -> Message) -> Component<'a, Message> {
    input("", value)
        .width(Fill)
        .on_input(on_input)
        .on_submit(Message::Noop)
        .into()
}
