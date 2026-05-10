use iced::Element;
use icedxdy::Theme;
use icedxdy::core::style::{
    Tint, accent, destructive, info, muted, neutral, outline, primary, secondary, success, warning,
};
use icedxdy::widget::button::Button;
use icedxdy::widget::{Column, button, row, text};

use crate::Message;

const BTN_WIDTH: f32 = 110.0;

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

pub fn view(flip: bool) -> Element<'static, Message, Theme> {
    let mut filled_row = row().spacing(8);
    let mut outlined_row = row().spacing(8);
    for (idx, (label, tint)) in TINTS.iter().enumerate() {
        let enabled = (idx % 2 == 0) != flip;
        filled_row = filled_row.push(filled_button(label, *tint, enabled).into());
        outlined_row = outlined_row.push(outlined_button(label, *tint, enabled).into());
    }

    let body: Column<'static, Message> = Column::new()
        .push(text::label("Filled").into())
        .push(filled_row.into())
        .push(text::label("Outlined").into())
        .push(outlined_row.into())
        .spacing(10);
    body.into()
}

fn filled_button(label: &'static str, tint: Tint, enabled: bool) -> Button<'static, Message> {
    let b = button(label).width(BTN_WIDTH).style(tint);
    if enabled { b.on_press(Message::Flip) } else { b }
}

fn outlined_button(label: &'static str, tint: Tint, enabled: bool) -> Button<'static, Message> {
    let b = button(label).width(BTN_WIDTH).style(outline + tint);
    if enabled { b.on_press(Message::Flip) } else { b }
}
