//! Border-color atom.

use std::ops::Add;

use super::super::{Token, Tweak, TweakSet};

/// Border-color tweak. Resolves to `style.border.color`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub Token);

impl<T: Into<Token>> From<T> for Color {
    fn from(t: T) -> Self {
        Color(t.into())
    }
}

impl From<Color> for Tweak {
    fn from(c: Color) -> Self {
        Tweak::BorderColor(c.0)
    }
}

impl<R: Into<Tweak>> Add<R> for Color {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
