//! Preview-02 cover-art card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/cover-art.tsx`.

use icedxdy::Theme;

use super::api_gap;
use crate::{Message, State};

pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    api_gap::missing("Cover Art", "cover-art.tsx", &["File input"])
}
