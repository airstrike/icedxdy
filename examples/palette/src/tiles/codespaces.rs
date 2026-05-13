//! Codespaces — empty-state card with a centered CTA.

use iced::{Center, Fill};
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::{button, column, container, text};
use icedxdy::Theme;

use crate::{Message, State};

const EMPTY_HEIGHT: f32 = 240.0;

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let empty = column([
        text::large("No codespaces yet").into(),
        text::muted("Spin up a codespace to start coding in the cloud.")
            .style(theme::text::Muted)
            .into(),
        button("Create Codespace").on_press(Message::Noop).into(),
    ])
    .spacing(12)
    .align_x(Center)
    .width(Fill);

    let inner = container(empty)
        .height(EMPTY_HEIGHT)
        .center_x(Fill)
        .center_y(Fill);

    card::card(
        column([
            card::header("Codespaces")
                .description("Cloud dev environments.")
                .into(),
            card::content(inner).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .into()
}
