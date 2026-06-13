//! Preview-02 index-investing card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/index-investing.tsx`.

use iced::Fill;
use icedxdy::theme::container;
use icedxdy::widget::{column, container, text};
use icedxdy::{Theme, card};

use crate::{Message, State};

#[granita::preview(state = State::dark(), group = "preview-02")]
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
    .style(|t: &Theme| {
        let mut style = container::card(t);
        style.text_color = Some(t.muted.foreground.into());
        style.into()
    })
    .into()
}
