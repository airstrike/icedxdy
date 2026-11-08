//! Preview-02 roller-shades card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/roller-shades.tsx`.

use iced::Fill;
use iced::Widget;
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::{column, row, switch, text, toggle_group};
use icedxdy::{Space, Theme};

use crate::{Message, ShadePosition, State};

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let position = state.shade_position;
    let shades = toggle_group(|| POSITIONS.map(|(_, label)| toggle_group::item(label, label)))
        .value(
            POSITIONS
                .iter()
                .filter(|(option, _)| *option == position)
                .map(|(_, label)| *label),
        )
        .on_value_change(move |pressed| {
            let chosen = POSITIONS
                .iter()
                .find(|(_, label)| pressed.first().is_some_and(|pressed| pressed == label))
                .map_or(position, |(option, _)| *option);

            Message::SetShadePosition(chosen)
        })
        .style(toggle_group::Outline)
        .width(Fill);

    card::card(
        column![
            card::header("Living Room").description("Roller Shades"),
            card::content(
                column![
                    row![
                        column![
                            text::small("Position").style(theme::text::Muted),
                            text::p("65%").medium(),
                        ]
                        .spacing(Space::U1)
                        .width(Fill),
                        column![
                            text::small("Auto Schedule").style(theme::text::Muted),
                            switch().default_checked(true),
                        ]
                        .spacing(Space::U2),
                    ]
                    .spacing(Space::U4)
                    .width(Fill),
                    shades,
                ]
                .spacing(Space::U5)
                .width(Fill)
            ),
        ]
        .spacing(Space::U4)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}

const POSITIONS: [(ShadePosition, &str); 3] = [
    (ShadePosition::Open, "Open"),
    (ShadePosition::Half, "Half"),
    (ShadePosition::Closed, "Closed"),
];
