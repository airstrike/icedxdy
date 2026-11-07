//! Observability card from the shadcn `/create` preview.

use iced::Fill;
use iced::Widget;
use iced::gradient::Linear;
use iced::widget::space;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::core::{Border, oklch};
use icedxdy::theme;
use icedxdy::widget::{badge, button, column, container, row, text};

use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let preview = container(space::vertical())
        .height(212.0)
        .width(Fill)
        .style(preview_art);

    let copy = column![
        text::large("Observability Plus is replacing Monitoring").width(Fill),
        text::muted(
            "Switch to the improved way to explore your data,\n\
             with natural language. Monitoring will no longer be\n\
             available on the Pro plan in November, 2025"
        )
        .width(Fill),
    ]
    .spacing(6)
    .width(Fill);

    let footer = row![
        button("Create Query  +").on_press(Message::Noop),
        space::horizontal(),
        badge::secondary("Warning"),
    ]
    .align_y(iced::Center)
    .width(Fill);

    let body = column![preview, copy, space::vertical(), footer]
        .spacing(18)
        .width(Fill)
        .height(Fill);

    card::card(container(body).padding(16).width(Fill).height(Fill))
        .width(Fill)
        .height(432.0)
        .boxed()
}

fn preview_art(_theme: &Theme) -> theme::container::Style {
    let left: iced::Color = oklch(0.16, 0.0, 0.0).into();
    let middle: iced::Color = oklch(0.28, 0.0, 0.0).into();
    let right: iced::Color = oklch(0.58, 0.0, 0.0).into();
    let shadow: iced::Color = oklch(0.10, 0.0, 0.0).into();

    theme::container::Style {
        background: Some(
            Linear::new(0.6)
                .add_stop(0.0, left)
                .add_stop(0.33, shadow)
                .add_stop(0.68, middle)
                .add_stop(1.0, right)
                .into(),
        ),
        border: Border {
            radius: 12.0.into(),
            width: 0.0,
            color: oklch(0.0, 0.0, 0.0),
        },
        ..Default::default()
    }
}
