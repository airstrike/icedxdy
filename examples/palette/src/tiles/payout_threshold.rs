//! Preview 01 payout-threshold form card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/payout-threshold.tsx`.

use iced::Widget;
use iced::widget::{space, svg};
use iced::{Bottom, Center, Fill, Padding};
use icedxdy::card;
use icedxdy::core::Border;
use icedxdy::widget::{Slot, button, column, container, field, row, slider, text};
use icedxdy::{Size, Space, Theme, font_size, radius};

use super::{api_gap, select};
use crate::{Message, State};

const CURRENCIES: &[&str] = &[
    "USD — United States Dollar",
    "EUR — Euro",
    "GBP — British Pound",
    "JPY — Japanese Yen",
];
const X_ICON: &[u8] = br##"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24">
  <path d="M18 6 6 18"/>
  <path d="m6 6 12 12"/>
</svg>
"##;

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let header = card::header("Payout Threshold")
        .description("Set the minimum balance required before a payout is triggered.")
        .action(
            row![
                api_gap::marker(),
                button::ghost(close_icon())
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
        text::p(format!("${:.2}", state.payout_threshold))
            .size(font_size::XL2)
            .semibold(),
    ]
    .align_y(Bottom)
    .width(Fill);

    let bounds = row![
        text::muted("$50 (MIN)"),
        space::horizontal(),
        text::muted("$10,000 (MAX)"),
    ]
    .width(Fill);

    let body = field::group([
        field::field(select::trigger("USD — United States Dollar", CURRENCIES))
            .label("Preferred Currency")
            .into(),
        field::field(
            column![
                amount,
                slider(50.0..=10000.0)
                    .value(state.payout_threshold)
                    .on_value_change(Message::SetPayoutThreshold)
                    .width(Fill),
                bounds,
            ]
            .spacing(Space::U2)
            .width(Fill),
        )
        .into(),
        field::field(api_gap::panel(&["Textarea"], 90.0))
            .label("Notes")
            .into(),
    ]);

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
    .boxed()
}

fn close_icon() -> Slot<'static, Message> {
    let icon = svg(svg::Handle::from_memory(X_ICON))
        .width(16.0)
        .height(16.0)
        .style(|theme: &Theme, _| iced_widget::svg::Style {
            color: Some(theme.muted.foreground.into()),
        });

    container(icon)
        .width(16.0)
        .height(16.0)
        .align_x(Center)
        .align_y(Center)
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
