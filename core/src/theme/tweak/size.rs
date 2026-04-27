//! Font-size atom for text-shaped catalogs.

use std::ops::Add;

use super::{Tweak, TweakSet};

/// Font size in iced `Pixels`. Silently dropped on widgets whose `Style`
/// has no text-size field (button, container, input, etc.).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size(pub iced_core::Pixels);

impl<T: Into<iced_core::Pixels>> From<T> for Size {
    fn from(v: T) -> Self {
        Size(v.into())
    }
}

impl From<Size> for Tweak {
    fn from(s: Size) -> Self {
        Tweak::Size(s.0)
    }
}

impl<R: Into<Tweak>> Add<R> for Size {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
