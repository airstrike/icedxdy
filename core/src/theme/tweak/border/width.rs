//! Border-width atom.

use std::ops::Add;

use super::super::{Tweak, TweakSet};

/// Border-width tweak in pixels. Resolves to `style.border.width`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Width(pub f32);

impl From<Width> for Tweak {
    fn from(w: Width) -> Self {
        Tweak::BorderWidth(w.0)
    }
}

impl<R: Into<Tweak>> Add<R> for Width {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
