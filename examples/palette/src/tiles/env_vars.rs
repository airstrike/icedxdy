//! Environment variables form — paired KEY/value inputs and a Deploy CTA.

use iced::widget::space;
use iced::Fill;
use icedxdy::card;
use icedxdy::widget::{button, column, container, input, row};
use icedxdy::{Component, Theme};

use crate::{Message, State};

struct EnvVar {
    key: &'static str,
    value: &'static str,
}

const ENV: &[EnvVar] = &[
    EnvVar { key: "DATABASE_URL", value: "postgres://localhost:5432/icedxdy" },
    EnvVar { key: "NEXT_PUBLIC_API", value: "https://api.icedxdy.dev" },
    EnvVar { key: "STRIPE_KEY", value: "sk_live_\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}" },
    EnvVar { key: "FEATURE_FLAGS", value: "nova,radix" },
];

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let rows: Vec<Component<'static, Message>> = ENV.iter().map(env_row).collect();

    let footer = row![
        button::ghost("Save").on_press(Message::Noop),
        space::horizontal(),
        button("Deploy").on_press(Message::Noop),
    ]
    .spacing(8)
    .width(Fill);

    let body = column(rows.into_iter().chain([footer.into()]))
        .spacing(12)
        .width(Fill);

    card::card(
        column([
            card::header("Environment Variables")
                .description("Deploy-time keys for the runtime.")
                .into(),
            card::content(container(body).padding([0, 16])).into(),
        ])
        .spacing(16)
        .padding([16, 0]),
    )
    .width(Fill)
    .into()
}

fn env_row(v: &EnvVar) -> Component<'static, Message> {
    row![
        input("KEY", v.key).width(Fill),
        input("value", v.value).width(Fill),
    ]
    .spacing(8)
    .width(Fill)
    .into()
}
