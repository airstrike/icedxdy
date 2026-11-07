//! Preview-02 index-investing card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/index-investing.tsx`.

use iced::Fill;
use iced::Widget;
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::{column, container, text};
use icedxdy::{Theme, card, radius};

use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    let copy = text::muted(
        "Over time, this smooths out the average cost of your investments. \
         When prices drop, your fixed amount buys more shares. When prices rise, \
         you buy fewer. The result is a lower average cost per share compared \
         to lump-sum investing during volatile periods.",
    )
    .line_height(1.55)
    .width(Fill);

    card::card(
        column![
            card::header("Dollar-Cost Averaging")
                .description("A strategy for building wealth over time."),
            card::content(container(copy).padding([12, 0]).width(Fill)),
        ]
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .style(index_card_style)
    .boxed()
}

fn index_card_style(theme: &Theme) -> iced_widget::container::Style {
    Style {
        text_color: Some(theme.muted.foreground),
        background: Some(theme.card.base.into()),
        border: Border {
            radius: radius::XL.into(),
            width: 1.0,
            color: theme.neutral.foreground.alpha(0.10),
        },
        ..Default::default()
    }
    .into()
}
