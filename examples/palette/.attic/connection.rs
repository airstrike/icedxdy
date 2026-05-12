//! Connection-status tile — switches between a success and a destructive
//! alert based on `state.connected`.
//!
//! Exercises: alert (default + destructive), button (link / outline),
//! switch (echo of the header toggle), text (xs / muted), container, row.

use iced::Fill;
use icedxdy::Component;
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::{alert, button, column, row, space, switch, text};
use icedxdy::Theme;

use crate::{Message, State};

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let alert_body = if state.connected {
        alert()
            .title("All operations enabled")
            .description("Buttons across the dashboard will dispatch their on_press messages.")
            .action(button::link("Manage").on_press(Message::Noop))
    } else {
        alert::destructive()
            .title("Read-only mode")
            .description("Reconnect to re-enable actions. Buttons render as disabled until then.")
            .action(button::outline("Reconnect").on_press(Message::ToggleConnection(true)))
    };

    let mirror_row = row![
        text::xs("Live mirror of the header toggle:").style(theme::text::Muted),
        space::horizontal(),
        switch::switch(state.connected)
            .label(if state.connected { "online" } else { "offline" })
            .on_toggle(Message::ToggleConnection),
    ]
    .spacing(8)
    .width(Fill);

    let body = column([
        Component::from(alert_body),
        Component::from(mirror_row),
    ])
    .spacing(16);

    card::card(
        column([
            card::header("Connection")
                .description("Toggle to flip the dashboard between live and read-only.")
                .into(),
            card::content(body).into(),
        ])
        .spacing(16)
        .padding([16, 0]),
    )
    .width(Fill)
    .into()
}
