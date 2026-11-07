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
    let shades = toggle_group([
        shade_item(state, ShadePosition::Open, "Open"),
        shade_item(state, ShadePosition::Half, "Half"),
        shade_item(state, ShadePosition::Closed, "Closed"),
    ])
    .variant(toggle_group::Variant::Outline)
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
                            switch(state.shade_auto_schedule)
                                .on_toggle(Message::ToggleShadeSchedule),
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

fn shade_item(
    state: &State,
    position: ShadePosition,
    label: &'static str,
) -> toggle_group::Item<'static, Message> {
    toggle_group::item(label)
        .selected(state.shade_position == position)
        .on_press(Message::SetShadePosition(position))
}
