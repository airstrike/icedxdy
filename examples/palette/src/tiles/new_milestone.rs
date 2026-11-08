//! Preview-02 new-milestone card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/new-milestone.tsx`.

use iced::Fill;
use iced::Widget;
use icedxdy::widget::{button, column, field, input, row};
use icedxdy::{Theme, card};

use crate::Message;

pub fn view() -> iced::Element<'static, Message, Theme> {
    let goal = field::field(input("e.g. New Car, Home Downpayment").width(Fill))
        .label("Goal Name")
        .width(Fill);

    let amount = field::field(input("").default_value("$15,000").width(Fill))
        .label("Target Amount")
        .width(Fill);

    let date = field::field(input("").default_value("Dec 2025").width(Fill))
        .label("Target Date")
        .width(Fill);

    card::card(
        column![
            card::header("Set a new milestone").description(
                "Define your financial target and we'll help you pace your\n\
                 savings.",
            ),
            card::content(
                column![goal, row![amount, date].spacing(12).width(Fill),]
                    .spacing(16)
                    .width(Fill)
            ),
            card::footer(
                column![
                    button("Create Goal").width(Fill).on_press(Message::Noop),
                    button("Cancel")
                        .style(button::Outline)
                        .width(Fill)
                        .on_press(Message::Noop),
                ]
                .spacing(8)
                .width(Fill)
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}
