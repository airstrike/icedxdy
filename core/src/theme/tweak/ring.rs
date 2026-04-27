//! Focus-ring atom.
//!
//! Phase 1 ships the atom shape; rendering is deferred until input/checkbox
//! land in Phase 5. See `.claude/PLAN_STYLE_TWEAKS.md` §3.

use std::ops::Add;

use super::{Token, Tweak, TweakSet};

/// Focus-ring tweak — outer stroke at `offset` px from the widget edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    pub color: Token,
    pub width: f32,
    pub offset: f32,
}

impl From<Ring> for Tweak {
    fn from(r: Ring) -> Self {
        Tweak::Ring {
            color: r.color,
            width: r.width,
            offset: r.offset,
        }
    }
}

impl<R: Into<Tweak>> Add<R> for Ring {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
