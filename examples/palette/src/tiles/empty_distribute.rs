//! Preview 01 empty-state card for the music distribution flow.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/empty-distribute-track.tsx`.

use iced::Fill;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::widget::{button, column, empty};

use crate::{Message, State, icon};

#[granita::preview(state = State::dark(), group = "preview-01")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let content = empty::empty()
        .padding(16)
        .media(empty::media_icon(icon::plus()))
        .title("Distribute Track")
        .description(
            "Upload your first master to start reaching\n\
                 listeners on Spotify, Apple Music, and more.",
        )
        .content(button("Create Release").on_press(Message::Noop));

    card::card(column![card::content(content)].padding([16, 0]).width(Fill))
        .width(Fill)
        .height(276.0)
        .into()
}
