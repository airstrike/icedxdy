//! Environment variables — pinned KEY chip + value, with Edit/Deploy footer.

use iced::Fill;
use iced::widget::space;
use icedxdy::card;
use icedxdy::widget::{button, column, container, input, row};
use icedxdy::{Component, Theme};

use crate::{Message, State};

struct EnvVar {
    key: &'static str,
    value: &'static str,
}

const DOTS: &str = "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}";

const ENV: &[EnvVar] = &[
    EnvVar {
        key: "DATABASE_URL",
        value: DOTS,
    },
    EnvVar {
        key: "NEXT_PUBLIC_API",
        value: "https://api.example.com",
    },
    EnvVar {
        key: "STRIPE_SECRET",
        value: DOTS,
    },
];

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let rows: Vec<Component<'static, Message>> = ENV.iter().map(env_row).collect();

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
    .into()
}

fn env_row(v: &EnvVar) -> Component<'static, Message> {
    row![active_input(v.key), active_input(v.value),]
        .spacing(8)
        .width(Fill)
        .into()
}

fn active_input(value: &'static str) -> Component<'static, Message> {
    input("", value)
        .width(Fill)
        .on_input(|_| Message::Noop)
        .on_submit(Message::Noop)
        .into()
}
