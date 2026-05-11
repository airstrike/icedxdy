//! Left rail — section nav + theme echo + tint legend.
//!
//! Mirrors shadcn `/create`'s customizer panel: a stacked list of nav
//! items at the top, a section of read-only theme/connection echoes,
//! and a tight tint legend at the bottom. The legend is the only place
//! tints appear as side-by-side samples — everywhere else in the demo
//! they show up through real content.

use iced::widget::space;
use iced::{Center, Fill};
use icedxdy::core::Palette;
use icedxdy::theme;
use icedxdy::widget::{column, container, row, sidebar, text};
use icedxdy::{Component, Theme};

use crate::{Message, NavItem, State};

const RAIL_WIDTH: f32 = 240.0;
const SWATCH_SIZE: f32 = 12.0;

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let content = column([
        nav_group(state.nav),
        status_group(state),
        Component::new(space::vertical()),
        legend_group(),
    ])
    .spacing(8)
    .height(Fill)
    .width(Fill);

    sidebar::fixed(sidebar::content([content.into()]))
        .width(RAIL_WIDTH)
        .height(Fill)
        .into()
}

fn nav_group(active: NavItem) -> Component<'static, Message> {
    let items: Vec<Component<'static, Message>> = NavItem::ALL
        .iter()
        .map(|item| {
            sidebar::menu_item([sidebar::menu_button(item.label())
                .active(*item == active)
                .on_press(Message::SelectNav(*item))
                .into()])
            .into()
        })
        .collect();

    sidebar::group([
        sidebar::group_label("Workspace").into(),
        sidebar::group_content([sidebar::menu(items).into()]).into(),
    ])
    .into()
}

fn status_group(state: &State) -> Component<'static, Message> {
    let mode_label = match state.mode {
        crate::Mode::Light => "Light",
        crate::Mode::Dark => "Dark",
    };

    let connection_label = if state.connected() {
        "Connected"
    } else {
        "Disconnected"
    };

    sidebar::group([
        sidebar::group_label("Theme").into(),
        sidebar::group_content([
            kv_row("Mode", mode_label),
            kv_row("Base color", "Neutral"),
            kv_row("Connection", connection_label),
        ])
        .into(),
    ])
    .into()
}

fn legend_group() -> Component<'static, Message> {
    sidebar::group([
        sidebar::group_label("Tint legend").into(),
        sidebar::group_content(
            TINTS
                .iter()
                .map(|(label, getter)| legend_row(label, *getter))
                .collect::<Vec<_>>(),
        )
        .into(),
    ])
    .into()
}

fn kv_row(key: &'static str, value: &'static str) -> Component<'static, Message> {
    let inner = row![
        text::xs(key).style(theme::text::Muted),
        space::horizontal(),
        text::xs(value).style(theme::text::Foreground),
    ]
    .width(Fill)
    .align_y(Center);

    // Row has no padding builder; wrap in a container for the inset.
    container(inner).padding([4, 4]).width(Fill).into()
}

fn legend_row(
    label: &'static str,
    getter: fn(&Theme) -> Palette,
) -> Component<'static, Message> {
    let swatch = container(space::vertical())
        .width(SWATCH_SIZE)
        .height(SWATCH_SIZE)
        .style(move |t: &Theme| {
            use icedxdy::core::Border;
            use theme::container::Style;
            let p = getter(t);
            Style {
                background: Some(p.base.into()),
                border: Border {
                    radius: 3.0.into(),
                    width: 1.0,
                    color: t.neutral.border,
                },
                ..Style::default()
            }
        });

    let inner = row![swatch, text::xs(label).style(theme::text::Muted)]
        .spacing(8)
        .width(Fill)
        .align_y(Center);

    container(inner).padding([4, 4]).width(Fill).into()
}

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
