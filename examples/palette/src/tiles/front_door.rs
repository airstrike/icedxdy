//! Preview-02 front-door card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/front-door.tsx`.

use iced::Widget;
use iced::{Fill, Right};
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::{badge, column, container, row, space, text};
use icedxdy::{Space, Theme, card};

use super::api_gap;
use crate::{Message, State, icon};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let action = row![
        api_gap::marker(),
        text::muted("Locked"),
        icon::square().size(16),
    ]
    .spacing(Space::U2);

    let camera = container(
        column![
            row![space::horizontal(), badge("Live").style(badge::Destructive),]
                .width(Fill)
                .align_y(iced::Center),
            space::vertical(),
        ]
        .width(Fill)
        .height(Fill),
    )
    .height(190.0)
    .width(Fill)
    .padding(8)
    .align_x(Right)
    .style(camera_style);

    card::card(
        column![
            card::header("Front Door")
                .description("Smart Lock Pro")
                .action(action),
            card::content(camera),
        ]
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}

fn camera_style(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.into()),
        border: Border {
            radius: 10.0.into(),
            width: 1.0,
            color: theme.neutral.border,
        },
        ..Style::default()
    }
}
