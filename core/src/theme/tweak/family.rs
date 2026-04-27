//! Font-family atom for text-shaped catalogs.

use std::ops::Add;

use super::{Tweak, TweakSet};

/// Font family, wrapping `iced::Font`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Family(pub iced_core::Font);

impl From<iced_core::Font> for Family {
    fn from(f: iced_core::Font) -> Self {
        Family(f)
    }
}

impl From<Family> for Tweak {
    fn from(f: Family) -> Self {
        Tweak::Family(f.0)
    }
}

impl<R: Into<Tweak>> Add<R> for Family {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
