//! Preview 01 claimable-balance card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/claimable-balance.tsx`.

use iced::widget::space;
use iced::{Center, Fill};
use icedxdy::card;
use icedxdy::core::{Border, oklch};
use icedxdy::theme::container::Style;
use icedxdy::widget::{badge, column, container, item, row, rule, text};
use icedxdy::{Component, Theme, Vertical};

use crate::{Message, State};

#[granita::preview(state = State::dark(), group = "preview-01")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let pending = badge::outline(
        row![dot(), text::small("Pending Setup")]
            .spacing(6)
            .align_y(Center),
    );

    let header_body: Component<'static, Message> = column![
        card::description("Claimable Balance"),
        text::p("$0.00").size(48.0).semibold(),
        pending,
    ]
    .spacing(4)
    .width(Fill)
    .into();
    let header: Component<'static, Message> = card::header(header_body).into();

    let totals = item::muted([item::content([
        money_row("Net Royalties", "$0.00", false),
        money_row("Processing Fee", "-$0.00", false),
        divider(),
        money_row("Total Ready to Claim", "$0.00 USD", true),
    ])
    .spacing(12.0)
    .into()])
    .orientation(Vertical);

    card::card(
        column![
            header,
            card::content(totals),
            card::footer(
                card::description(
                    "Once your bank is connected, balances over\n\
                     $10.00 are automatically eligible for monthly\n\
                     distribution on the 15th of each month.",
                )
                .width(Fill),
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .into()
}

fn money_row(
    label: &'static str,
    amount: &'static str,
    strong: bool,
) -> Component<'static, Message> {
    let amount = if strong {
        text::small(amount).semibold()
    } else {
        text::small(amount).medium()
    };

    row![text::muted(label).size(16.0), space::horizontal(), amount,]
        .align_y(Center)
        .width(Fill)
        .into()
}

fn divider() -> Component<'static, Message> {
    let line: iced::Element<'static, Message, Theme> =
        rule::horizontal(1).style(divider_style()).into();

    container(line).height(1.0).width(Fill).into()
}

fn dot() -> Component<'static, Message> {
    container(space::vertical())
        .width(10.0)
        .height(10.0)
        .style(dot_style)
        .into()
}

fn dot_style(_theme: &Theme) -> Style {
    Style {
        background: Some(oklch(0.795, 0.184, 86.047).into()),
        border: Border {
            radius: 999.0.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

fn divider_style() -> rule::StyleFn<'static, Theme> {
    Box::new(|theme| rule::Style {
        background: iced::Background::Color(theme.neutral.border.into()),
        ..rule::Style::default()
    })
}
