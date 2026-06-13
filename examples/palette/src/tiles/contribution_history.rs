//! Preview 01 contribution history card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/contribution-history.tsx`.

use icedxdy::Theme;

use super::api_gap;
use crate::{Message, State};

#[granita::preview(state = State::dark(), group = "preview-01")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    api_gap::missing(
        "Contribution History",
        "contribution-history.tsx",
        &["ChartContainer", "BarChart", "XAxis", "ChartTooltip"],
    )
}
