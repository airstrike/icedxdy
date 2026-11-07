//! Contribution-history heatmap — 7 rows × 24 cols of tinted cells.

use iced::Fill;
use iced::Widget;
use iced::widget::space;
use icedxdy::card;
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::column::Column;
use icedxdy::widget::row::Row;
use icedxdy::widget::{button, column, container};
use icedxdy::{Slot, Theme};

use crate::{Message, State};

const ROWS: usize = 7;
const COLS: usize = 24;
const CELL_PX: f32 = 10.0;
const GAP_PX: f32 = 2.0;

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let grid_rows: Vec<Slot<'static, Message>> = (0..ROWS)
        .map(|r| {
            let cells: Vec<Slot<'static, Message>> = (0..COLS).map(|c| cell(level(r, c))).collect();
            Row::with_children(cells).spacing(GAP_PX).into()
        })
        .collect();

    let grid: Column<'static, Message> = Column::with_children(grid_rows).spacing(GAP_PX);

    let body = container(grid).padding([0, 16]).width(Fill);

    let header = card::header("Contribution history")
        .description("Last 6 months of activity")
        .action(button::ghost("View Full Report").on_press(Message::Noop));

    card::card(
        column([header.into(), card::content(body).into()])
            .spacing(16)
            .padding([16, 0])
            .width(Fill),
    )
    .width(Fill)
    .boxed()
}

fn cell(level: u8) -> Slot<'static, Message> {
    container(space::vertical())
        .width(CELL_PX)
        .height(CELL_PX)
        .style(move |t: &Theme| {
            let alpha = match level {
                0 => 0.10,
                1 => 0.35,
                2 => 0.65,
                _ => 1.0,
            };
            Style {
                background: Some(t.primary.base.scale_alpha(alpha).into()),
                border: Border {
                    radius: 2.0.into(),
                    ..Border::default()
                },
                ..Style::default()
            }
        })
        .into()
}

// Deterministic 4-level noise that LOOKS like real contribution density.
fn level(r: usize, c: usize) -> u8 {
    let bucket = ((r * 7) ^ (c * 13)).wrapping_add(c) % 100;
    match bucket {
        0..=34 => 0,
        35..=64 => 1,
        65..=84 => 2,
        _ => 3,
    }
}
