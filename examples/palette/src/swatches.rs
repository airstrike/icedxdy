use iced::widget::Space;
use iced::widget::text::{Style, StyleFn};
use iced::{Element, Fill};
use icedxdy::Theme;
use icedxdy::core::Palette;
use icedxdy::core::style::{bg, border, radius};
use icedxdy::widget::container::Container;
use icedxdy::widget::{Column, container, row, text};

use crate::Message;

// Mirror `icedxdy::core::radius::{SM, MD}` (shadcn nova spec) — held
// here as `f32` so the `radius` atom (which shadows the `radius` module
// path) resolves without a use-rename.
const RADIUS_SM: f32 = 6.0;
const RADIUS_MD: f32 = 8.0;

#[allow(clippy::type_complexity)]
const TINTS: &[(&str, fn(&Theme) -> Palette)] = &[
    ("neutral", |t| t.neutral),
    ("primary", |t| t.primary),
    ("secondary", |t| t.secondary),
    ("muted", |t| t.muted),
    ("accent", |t| t.accent),
    ("destructive", |t| t.destructive),
    ("info", |t| t.info),
    ("success", |t| t.success),
    ("warning", |t| t.warning),
];

pub fn view(theme: &Theme) -> Element<'static, Message, Theme> {
    let mut grid: Column<'static, Message> = Column::new().spacing(12);
    for chunk in TINTS.chunks(3) {
        let mut r_widget = row().spacing(12);
        for (label, getter) in chunk {
            r_widget = r_widget.push(card(theme, label, getter(theme)).into());
        }
        grid = grid.push(r_widget.into());
    }
    grid.into()
}

fn card(theme: &Theme, label: &'static str, p: Palette) -> Container<'static, Message> {
    let body: Column<'static, Message> = Column::new()
        .push(text::label(label).into())
        .push(swatch_bar(p.base, 24.0).into())
        .push(swatch_bar(p.foreground, 14.0).into())
        .push(swatch_bar(p.border, 10.0).into())
        .push(
            container(text::small("Aa").style(text_color(p.tint_text)))
                .padding([2, 6])
                .class(border(p.border) + radius(RADIUS_SM))
                .into(),
        )
        .spacing(6)
        .padding(12)
        .width(Fill);

    container(body)
        .class(bg(theme.card.base) + border(theme.card.border) + radius(RADIUS_MD))
        .width(160)
}

fn swatch_bar(color: icedxdy::Color, height: f32) -> Container<'static, Message> {
    container(Space::new().width(Fill).height(height))
        .width(Fill)
        .height(height)
        .class(bg(color) + radius(RADIUS_SM))
}

fn text_color(c: icedxdy::Color) -> StyleFn<'static, Theme> {
    Box::new(move |_t: &Theme| Style {
        color: Some(c.into()),
    })
}
