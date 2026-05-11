//! Transactions tile — list rows with an avatar fallback, merchant name,
//! category badge, amount (success/destructive tinted), and a muted
//! timestamp. The body sits inside a fixed-height scrollable so the row
//! list scrolls inside its card.
//!
//! Exercises: avatar (no-image fallback), badge (default + secondary +
//! outline), separator (horizontal between rows), scrollable (inner
//! card-bounded list), button (ghost), text variants (p / xs / small),
//! rule (vertical thin), container, row, column.

use iced::widget::space;
use iced::{Center, Fill};
use icedxdy::Component;
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::{
    Column, avatar, badge, button, column, container, row, rule, scrollable, separator, text,
};
use icedxdy::Theme;

use crate::{Message, State};

/// One row in the transactions list. Static for the demo — a real app
/// would source these from state.
struct Tx {
    initials: &'static str,
    merchant: &'static str,
    category: &'static str,
    /// Category visual rendered as one of three badge variants.
    badge_kind: BadgeKind,
    /// Amount in cents. Positive = income (success tint), negative = outflow
    /// (destructive tint).
    amount_cents: i64,
    when: &'static str,
}

#[derive(Clone, Copy)]
enum BadgeKind {
    Default,
    Secondary,
    Outline,
}

const ROWS: &[Tx] = &[
    Tx {
        initials: "AL",
        merchant: "Ada Loveline",
        category: "Salary",
        badge_kind: BadgeKind::Default,
        amount_cents: 412_500,
        when: "Today \u{2022} 09:12",
    },
    Tx {
        initials: "GH",
        merchant: "GitHub Sponsors",
        category: "Income",
        badge_kind: BadgeKind::Default,
        amount_cents: 9_800,
        when: "Today \u{2022} 08:04",
    },
    Tx {
        initials: "FW",
        merchant: "Foundry Works",
        category: "Studio",
        badge_kind: BadgeKind::Outline,
        amount_cents: -28_400,
        when: "Yesterday",
    },
    Tx {
        initials: "BC",
        merchant: "Blue Bottle Coffee",
        category: "Food",
        badge_kind: BadgeKind::Secondary,
        amount_cents: -725,
        when: "Yesterday",
    },
    Tx {
        initials: "AW",
        merchant: "AWS",
        category: "Infrastructure",
        badge_kind: BadgeKind::Outline,
        amount_cents: -13_422,
        when: "2d ago",
    },
    Tx {
        initials: "MK",
        merchant: "MK Bookstore",
        category: "Personal",
        badge_kind: BadgeKind::Secondary,
        amount_cents: -4_299,
        when: "3d ago",
    },
];

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let header = card::header("Recent activity")
        .description("Filtered view: most-recent entries.")
        .action(
            if state.connected {
                button::ghost("View all").on_press(Message::Noop)
            } else {
                button::ghost("View all")
            },
        );

    let rows: Vec<Component<'_, Message>> = ROWS
        .iter()
        .enumerate()
        .flat_map(|(i, tx)| {
            let mut items: Vec<Component<'_, Message>> = Vec::new();
            if i > 0 {
                items.push(Component::from(iced::Element::from(separator::horizontal())));
            }
            items.push(tx_row(tx));
            items
        })
        .collect();

    // Cap the scrollable height so the card stays a fixed visual size
    // regardless of row count — wheel events bubble into the inner list.
    let inner: Column<'_, Message> = Column::with_children(rows).spacing(0).width(Fill);
    let list = scrollable(container(inner).padding([8, 16]))
        .width(Fill)
        .height(240);

    card::card(
        column([header.into(), card::content(list).into()])
            .spacing(16)
            .padding([16, 0]),
    )
    .width(Fill)
    .into()
}

fn tx_row(tx: &Tx) -> Component<'_, Message> {
    let avatar_chip = avatar(None).fallback(tx.initials).size(avatar::Default);

    let badge_chip: Component<'_, Message> = match tx.badge_kind {
        BadgeKind::Default => badge(tx.category).into(),
        BadgeKind::Secondary => badge::secondary(tx.category).into(),
        BadgeKind::Outline => badge::outline(tx.category).into(),
    };

    let merchant_col: Column<'_, Message> = column([
        text::p(tx.merchant).into(),
        text::xs(tx.when).style(theme::text::Muted).into(),
    ])
    .spacing(2);

    let amount = format_amount(tx.amount_cents);
    let pos = tx.amount_cents >= 0;
    let amount_style: iced_widget::text::StyleFn<'static, icedxdy::Theme> =
        Box::new(move |t: &icedxdy::Theme| iced_widget::text::Style {
            color: Some(if pos { t.success.base.into() } else { t.destructive.base.into() }),
        });
    let amount_text = text::small(amount).medium().style(amount_style);

    // 1px thin divider after the avatar — exercises rule::vertical
    // alongside the separator::horizontal between rows. Wrap the
    // Fill-height rule in a fixed-height container so the row's
    // shrink height doesn't collapse it to zero.
    let divider: iced::Element<'_, Message, icedxdy::Theme> = rule::vertical(1).into();

    let inner = row![
        Component::from(avatar_chip),
        Component::from(container(divider).height(28)),
        merchant_col,
        badge_chip,
        space::horizontal(),
        amount_text,
    ]
    .spacing(12)
    .align_y(Center)
    .width(Fill);

    container(inner).padding([8, 0]).width(Fill).into()
}

fn format_amount(cents: i64) -> String {
    let sign = if cents >= 0 { "+" } else { "-" };
    let abs = cents.unsigned_abs();
    let dollars = abs / 100;
    let cents = abs % 100;
    format!("{sign}${dollars}.{cents:02}")
}
