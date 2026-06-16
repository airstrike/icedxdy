//! Preview-02 recent-transactions card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/recent-transactions.tsx`.

use iced::{Fill, Padding};
use icedxdy::card;
use icedxdy::widget::{button, column};
use icedxdy::{Size, Theme};

use super::api_gap;
use crate::{Message, State};

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
            card::content(api_gap::panel(&["Table", "DropdownMenu"], 296.0))
        ]
        .spacing(16)
        .padding(Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .into()
}
