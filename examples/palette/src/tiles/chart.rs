//! Traffic-channels bar chart — 12 stylized bars over a baseline.

use iced::widget::space;
use iced::{Bottom, Fill};
use icedxdy::card;
use icedxdy::core::Border;
use icedxdy::theme;
use icedxdy::theme::container::Style;
use icedxdy::widget::row::Row;
use icedxdy::widget::{badge, column, container, row, text};
use icedxdy::{Component, Theme};

use crate::{Message, State};

const BARS: [f32; 12] = [
    40.0, 65.0, 50.0, 80.0, 95.0, 70.0, 110.0, 85.0, 120.0, 100.0, 75.0, 90.0,
];
const BAR_WIDTH: f32 = 20.0;
const CHART_HEIGHT: f32 = 140.0;

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let bars: Vec<Component<'static, Message>> = BARS.iter().map(|h| bar(*h)).collect();

    let chart = Row::with_children(bars)
        .spacing(6)
        .align_y(Bottom)
        .height(CHART_HEIGHT)
        .width(Fill);

    let footer = row![
        text::large("1,234"),
        text::xs("visits").style(theme::text::Muted),
        space::horizontal(),
        badge::secondary("+47%"),
    ]
    .spacing(8)
    .width(Fill);

    let body = column([Component::from(chart), footer.into()])
        .spacing(16)
        .width(Fill);

    card::card(
        column([
            card::header("Traffic channels")
                .description("Top channels by visits")
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

fn bar(height: f32) -> Component<'static, Message> {
    container(space::vertical())
        .width(BAR_WIDTH)
        .height(height)
        .style(|t: &Theme| Style {
            background: Some(t.primary.base.into()),
            border: Border {
                radius: 3.0.into(),
                ..Border::default()
            },
            ..Style::default()
        })
        .into()
}
