//! Source-backed gap cards for preview-02 tiles.
//!
//! These are intentionally explicit: a tile may be present in the grid while
//! still documenting which icedxdy primitive is missing before it can be LPP.

use iced::Widget;
use iced::{Center, Fill};
use icedxdy::core::Border;
use icedxdy::theme;
use icedxdy::theme::container::Style;
use icedxdy::widget::{badge, column, container, row, text};
use icedxdy::{Component, Theme, card};

use crate::Message;

pub fn marker() -> Component<'static, Message> {
    badge::destructive("GAP").into()
}

pub fn inline(gaps: &'static [&'static str]) -> Component<'static, Message> {
    let rows = gap_rows(gaps);

    container(column(rows).spacing(8).width(Fill))
        .padding(12)
        .width(Fill)
        .style(inline_style)
        .into()
}

pub fn panel(gaps: &'static [&'static str], height: f32) -> Component<'static, Message> {
    let rows = gap_rows(gaps);

    container(column(rows).spacing(8).width(Fill))
        .padding(12)
        .width(Fill)
        .height(height)
        .align_y(Center)
        .style(inline_style)
        .into()
}

fn gap_rows(gaps: &'static [&'static str]) -> Vec<Component<'static, Message>> {
    let mut rows: Vec<Component<'static, Message>> = vec![
        row![
            marker(),
            text::small("Missing primitive blocks LPP")
                .medium()
                .width(Fill),
        ]
        .spacing(8)
        .width(Fill)
        .into(),
    ];

    rows.extend(gaps.iter().map(|gap| gap_row(gap)));

    rows
}

pub fn missing(
    title: &'static str,
    source: &'static str,
    gaps: &'static [&'static str],
) -> iced::Element<'static, Message, Theme> {
    let mut rows: Vec<Component<'static, Message>> = vec![
        text::xs("SOURCE")
            .medium()
            .letter_spacing(0.08)
            .style(theme::text::Muted)
            .into(),
        text::small(source).medium().into(),
        text::muted("Present in the source grid, blocked from LPP by missing primitives.")
            .width(Fill)
            .into(),
    ];

    rows.extend(gaps.iter().map(|gap| gap_row(gap)));

    let body = column(rows).spacing(10).width(Fill);

    card::card(
        column![
            card::header(title)
                .description("Flagged for icedxdy API coverage.")
                .action(marker()),
            card::content(container(body).width(Fill)),
        ]
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}

fn gap_row(gap: &'static str) -> Component<'static, Message> {
    row![badge::outline("missing"), text::muted(gap).width(Fill),]
        .spacing(8)
        .width(Fill)
        .into()
}

fn inline_style(theme: &Theme) -> Style {
    Style {
        background: Some(theme.card.base.into()),
        border: Border {
            radius: 10.0.into(),
            width: 1.0,
            color: theme.neutral.border.into(),
        },
        ..Style::default()
    }
}
