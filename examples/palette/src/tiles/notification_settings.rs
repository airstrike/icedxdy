//! Preview-02 notification-settings card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/notification-settings.tsx`.

use iced::Fill;
use icedxdy::widget::checkbox;
use icedxdy::widget::{button, column, field};
use icedxdy::{Theme, card};

use crate::{Message, State};

struct Notification {
    id: &'static str,
    label: &'static str,
    description: &'static str,
}

const NOTIFICATIONS: &[Notification] = &[
    Notification {
        id: "transactions",
        label: "Transaction alerts",
        description: "Deposits, withdrawals, and transfers.",
    },
    Notification {
        id: "security",
        label: "Security alerts",
        description: "Login attempts and account changes.",
    },
    Notification {
        id: "goals",
        label: "Goal milestones",
        description: "Updates at 25%, 50%, 75%, and 100%.",
    },
    Notification {
        id: "market",
        label: "Market updates",
        description: "Daily portfolio summary and price alerts.",
    },
];

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let select_all_state = match state.notification_settings.len() {
        0 => checkbox::State::Unchecked,
        n if n == NOTIFICATIONS.len() => checkbox::State::Checked,
        _ => checkbox::State::Indeterminate,
    };

    let select_all = field::field(
        checkbox::checkbox(select_all_state).on_toggle(Message::SetAllNotificationSettings),
    )
    .orientation(field::Horizontal)
    .label("Select all");

    let mut fields = vec![select_all.into()];
    fields.extend(NOTIFICATIONS.iter().map(|n| notification_field(state, n)));

    card::card(
        column![
            card::header("Notifications").description("Choose what you want to be notified about."),
            card::content(column(fields).spacing(14).width(Fill)),
            card::footer(
                button("Save Preferences")
                    .width(Fill)
                    .on_press(Message::Noop)
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .into()
}

fn notification_field(state: &State, n: &Notification) -> icedxdy::Component<'static, Message> {
    let id = n.id;
    field::field(
        checkbox::checkbox(state.notification_settings.contains(id))
            .on_toggle(move |_| Message::ToggleNotificationSetting(id.to_owned())),
    )
    .orientation(field::Horizontal)
    .label(n.label)
    .description(n.description)
    .into()
}
