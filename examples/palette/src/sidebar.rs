//! Left rail — `/create`-style customizer panels.
//!
//! Pure visual placeholders: every control fires `Message::Noop`. The
//! real wiring lands later on `feat/style-api` via the `Tweak` atom
//! system; this file just renders the shape.

use iced::widget::{qr_code, space};
use iced::{Center, Fill};
use icedxdy::theme;
use icedxdy::widget::row::Row;
use icedxdy::widget::{badge, button, column, container, row, sidebar, text};
use icedxdy::{Component, Theme};

use crate::{Message, State};

const RAIL_WIDTH: f32 = 240.0;
const QR_CELL_PX: f32 = 2.5;

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let content = column![
        style_group(),
        base_color_group(),
        border_radius_group(),
        heading_font_group(),
        body_font_group(),
        variant_group(),
        echo_block(),
        space::vertical(),
        footer_block(state),
    ]
    .spacing(8)
    .height(Fill)
    .width(Fill);

    sidebar::fixed(sidebar::content([content.into()]))
        .width(RAIL_WIDTH)
        .height(Fill)
        .into()
}

fn style_group() -> Component<'static, Message> {
    let chip_row = row![badge::secondary("preset 50")].align_y(Center);

    let actions = row![
        button::ghost("Open Preset").on_press(Message::Noop),
        button::ghost("Random").on_press(Message::Noop),
        button::ghost("Shuffle").on_press(Message::Noop),
    ]
    .spacing(4)
    .align_y(Center);

    sidebar::group([
        sidebar::group_label("Style").into(),
        sidebar::group_content([chip_row.into(), actions.into()])
            .spacing(8)
            .into(),
    ])
    .into()
}

fn base_color_group() -> Component<'static, Message> {
    sidebar::group([
        sidebar::group_label("Base Color").into(),
        sidebar::group_content([
            button::outline("Neutral  \u{25BE}")
                .width(Fill)
                .on_press(Message::Noop)
                .into(),
        ])
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
        sidebar::group_content([
            button::outline("Inter  \u{25BE}")
                .width(Fill)
                .on_press(Message::Noop)
                .into(),
        ])
        .into(),
    ])
    .into()
}

fn body_font_group() -> Component<'static, Message> {
    sidebar::group([
        sidebar::group_label("Body Font").into(),
        sidebar::group_content([
            button::outline("Inter  \u{25BE}")
                .width(Fill)
                .on_press(Message::Noop)
                .into(),
        ])
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

fn echo_block() -> Component<'static, Message> {
    let inner = column![
        text::xs("Default").style(theme::text::Foreground),
        text::xs("Solid").style(theme::text::Muted),
    ]
    .spacing(2)
    .width(Fill);

    container(inner).padding([4, 8]).width(Fill).into()
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
