//! Preview-02 social-links card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/social-links.tsx`.

use iced::Fill;
use icedxdy::widget::{button, column, field, input, row, space};
use icedxdy::{Theme, card};

use super::api_gap;
use crate::{Message, State};

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    card::card(
        column![
            card::header("Social Links").action(api_gap::marker()),
            card::content(
                column![
                    link_field("Spotify Artist URL", "spotify.com/artist/3j...2k"),
                    link_field("Instagram Handle", "@julianduryea_music"),
                    link_field("SoundCloud URL", "soundcloud.com/username"),
                    link_field("Website", "https://yoursite.com"),
                ]
                .spacing(16)
                .width(Fill)
            ),
            card::footer(
                row![
                    space::horizontal(),
                    button::secondary("Discard").on_press(Message::Noop),
                    button("Save Changes").on_press(Message::Noop),
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

fn link_field(label: &'static str, value: &'static str) -> icedxdy::Component<'static, Message> {
    field::field(input("", value).on_input(|_| Message::Noop).width(Fill))
        .label(label)
        .width(Fill)
        .into()
}
