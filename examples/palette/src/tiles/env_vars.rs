//! Environment variables — pinned KEY chip + value, with Edit/Deploy footer.

use iced::Fill;
use iced::Widget;
use iced::widget::space;
use icedxdy::card;
use icedxdy::widget::{button, column, container, input, row};
use icedxdy::{Slot, Theme};

use crate::Message;

pub fn view() -> iced::Element<'static, Message, Theme> {
    let rows: Vec<Slot<'_, Message>> = vec![
        env_row(
            "DATABASE_URL",
            "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}",
        ),
        env_row("NEXT_PUBLIC_API", "https://api.example.com"),
        env_row(
            "STRIPE_SECRET",
            "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}",
        ),
    ];

    let footer = row![
        button("Edit")
            .style(button::Outline)
            .on_press(Message::Noop),
        space::horizontal(),
        button("Deploy")
            .style(button::Secondary)
            .on_press(Message::Noop),
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

fn env_row<'a>(key: &'static str, value: &'a str) -> Slot<'a, Message> {
    row![readonly_input(key), active_input(value)]
        .spacing(8)
        .width(Fill)
        .into()
}

fn readonly_input(value: &'static str) -> Slot<'static, Message> {
    input("").value(value).width(Fill).into()
}

fn active_input<'a>(value: &'a str) -> Slot<'a, Message> {
    input("")
        .default_value(value)
        .width(Fill)
        .on_submit(Message::Noop)
        .into()
}
