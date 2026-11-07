//! Preview-02 upcoming-payments card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/upcoming-payments.tsx`.

use icedxdy::Theme;

use super::api_gap;
use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    api_gap::missing("Upcoming Payments", "upcoming-payments.tsx", &["Calendar"])
}
