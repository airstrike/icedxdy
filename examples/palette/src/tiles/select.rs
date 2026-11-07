//! Local select examples backed by iced's real pick-list widget.

use iced::{Fill, Padding};
use iced_widget::pick_list;
use icedxdy::widget::Slot;
use icedxdy::{Space, Theme, font_size};

use crate::Message;

pub fn trigger(selected: &'static str, options: &'static [&'static str]) -> Slot<'static, Message> {
    pick_list(Some(selected), options, |value| (*value).to_owned())
        .on_select(|_| Message::Noop)
        .width(Fill)
        .text_size(font_size::SM)
        .padding(Padding {
            top: Space::U2.px(),
            right: Space::U2.px(),
            bottom: Space::U2.px(),
            left: Space::U2_5.px(),
        })
        .style(select_style)
        .into()
}

fn select_style(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let mut style = <Theme as pick_list::Catalog>::style(
        theme,
        &<Theme as pick_list::Catalog>::default(),
        status,
    );
    style.background = theme.card.base.into();
    style
}
