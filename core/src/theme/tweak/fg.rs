//! Foreground/active-region color atom.

use std::ops::Add;

use super::{Token, Tweak, TweakSet};

/// Foreground/active-region color tweak. By convention binds to a widget's
/// figure region (text for typography widgets; bar for progress; icon for
/// checkbox; thumb for switch).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fg(pub Token);

impl<T: Into<Token>> From<T> for Fg {
    fn from(t: T) -> Self {
        Fg(t.into())
    }
}

impl From<Fg> for Tweak {
    fn from(f: Fg) -> Self {
        Tweak::Fg(f.0)
    }
}

impl<R: Into<Tweak>> Add<R> for Fg {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
