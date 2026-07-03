//! Preview-02 social-links card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/social-links.tsx`.

use iced::Fill;
use icedxdy::widget::{button, column, field, input, row, space};
use icedxdy::{Theme, card};

use super::api_gap;
use crate::{Message, State};

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    card::card(
        column![
            card::header("Social Links").action(api_gap::marker()),
            card::content(
                column![
                    link_field(
                        "Spotify Artist URL",
                        &state.social_spotify,
                        Message::SocialSpotifyChanged
                    ),
                    link_field(
                        "Instagram Handle",
                        &state.social_instagram,
                        Message::SocialInstagramChanged
                    ),
                    link_field(
                        "SoundCloud URL",
                        &state.social_soundcloud,
                        Message::SocialSoundCloudChanged
                    ),
                    link_field(
                        "Website",
                        &state.social_website,
                        Message::SocialWebsiteChanged
                    ),
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

fn link_field<'a>(
    label: &'static str,
    value: &'a str,
    on_input: fn(String) -> Message,
) -> icedxdy::Component<'a, Message> {
    field::field(input("", value).on_input(on_input).width(Fill))
        .label(label)
        .width(Fill)
        .into()
}
