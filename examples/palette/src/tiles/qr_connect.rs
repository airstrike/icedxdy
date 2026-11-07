//! Preview 01 QR-connect card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/qr-connect.tsx`.

use iced::Widget;
use iced::{Center, Color, Fill, Padding};
use icedxdy::card;
use icedxdy::core::{Border, oklch};
use icedxdy::theme::container::Style;
use icedxdy::widget::{button, column, container, qr_code, text};
use icedxdy::{Slot, Theme};

use crate::{Message, State};

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let qr: Slot<'_, Message> = match &state.qr_connect {
        Some(data) => container(qr_code(data).total_size(160.0).style(|_| qr_code::Style {
            cell: Color::BLACK,
            background: Color::WHITE,
        }))
        .padding(16)
        .style(qr_shell_style)
        .into(),
        None => container(text::muted("QR unavailable"))
            .width(192.0)
            .height(192.0)
            .center_x(Fill)
            .center_y(Fill)
            .style(qr_shell_style)
            .into(),
    };

    let description: Slot<'static, Message> = column![
        card::description("Open the Ledger mobile app and scan this"),
        card::description("code to link your device."),
    ]
    .spacing(0)
    .align_x(Center)
    .into();

    let header_body: Slot<'static, Message> = column![
        card::title("Scan to connect your mobile device"),
        description,
    ]
    .spacing(4)
    .align_x(Center)
    .width(Fill)
    .into();
    let header: Slot<'static, Message> = card::header(header_body).into();

    card::card(
        column![
            card::content(
                container(qr)
                    .padding(Padding {
                        top: 24.0,
                        ..Padding::ZERO
                    })
                    .center_x(Fill)
                    .width(Fill)
            ),
            header,
            card::footer(
                button::secondary("Got it")
                    .width(Fill)
                    .on_press(Message::Noop)
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}

fn qr_shell_style(_theme: &Theme) -> Style {
    Style {
        background: Some(oklch(1.0, 0.0, 0.0).into()),
        border: Border {
            radius: 12.0.into(),
            width: 1.0,
            color: oklch(0.922, 0.0, 0.0),
        },
        ..Style::default()
    }
}
