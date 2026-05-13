//! Notifications tile — accordion stack with a count badge per item and
//! a destructive dismiss action inside the open body.
//!
//! Exercises: accordion (multi-mode), badge (default + count), button
//! (destructive ghost), text variants (small / muted), row, column,
//! container.

use iced::Fill;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::core::style::{destructive, outline};
use icedxdy::theme;
use icedxdy::widget::{accordion, badge, button, column, container, row, space, text};

use crate::{Message, State};

struct Note {
    id: &'static str,
    title: &'static str,
    body: &'static str,
}

const NOTES: &[Note] = &[
    Note {
        id: "welcome",
        title: "Welcome to icedxdy",
        body: "Open any item to reveal its body, then dismiss to clear it. The accordion is in multi-mode so several can stay open at once.",
    },
    Note {
        id: "release",
        title: "v0.1 ships with the style API",
        body: "Every widget now resolves through the same composable .style(tint) pipeline. See .claude/docs/STYLE.md for the design.",
    },
    Note {
        id: "feedback",
        title: "Send feedback to tech@inboard.ai",
        body: "Bug reports and pixel-comparison screenshots welcome. We treat shadcn parity as load-bearing.",
    },
];

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let items: Vec<accordion::Item<'_, Message>> = NOTES
        .iter()
        .map(|note| {
            let body_col = column([
                text::small(note.body).style(theme::text::Muted).into(),
                row![
                    space::horizontal(),
                    {
                        let mut b = button("Dismiss")
                            .style(outline + destructive);
                        if state.connected {
                            b = b.on_press(Message::NotificationCleared);
                        }
                        b
                    },
                ]
                .spacing(8)
                .width(Fill)
                .into(),
            ])
            .spacing(8);

            let trigger = row![text::small(note.title).medium()]
                .spacing(8)
                .width(Fill);

            accordion::item(note.id.to_owned(), trigger).content(body_col)
        })
        .collect();

    let open_count = state.open_notifications.len();
    let header = card::header("Inbox")
        .description("Multi-mode accordion — open more than one at a time.")
        .action(badge(format!("{open_count} open")));

    let body = accordion(items)
        .mode(accordion::Mode::Multiple)
        .open(state.open_notifications.iter().cloned())
        .on_toggle(Message::ToggleNotification);

    card::card(
        column([
            header.into(),
            card::content(container(body).padding([0, 16]).width(Fill)).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .into()
}
