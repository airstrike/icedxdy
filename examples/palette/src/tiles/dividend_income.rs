//! Preview-02 dividend-income card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/dividend-income.tsx`.

use icedxdy::Theme;

use super::api_gap;
use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    api_gap::missing(
        "Q2 Dividend Income",
        "dividend-income.tsx",
        &["ChartContainer", "BarChart"],
    )
}
