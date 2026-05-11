//! Tints tile — one filled + one outlined button per tint, plus a
//! row of badge variants. Only place in the demo where the nine tints
//! sit side-by-side; everywhere else they appear thematically.
//!
//! Exercises: button (style API: bare tint / outline + tint), badge
//! (default / secondary / destructive / outline / ghost), text variants
//! (label), row, column.

use iced::Fill;
use icedxdy::Component;
use icedxdy::card;
use icedxdy::core::style::{
    Tint, accent, destructive, info, muted, neutral, outline, primary, secondary, success, warning,
};
use icedxdy::widget::{Row, badge, button, column, row, text};
use icedxdy::Theme;

use crate::{Message, State};

const BUTTON_WIDTH: f32 = 110.0;
const BUTTONS_PER_ROW: usize = 5;

const TINTS: &[(&str, Tint)] = &[
    ("Primary", primary),
    ("Secondary", secondary),
    ("Destructive", destructive),
    ("Accent", accent),
    ("Muted", muted),
    ("Info", info),
    ("Success", success),
    ("Warning", warning),
    ("Neutral", neutral),
];

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let filled_rows = tint_rows(state.connected, false);
    let outlined_rows = tint_rows(state.connected, true);

    let badges = row![
        badge("Default"),
        badge::secondary("Secondary"),
        badge::destructive("Destructive"),
        badge::outline("Outline"),
        badge::ghost("Ghost"),
    ]
    .spacing(8);

    let body = column([
        text::label("Filled").into(),
        column(filled_rows).spacing(8).into(),
        text::label("Outlined").into(),
        column(outlined_rows).spacing(8).into(),
        text::label("Badges").into(),
        badges.into(),
    ])
    .spacing(12)
    .width(Fill);

    card::card(
        column([
            card::header("Tint reference")
                .description("Filled + outlined per tint and the badge variants.")
                .into(),
            card::content(body).into(),
        ])
        .spacing(16)
        .padding([16, 0]),
    )
    .width(Fill)
    .into()
}

fn tint_rows(connected: bool, is_outline: bool) -> Vec<Component<'static, Message>> {
    TINTS
        .chunks(BUTTONS_PER_ROW)
        .map(|chunk| {
            let buttons = chunk
                .iter()
                .map(|(label, tint)| tint_button(label, *tint, connected, is_outline).into());
            Row::with_children(buttons).spacing(8).into()
        })
        .collect()
}

fn tint_button(
    label: &'static str,
    tint: Tint,
    connected: bool,
    is_outline: bool,
) -> icedxdy::widget::Button<'static, Message> {
    let mut b = if is_outline {
        button(label).style(outline + tint).width(BUTTON_WIDTH)
    } else {
        button(label).style(tint).width(BUTTON_WIDTH)
    };
    if connected {
        b = b.on_press(Message::Noop);
    }
    b
}
