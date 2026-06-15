//! Preview 01 payout-threshold form card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/payout-threshold.tsx`.

use iced::widget::space;
use iced::{Bottom, Fill, Padding};
use iced_widget::slider::{Handle, HandleShape, Rail, Style, StyleFn};
use icedxdy::card;
use icedxdy::core::{Border, oklch};
use icedxdy::widget::{button, column, field, row, slider, text};
use icedxdy::{Size, Space, Theme, font_size, radius};

use super::{api_gap, select};
use crate::{Message, State};

const CURRENCIES: &[&str] = &[
    "USD — United States Dollar",
    "EUR — Euro",
    "GBP — British Pound",
    "JPY — Japanese Yen",
];

#[granita::preview(state = State::dark(), group = "preview-01")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let header = card::header("Payout Threshold")
        .description("Set the minimum balance required before a payout is triggered.")
        .action(
            row![
                api_gap::marker(),
                button::ghost("x")
                    .size(Size::Sm)
                    .square()
                    .style(close_button_style())
                    .on_press(Message::Noop),
            ]
            .spacing(Space::U2),
        );

    let amount = row![
        text::small("Minimum Payout Amount").medium(),
        space::horizontal(),
        text::p("$2500.00").size(font_size::XL2).semibold(),
    ]
    .align_y(Bottom)
    .width(Fill);

    let bounds = row![
        text::muted("$50 (MIN)"),
        space::horizontal(),
        text::muted("$10,000 (MAX)"),
    ]
    .width(Fill);

    let body = column![
        field::field(select::trigger("USD — United States Dollar", CURRENCIES))
            .label("Preferred Currency"),
        field::field(
            column![
                amount,
                slider(50.0..=10000.0, 2500.0, |_| Message::Noop)
                    .width(Fill)
                    .style(slider_style()),
                bounds,
            ]
            .spacing(Space::U2)
            .width(Fill),
        ),
        field::field(api_gap::inline(&["Textarea"])).label("Notes"),
    ]
    .spacing(Space::U5)
    .width(Fill);

    card::card(
        column![
            header,
            card::content(body),
            card::footer(button("Save Threshold").width(Fill).on_press(Message::Noop)),
        ]
        .spacing(Space::U4)
        .padding(Padding::ZERO.top(Space::U4))
        .width(Fill),
    )
    .width(Fill)
    .into()
}

fn close_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|theme, _status| iced_widget::button::Style {
        background: Some(theme.muted.base.into()),
        text_color: theme.muted.foreground.into(),
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        }
        .into(),
        ..Default::default()
    })
}

fn slider_style() -> StyleFn<'static, Theme> {
    Box::new(|theme, _status| Style {
        rail: Rail {
            backgrounds: (
                iced::Background::Color(theme.neutral.foreground.into()),
                iced::Background::Color(oklch(0.23, 0.0, 0.0).into()),
            ),
            width: 4.0,
            border: Border {
                radius: 999.0.into(),
                ..Border::default()
            }
            .into(),
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 8.0 },
            background: iced::Background::Color(theme.neutral.foreground.into()),
            border_color: theme.neutral.foreground.into(),
            border_width: 0.0,
        },
    })
}
