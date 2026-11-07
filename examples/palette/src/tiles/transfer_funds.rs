//! Preview-02 transfer-funds card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/transfer-funds.tsx`.

use iced::Widget;
use iced::widget::space;
use iced::{Center, Fill};
use icedxdy::widget::{button, column, container, field, input, item, row, rule, text};
use icedxdy::{Component, Space, Theme, Vertical, card};

use super::{api_gap, select};
use crate::{Message, State};

const FROM_ACCOUNTS: &[&str] = &[
    "Main Checking (··8402) — $12,450.00",
    "Business (··7731) — $8,920.00",
];
const TO_ACCOUNTS: &[&str] = &[
    "High Yield Savings (··1192) — $42,100.00",
    "Investment (··3349) — $18,200.00",
];

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let amount = field::field(
        input("")
            .value(&state.transfer_amount)
            .on_input(Message::TransferAmountChanged)
            .width(Fill),
    )
    .label("Amount to Transfer")
    .width(Fill);

    let from = field::field(select::trigger(
        "Main Checking (··8402) — $12,450.00",
        FROM_ACCOUNTS,
    ))
    .label("From Account")
    .width(Fill);

    let to = field::field(select::trigger(
        "High Yield Savings (··1192) — $42,100.00",
        TO_ACCOUNTS,
    ))
    .label("To Account")
    .width(Fill);

    let summary = item::muted([item::content([
        money_row("Estimated arrival", "Today, Apr 14", false),
        divider(),
        money_row("Transaction fee", "$0.00", false),
        divider(),
        money_row("Total amount", &state.transfer_amount, true),
    ])
    .spacing(12.0)
    .into()])
    .orientation(Vertical);

    card::card(
        column![
            card::header("Transfer Funds")
                .description("Move money between your connected accounts.")
                .action(
                    row![
                        api_gap::marker(),
                        button::ghost("x").on_press(Message::Noop),
                    ]
                    .spacing(Space::U2)
                ),
            card::content(column![amount, from, to, summary].spacing(16).width(Fill)),
            card::footer(
                button("Confirm Transfer")
                    .width(Fill)
                    .on_press(Message::Noop)
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}

fn money_row<'a>(label: &'static str, value: &'a str, strong: bool) -> Component<'a, Message> {
    let value = if strong {
        text::small(value).semibold()
    } else {
        text::small(value).medium()
    };

    row![text::muted(label), space::horizontal(), value]
        .align_y(Center)
        .width(Fill)
        .into()
}

fn divider() -> Component<'static, Message> {
    let line: iced::Element<'static, Message, Theme> = rule::horizontal(1).into();
    container(line).height(1.0).width(Fill).into()
}
