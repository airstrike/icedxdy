//! Preview-02 new-milestone card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/new-milestone.tsx`.

use iced::Fill;
use icedxdy::widget::{button, column, field, input, row};
use icedxdy::{Theme, card};

use crate::{Message, State};

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let goal = field::field(
        input("e.g. New Car, Home Downpayment", &state.new_milestone_goal)
            .on_input(Message::NewMilestoneGoalChanged)
            .width(Fill),
    )
    .label("Goal Name")
    .width(Fill);

    let amount = field::field(
        input("", &state.new_milestone_amount)
            .on_input(Message::NewMilestoneAmountChanged)
            .width(Fill),
    )
    .label("Target Amount")
    .width(Fill);

    let date = field::field(
        input("", &state.new_milestone_date)
            .on_input(Message::NewMilestoneDateChanged)
            .width(Fill),
    )
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
                    button::outline("Cancel")
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
    .into()
}
