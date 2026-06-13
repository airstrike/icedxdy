//! Preview-02 savings-targets card pair.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/savings-targets.tsx`.

use iced::widget::space;
use iced::{Center, Fill};
use icedxdy::theme;
use icedxdy::widget::{button, column, container, field, input, item, progress, row, text};
use icedxdy::{Component, Space, Theme, Vertical, card};

use super::{api_gap, select};
use crate::{Message, State};

const ORDER_TYPES: &[&str] = &["Market Order", "Limit Order", "Stop Order"];

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    row![targets_card(), buy_card()]
        .spacing(40)
        .align_y(Center)
        .width(Fill)
        .into()
}

fn targets_card() -> Component<'static, Message> {
    card::card(
        column![
            card::header("Savings Targets")
                .description("Active milestones for 2024")
                .action(
                    row![
                        api_gap::marker(),
                        button::outline("New Goal").on_press(Message::Noop),
                    ]
                    .spacing(Space::U2)
                ),
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
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .into()
}

fn buy_card() -> Component<'static, Message> {
    let amount = field::field(
        input("", "$ 1,000.00")
            .on_input(|_| Message::Noop)
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
            card::content(
                column![
                    amount,
                    order,
                    info_row("Estimated Shares", "1.95"),
                    info_row("Buying Power", "$12,450.00"),
                ]
                .spacing(16)
                .width(Fill)
            ),
            card::footer(
                column![
                    button("Review Order").width(Fill).on_press(Message::Noop),
                    container(text::muted(
                        "Trades are typically executed within minutes during market hours.",
                    ))
                    .center_x(Fill),
                ]
                .spacing(10)
                .width(Fill)
            ),
        ]
        .spacing(16)
        .padding([16, 0])
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
) -> Component<'static, Message> {
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

fn info_row(label: &'static str, value: &'static str) -> Component<'static, Message> {
    row![
        text::muted(label),
        space::horizontal(),
        text::small(value).semibold()
    ]
    .align_y(Center)
    .width(Fill)
    .into()
}
