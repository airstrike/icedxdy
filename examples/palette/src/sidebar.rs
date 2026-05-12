//! Left rail — `/create`-style customizer panels.
//!
//! Pure visual placeholders: every control fires `Message::Noop`. The
//! real wiring lands later on `feat/style-api` via the `Tweak` atom
//! system; this file just renders the shape.

use iced::widget::space;
use iced::{Center, Fill};
use icedxdy::theme;
use icedxdy::widget::column::Column;
use icedxdy::widget::row::Row;
use icedxdy::widget::{badge, button, column, container, row, sidebar, text};
use icedxdy::{Component, Theme};

use crate::{Message, State};

const RAIL_WIDTH: f32 = 240.0;
const QR_CELLS: usize = 8;
const QR_CELL_PX: f32 = 10.0;
const QR_GAP_PX: f32 = 1.0;

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let content = column![
        style_group(),
        base_color_group(),
        border_radius_group(),
        heading_font_group(),
        body_font_group(),
        variant_group(),
        echo_block(),
        space::vertical(),
        footer_block(),
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

fn footer_block() -> Component<'static, Message> {
    let qr = qr_placeholder();

    let get_code = button("Get Code").width(Fill).on_press(Message::Noop);

    let inner = column![qr, get_code].spacing(8).width(Fill).align_x(Center);

    container(inner).padding([8, 8]).width(Fill).into()
}

fn qr_placeholder() -> Component<'static, Message> {
    let rows: Vec<Component<'static, Message>> = (0..QR_CELLS)
        .map(|r| {
            let cells: Vec<Component<'static, Message>> =
                (0..QR_CELLS).map(|c| qr_cell(qr_filled(r, c))).collect();
            Row::with_children(cells).spacing(QR_GAP_PX).into()
        })
        .collect();

    Column::with_children(rows).spacing(QR_GAP_PX).into()
}

fn qr_cell(filled: bool) -> Component<'static, Message> {
    container(space::vertical())
        .width(QR_CELL_PX)
        .height(QR_CELL_PX)
        .style(move |t: &Theme| {
            use theme::container::Style;
            let color = if filled {
                t.sidebar.foreground
            } else {
                t.sidebar.background
            };
            Style {
                background: Some(color.into()),
                ..Style::default()
            }
        })
        .into()
}

fn qr_filled(r: usize, c: usize) -> bool {
    let r = r as u32;
    let c = c as u32;
    ((r * 3 + c * 5) ^ (r ^ c).wrapping_mul(7)).count_ones() & 1 == 1
}
