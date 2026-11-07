//! Preview 01 contribution history card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/contribution-history.tsx`.

use iced::Widget;
use iced::{Center, Fill, Padding};
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::core::Border;
use icedxdy::theme;
use icedxdy::theme::container::Style;
use icedxdy::widget::{button, column, container, row, text};
use icedxdy::{Component, Space};

use super::api_gap;
use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let chart = column![
        api_gap::panel(
            &["ChartContainer", "BarChart", "XAxis", "ChartTooltip"],
            156.0
        ),
        row![
            month("Dec"),
            month("Jan"),
            month("Feb"),
            month("Mar"),
            month("Apr"),
            month("May"),
        ]
        .spacing(8)
        .width(Fill),
    ]
    .spacing(Space::U2)
    .width(Fill);

    let details = row![
        detail("UPCOMING", "May 25, 2024", "$1,000 scheduled"),
        detail("AUTO-SAVE PLAN", "Accelerated", "Recurring weekly"),
    ]
    .spacing(12)
    .width(Fill);

    card::card(
        column![
            card::header("Contribution History").description("Last 6 months of activity"),
            card::content(column![chart, details].spacing(16).width(Fill)),
            card::footer(
                button("View Full Report")
                    .width(Fill)
                    .on_press(Message::Noop)
            ),
        ]
        .spacing(16)
        .padding(Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}

fn month(label: &'static str) -> Component<'static, Message> {
    container(text::muted(label).size(12.0))
        .width(Fill)
        .align_x(Center)
        .into()
}

fn detail(
    label: &'static str,
    title: &'static str,
    description: &'static str,
) -> Component<'static, Message> {
    container(
        column![
            text::xs(label)
                .medium()
                .letter_spacing(0.08)
                .style(theme::text::Muted),
            text::p(title).semibold(),
            text::muted(description),
        ]
        .spacing(6)
        .width(Fill),
    )
    .padding(12)
    .width(Fill)
    .style(detail_style)
    .into()
}

fn detail_style(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.alpha(0.45).into()),
        border: Border {
            radius: 10.0.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}
