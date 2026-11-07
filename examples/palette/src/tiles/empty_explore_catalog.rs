//! Preview-02 empty-explore-catalog card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/empty-explore-catalog.tsx`.

use iced::Fill;
use iced::Widget;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::widget::{button, column, empty};

use crate::{Message, State, icon};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let content = empty::empty()
        .padding(16)
        .media(empty::media_icon(icon::circle().size(16)))
        .title("Explore Catalog")
        .description(
            "Check your ISRC codes, metadata, and visual assets\n\
             before going live.",
        )
        .content(button("View Catalog").on_press(Message::Noop));

    card::card(column![card::content(content)].padding([16, 0]).width(Fill))
        .width(Fill)
        .boxed()
}
