//! Preview-02 power-usage card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/power-usage.tsx`.

use icedxdy::Theme;

use super::api_gap;
use crate::{Message, State};

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    api_gap::missing(
        "Power Usage",
        "power-usage.tsx",
        &["ChartContainer", "BarChart", "XAxis", "ChartTooltip"],
    )
}
