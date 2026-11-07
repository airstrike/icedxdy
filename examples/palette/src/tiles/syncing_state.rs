//! Preview-02 syncing-state card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/syncing-state.tsx`.

use iced::Fill;
use iced::Widget;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::widget::{button, column, empty, spinner};

use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let content = empty::empty()
        .padding(16)
        .media(empty::media_icon(spinner().size(16.0)))
        .title("Syncing your accounts")
        .description(
            "We're pulling in your latest transactions. This usually takes\n\
             a few seconds.",
        )
        .content(
            button("Cancel")
                .style(button::Outline)
                .on_press(Message::Noop),
        );

    card::card(column![content].padding([16, 0]).width(Fill))
        .width(Fill)
        .boxed()
}
