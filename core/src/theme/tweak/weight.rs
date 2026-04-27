//! Font-weight atom for text-shaped catalogs.

use std::ops::Add;

use super::{Tweak, TweakSet};

/// Font weight, wrapping `iced::font::Weight`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weight(pub iced_core::font::Weight);

impl From<iced_core::font::Weight> for Weight {
    fn from(w: iced_core::font::Weight) -> Self {
        Weight(w)
    }
}

impl From<Weight> for Tweak {
    fn from(w: Weight) -> Self {
        Tweak::Weight(w.0)
    }
}

impl<R: Into<Tweak>> Add<R> for Weight {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
