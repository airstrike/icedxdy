//! Observability-Plus-style hero — aspect-ratio thumbnail + text + CTA.

use iced::Fill;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::theme::container::card_muted;
use icedxdy::widget::{aspect_ratio, button, column, container, row, text};

use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let preview = aspect_ratio(
        container(text::muted("preview"))
            .center_x(Fill)
            .center_y(Fill)
            .style(card_muted),
        16.0 / 9.0,
    )
    .max_width(180.0);

    let copy = column![
        text::large("Observability +").width(Fill),
        text::p(
            "Replacing Monitoring across the org. Drop-in OTLP support, faster \
             queries, lower cost.",
        )
        .width(Fill),
        button("Learn more").on_press(Message::Noop),
    ]
    .spacing(8)
    .width(Fill);

    let body = row![preview, copy].spacing(16).width(Fill);

    card::card(container(body).padding(16)).width(Fill).into()
}
