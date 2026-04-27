//! Background-color atom.

use std::ops::Add;

use super::{Token, Tweak, TweakSet};

/// Background-color tweak. Resolves to `style.background = Some(token.resolve(palette).into())`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bg(pub Token);

impl<T: Into<Token>> From<T> for Bg {
    fn from(t: T) -> Self {
        Bg(t.into())
    }
}

impl From<Bg> for Tweak {
    fn from(b: Bg) -> Self {
        Tweak::Bg(b.0)
    }
}

impl<R: Into<Tweak>> Add<R> for Bg {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
