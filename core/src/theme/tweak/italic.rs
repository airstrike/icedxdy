//! Italic-style marker atom for text-shaped catalogs.

use std::ops::Add;

use super::{Tweak, TweakSet};

/// Italic font style. Silently dropped on non-text widgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Italic;

impl From<Italic> for Tweak {
    fn from(_: Italic) -> Self {
        Tweak::Italic
    }
}

impl<R: Into<Tweak>> Add<R> for Italic {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
