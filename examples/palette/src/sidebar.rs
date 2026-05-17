//! Left rail — `/create`-style customizer panels.
//!
//! Pure visual placeholders: every control fires `Message::Noop`. The
//! real wiring lands later on `feat/style-api` via the `Tweak` atom
//! system; this file just renders the shape.

use iced::widget::{qr_code, space};
use iced::{Center, Fill};
use icedxdy::core::Border;
use icedxdy::theme;
use icedxdy::theme::container::{Style, card};
use icedxdy::widget::row::Row;
use icedxdy::widget::{badge, button, column, container, row, sidebar, text};
use icedxdy::{Component, Theme};

use crate::{Message, State};

const RAIL_WIDTH: f32 = 240.0;
const QR_CELL_PX: f32 = 2.4;
const SWATCH_PX: f32 = 18.0;

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let content = column![
        style_group(),
        base_color_group(),
        heading_font_group(),
        body_font_group(),
        border_radius_group(),
        variant_group(),
        space::vertical(),
        bottom_actions(),
        footer_block(state),
    ]
    .spacing(10)
    .height(Fill)
    .width(Fill);

    sidebar::fixed(sidebar::content([content.into()]))
        .width(RAIL_WIDTH)
        .height(Fill)
        .into()
}

fn style_group() -> Component<'static, Message> {
    sidebar::group([
        sidebar::group_label("Style").into(),
        sidebar::group_content([preset_card()]).into(),
    ])
    .into()
}

fn preset_card() -> Component<'static, Message> {
    let name = row![
        text::small("preset 50").medium(),
        space::horizontal(),
        text::xs("\u{2714}").style(theme::text::Muted),
    ]
    .align_y(Center)
    .width(Fill);

    let swatches = row![
        swatch(|t: &Theme| t.primary.base),
        swatch(|t: &Theme| t.accent.base),
        swatch(|t: &Theme| t.muted.base),
        swatch(|t: &Theme| t.destructive.base),
        swatch(|t: &Theme| t.success.base),
    ]
    .spacing(6)
    .width(Fill);

    let inner = column![name, swatches].spacing(10).width(Fill);

    container(inner).padding(12).width(Fill).style(card).into()
}

fn swatch(getter: fn(&Theme) -> icedxdy::core::Color) -> Component<'static, Message> {
    container(space::vertical())
        .width(SWATCH_PX)
        .height(SWATCH_PX)
        .style(move |t: &Theme| Style {
            background: Some(getter(t).into()),
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: t.neutral.border,
            },
            ..Style::default()
        })
        .into()
}

fn base_color_group() -> Component<'static, Message> {
    sidebar::group([
        sidebar::group_label("Base Color").into(),
        sidebar::group_content([button::outline("Neutral  \u{25BE}")
            .width(Fill)
            .on_press(Message::Noop)
            .into()])
        .into(),
    ])
    .into()
}

fn border_radius_group() -> Component<'static, Message> {
    let labels = ["0", "0.25", "0.5", "0.75", "1.0"];
    let active_index = 2;

    let buttons: Vec<Component<'static, Message>> = labels
        .iter()
        .enumerate()
        .map(|(i, label)| {
            if i == active_index {
                button::secondary(*label)
                    .width(Fill)
                    .on_press(Message::Noop)
                    .into()
            } else {
                button::outline(*label)
                    .width(Fill)
                    .on_press(Message::Noop)
                    .into()
            }
        })
        .collect();

    let inner = Row::with_children(buttons).spacing(4).width(Fill);

    sidebar::group([
        sidebar::group_label("Border Radius").into(),
        sidebar::group_content([inner.into()]).into(),
    ])
    .into()
}

fn heading_font_group() -> Component<'static, Message> {
    sidebar::group([
        sidebar::group_label("Heading Font").into(),
        sidebar::group_content([button::outline("Inter  \u{25BE}")
            .width(Fill)
            .on_press(Message::Noop)
            .into()])
        .into(),
    ])
    .into()
}

fn body_font_group() -> Component<'static, Message> {
    sidebar::group([
        sidebar::group_label("Body Font").into(),
        sidebar::group_content([button::outline("Inter  \u{25BE}")
            .width(Fill)
            .on_press(Message::Noop)
            .into()])
        .into(),
    ])
    .into()
}

fn variant_group() -> Component<'static, Message> {
    let buttons: Vec<Component<'static, Message>> = vec![
        button::secondary("Default")
            .width(Fill)
            .on_press(Message::Noop)
            .into(),
        button::outline("Outline")
            .width(Fill)
            .on_press(Message::Noop)
            .into(),
        button::outline("Ghost")
            .width(Fill)
            .on_press(Message::Noop)
            .into(),
    ];

    let inner = Row::with_children(buttons).spacing(4).width(Fill);

    sidebar::group([
        sidebar::group_label("Variant").into(),
        sidebar::group_content([inner.into()]).into(),
    ])
    .into()
}

fn bottom_actions() -> Component<'static, Message> {
    let inner = column![
        radio_row("Default", true),
        radio_row("Solid", true),
        row![
            text::xs("\u{2295}").style(theme::text::Muted),
            badge::secondary("preset 50"),
        ]
        .spacing(6)
        .align_y(Center),
        button::ghost("\u{2197}  Open Preset")
            .width(Fill)
            .on_press(Message::Noop),
        button::ghost("\u{21BB}  Shuffle")
            .width(Fill)
            .on_press(Message::Noop),
    ]
    .spacing(6)
    .width(Fill);

    container(inner).padding([4, 8]).width(Fill).into()
}

fn radio_row(label: &'static str, active: bool) -> Component<'static, Message> {
    let bullet: Component<'static, Message> = container(space::vertical())
        .width(8.0)
        .height(8.0)
        .style(move |t: &Theme| Style {
            background: if active {
                Some(t.neutral.foreground.into())
            } else {
                None
            },
            border: Border {
                radius: 999.0.into(),
                width: 1.0,
                color: if active {
                    t.neutral.foreground
                } else {
                    t.neutral.border
                },
            },
            ..Style::default()
        })
        .into();

    let text_widget = if active {
        text::xs(label).style(theme::text::Foreground)
    } else {
        text::xs(label).style(theme::text::Muted)
    };

    let inner = row![bullet, text_widget]
        .spacing(8)
        .align_y(Center)
        .width(Fill);

    container(inner).padding([2, 0]).width(Fill).into()
}

fn footer_block(_state: &State) -> Component<'static, Message> {
    let qr: iced::Element<'static, Message, Theme> =
        qr_code(crate::qr_data()).cell_size(QR_CELL_PX).into();

    let get_code = button("Get Code").width(Fill).on_press(Message::Noop);

    let inner = column![Component::from(qr), get_code]
        .spacing(8)
        .width(Fill)
        .align_x(Center);

    container(inner).padding([8, 8]).width(Fill).into()
}
