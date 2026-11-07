//! Preview-02 savings-targets card pair.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/savings-targets.tsx`.

use iced::widget::space;
use iced::{Center, Fill};
use icedxdy::theme;
use icedxdy::widget::{button, column, container, field, input, item, progress, row, text};
use icedxdy::{Slot, Theme, Vertical, card};

use super::{api_gap, select};
use crate::{Message, State};

const ORDER_TYPES: &[&str] = &["Market Order", "Limit Order", "Stop Order"];

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    row![targets_card(), buy_card(state)]
        .spacing(40)
        .align_y(Center)
        .width(Fill)
        .into()
}

fn targets_card() -> Slot<'static, Message> {
    card::card(
        column![
            card::header("Savings Targets")
                .description("Active milestones for 2024")
                .action(button::outline("New Goal").on_press(Message::Noop)),
            card::content(
                column![
                    target("RETIREMENT", "$420,000", 65.0, "65% achieved", "$273,000"),
                    target("REAL ESTATE", "$85,000", 32.0, "32% achieved", "$27,200"),
                ]
                .spacing(12)
                .width(Fill)
            ),
            card::footer(
                container(text::muted("You have not met your targets for this year."))
                    .center_x(Fill)
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .into()
}

fn buy_card(state: &State) -> Slot<'_, Message> {
    let amount = field::field(
        input("")
            .value(&state.buy_investment_amount)
            .on_input(Message::BuyInvestmentAmountChanged)
            .width(Fill),
    )
    .label("Amount to Invest")
    .width(Fill);

    let order = field::field(select::trigger("Market Order", ORDER_TYPES))
        .label("Order Type")
        .description("Market orders execute at the current price.")
        .width(Fill);

    card::card(
        column![
            card::header("Buy Investment").action(api_gap::marker()),
            card::content(field::group([
                amount.into(),
                order.into(),
                column![
                    info_row("Estimated Shares", "1.95"),
                    info_row("Buying Power", "$12,450.00"),
                ]
                .spacing(8)
                .width(Fill)
                .into(),
            ])),
            card::footer(
                column![
                    button("Review Order").width(Fill).on_press(Message::Noop),
                    container(text::muted(
                        "Trades are typically executed within minutes during market hours.",
                    ))
                    .center_x(Fill),
                ]
                .spacing(12)
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

fn target(
    label: &'static str,
    total: &'static str,
    value: f32,
    status: &'static str,
    amount: &'static str,
) -> Slot<'static, Message> {
    item::muted([
        item::content([
            text::xs(label)
                .medium()
                .letter_spacing(0.08)
                .style(theme::text::Muted)
                .into(),
            text::p(total).size(30.0).semibold().into(),
            progress(value).width(Fill).into(),
        ])
        .spacing(12.0)
        .into(),
        item::footer([
            text::muted(status).into(),
            text::small(amount).medium().into(),
        ]),
    ])
    .orientation(Vertical)
    .into()
}

fn info_row(label: &'static str, value: &'static str) -> Slot<'static, Message> {
    row![
        text::muted(label),
        space::horizontal(),
        text::small(value).semibold()
    ]
    .align_y(Center)
    .width(Fill)
    .into()
}
