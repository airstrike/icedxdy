//! Preview-02 empty-connect-bank card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/empty-connect-bank.tsx`.

use iced::Fill;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::widget::{button, column, empty};

use crate::{Message, State, icon};

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let content = empty::empty()
        .padding(16)
        .media(empty::media_icon(icon::square().size(16)))
        .title("Connect Bank")
        .description(
            "Link your payout method to receive monthly royalty\n\
             distributions automatically.",
        )
        .content(button("Set Up Payouts").on_press(Message::Noop));

    card::card(column![card::content(content)].padding([16, 0]).width(Fill))
        .width(Fill)
        .into()
}
