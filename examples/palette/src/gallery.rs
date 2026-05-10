use iced::Element;
use icedxdy::Theme;
use icedxdy::core::Palette;
use icedxdy::core::style::{
    Status, Style, StyleFn, Tint, accent, destructive, info, muted, neutral, outline, primary,
    secondary, success, warning,
};
use icedxdy::theme;
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

pub fn view() -> Element<'static, Message, Theme> {
    let mut filled_row = row().spacing(8);
    let mut outlined_row = row().spacing(8);
    for (label, tint) in TINTS {
        filled_row = filled_row.push(filled_button(label, *tint).into());
        outlined_row = outlined_row.push(outlined_button(label, *tint).into());
    }

    let body: Column<'static, Message> = Column::new()
        .push(text::label("Filled").into())
        .push(filled_row.into())
        .push(text::label("Outlined").into())
        .push(outlined_row.into())
        .spacing(10);
    body.into()
}

fn filled_button(label: &'static str, tint: Tint) -> Button<'static, Message> {
    // `Default + tint` reuses button.rs's `default_impl` (which paints
    // fill = `p.base`, text = `p.foreground`) and swaps the resolved
    // palette to `tint`. `+ Tint` is RHS-wins on the tint slot.
    let style: StyleFn<fn(Palette, Status) -> Style> = theme::button::Default.into();
    button(label)
        .width(BTN_WIDTH)
        .style(style + tint)
}

fn outlined_button(label: &'static str, tint: Tint) -> Button<'static, Message> {
    button(label)
        .width(BTN_WIDTH)
        .style(outline + tint)
}
