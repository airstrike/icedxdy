//! Preview-02 loading-card card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/loading-card.tsx`.

use iced::Fill;
use icedxdy::card;
use icedxdy::widget::{Skeleton, column, row, skeleton};
use icedxdy::{Space, Theme};

use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let identity = row![
        skeleton().width(40.0).height(40.0),
        column![line(132.0, 16.0), line(196.0, 12.0),]
            .spacing(Space::U3)
            .width(Fill),
    ]
    .spacing(Space::U3)
    .width(Fill);

    let body = column![
        identity,
        line(310.0, 14.0),
        line(260.0, 14.0),
        row![line(108.0, 32.0), line(84.0, 32.0),].spacing(Space::U2),
    ]
    .spacing(Space::U4)
    .width(Fill);

    card::card(card::content(body))
        .width(Fill)
        .height(190.0)
        .into()
}

fn line(width: f32, height: f32) -> Skeleton<'static, Message> {
    skeleton().width(width).height(height)
}
