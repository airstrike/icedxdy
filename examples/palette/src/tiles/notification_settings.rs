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
    checked: bool,
}

const NOTIFICATIONS: &[Notification] = &[
    Notification {
        id: "transactions",
        label: "Transaction alerts",
        description: "Deposits, withdrawals, and transfers.",
        checked: true,
    },
    Notification {
        id: "security",
        label: "Security alerts",
        description: "Login attempts and account changes.",
        checked: true,
    },
    Notification {
        id: "goals",
        label: "Goal milestones",
        description: "Updates at 25%, 50%, 75%, and 100%.",
        checked: false,
    },
    Notification {
        id: "market",
        label: "Market updates",
        description: "Daily portfolio summary and price alerts.",
        checked: false,
    },
];

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let select_all = field::field(
        checkbox::checkbox(checkbox::State::Indeterminate).on_toggle(|_| Message::Noop),
    )
    .orientation(field::Horizontal)
    .label("Select all");

    let mut fields = vec![select_all.into()];
    fields.extend(NOTIFICATIONS.iter().map(notification_field));

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

fn notification_field(n: &Notification) -> icedxdy::Component<'static, Message> {
    let id = n.id;
    field::field(
        checkbox::checkbox(n.checked)
            .on_toggle(move |_| Message::ToggleNotification(id.to_owned())),
    )
    .orientation(field::Horizontal)
    .label(n.label)
    .description(n.description)
    .into()
}
