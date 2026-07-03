//! Preview-02 recent-transactions card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/recent-transactions.tsx`.

use iced::{Center, Fill, Right};
use icedxdy::card;
use icedxdy::core::Border;
use icedxdy::padding::*;
use icedxdy::theme::container::Style;
use icedxdy::widget::{Component, button, column, container, row, table, text};
use icedxdy::{Gap, Size, Theme, radius};

use crate::{Message, State, icon};

const TRANSACTIONS: &[Transaction] = &[
    Transaction {
        merchant: "Blue Bottle Coffee",
        category: "Food & Drink",
        date: "Today, 10:24 AM",
        amount: "-$6.40",
        icon: Kind::Coffee,
    },
    Transaction {
        merchant: "Whole Foods Market",
        category: "Groceries",
        date: "Yesterday",
        amount: "-$84.32",
        icon: Kind::Groceries,
    },
    Transaction {
        merchant: "Stripe Payout",
        category: "Income",
        date: "Oct 12",
        amount: "+$2,500.00",
        icon: Kind::Payout,
    },
    Transaction {
        merchant: "Uber Technologies",
        category: "Transport",
        date: "Oct 11",
        amount: "-$24.80",
        icon: Kind::Transport,
    },
    Transaction {
        merchant: "Netflix Subscription",
        category: "Entertainment",
        date: "Oct 10",
        amount: "-$15.99",
        icon: Kind::Entertainment,
    },
];

const WIDTH: f32 = 800.0;

#[derive(Debug, Clone, Copy)]
struct Transaction {
    merchant: &'static str,
    category: &'static str,
    date: &'static str,
    amount: &'static str,
    icon: Kind,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Coffee,
    Entertainment,
    Groceries,
    Payout,
    Transport,
}

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    card::card(
        column![
            card::header("Recent Transactions")
                .description("Your latest account activity.")
                .action(
                    button::outline("View All")
                        .size(Size::Sm)
                        .on_press(Message::Noop)
                ),
            card::content(transactions_table())
        ]
        .spacing(Gap::G4)
        .padding(Y4)
        .width(Fill),
    )
    .width(Fill)
    .max_width(WIDTH)
    .into()
}

fn transactions_table() -> Component<'static, Message> {
    table([table::body(TRANSACTIONS.iter().map(transaction_row))])
        .width(Fill)
        .into()
}

fn transaction_row(transaction: &Transaction) -> table::Row<'static, Message> {
    table::row([
        table::cell(transaction_identity(transaction)).width(Fill),
        table::cell(text::muted(transaction.date))
            .width(128.0)
            .align_x(Right),
        table::cell(text::small(transaction.amount).medium())
            .width(96.0)
            .align_x(Right),
        table::cell(
            button::ghost(icon::ellipsis().size(16))
                .size(Size::Icon)
                .square()
                .on_press(Message::Noop),
        )
        .width(48.0),
    ])
}

fn transaction_identity(transaction: &Transaction) -> Component<'static, Message> {
    row![
        transaction_icon(transaction.icon),
        column![
            text::small(transaction.merchant).medium(),
            text::muted(transaction.category),
        ]
        .spacing(1)
        .width(Fill)
    ]
    .spacing(12)
    .align_y(Center)
    .width(Fill)
    .into()
}

fn transaction_icon(kind: Kind) -> Component<'static, Message> {
    let icon = match kind {
        Kind::Coffee => icon::coffee().size(16),
        Kind::Entertainment => icon::tv().size(16),
        Kind::Groceries => icon::shopping_cart().size(16),
        Kind::Payout => icon::wallet().size(16),
        Kind::Transport => icon::car().size(16),
    };

    container(icon)
        .center(32.0)
        .style(transaction_icon_style)
        .into()
}

fn transaction_icon_style(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.into()),
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}
