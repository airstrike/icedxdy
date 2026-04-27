//! Horizontal-alignment atom for text-shaped catalogs.

use std::ops::Add;

use super::{Tweak, TweakSet};

/// Horizontal text alignment, wrapping `iced::alignment::Horizontal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Align(pub iced_core::alignment::Horizontal);

impl From<iced_core::alignment::Horizontal> for Align {
    fn from(h: iced_core::alignment::Horizontal) -> Self {
        Align(h)
    }
}

impl From<Align> for Tweak {
    fn from(a: Align) -> Self {
        Tweak::Align(a.0)
    }
}

impl<R: Into<Tweak>> Add<R> for Align {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
