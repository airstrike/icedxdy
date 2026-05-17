//! Savings targets tile — progress bars with caption + button per row.
//!
//! Exercises: progress, button (outline / link), text variants (large /
//! small / xs / lead), container, row, column.

use iced::Fill;
use icedxdy::Component;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::{button, column, container, progress, row, space, text};

use crate::{Message, State};

struct Target {
    label: &'static str,
    sub: &'static str,
    percent: f32,
}

const TARGETS: &[Target] = &[
    Target {
        label: "Emergency fund",
        sub: "$8,200 / $12,000",
        percent: 68.0,
    },
    Target {
        label: "Vacation",
        sub: "$1,140 / $3,000",
        percent: 38.0,
    },
    Target {
        label: "New laptop",
        sub: "$1,890 / $2,000",
        percent: 94.0,
    },
];

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let lead = text::lead("Three goals, three trajectories.").width(Fill);

    let rows: Vec<Component<'static, Message>> = TARGETS
        .iter()
        .map(|t| target_row(t, state.connected))
        .collect();

    let footer = row![
        space::horizontal(),
        button::link("View full report").on_press(Message::Noop),
    ]
    .spacing(8)
    .width(Fill);

    let body = column(
        std::iter::once(lead.into())
            .chain(rows)
            .chain([footer.into()]),
    )
    .spacing(16);

    card::card(
        column([
            card::header("Savings targets")
                .description("Progress against monthly goals.")
                .into(),
            card::content(container(body).padding([0, 16]).width(Fill)).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .into()
}

fn target_row(t: &'static Target, connected: bool) -> Component<'static, Message> {
    let head = row![
        text::small(t.label).medium(),
        space::horizontal(),
        text::xs(format!("{:.0}%", t.percent)).style(theme::text::Muted),
    ]
    .spacing(8)
    .width(Fill);

    let bar = progress(t.percent).width(Fill);

    let footer = row![
        text::xs(t.sub).style(theme::text::Muted),
        space::horizontal(),
        {
            let mut b = button::outline("Adjust");
            if connected {
                b = b.on_press(Message::Noop);
            }
            b
        },
    ]
    .spacing(8)
    .width(Fill);

    column([head.into(), Component::from(bar), footer.into()])
        .spacing(8)
        .width(Fill)
        .into()
}
