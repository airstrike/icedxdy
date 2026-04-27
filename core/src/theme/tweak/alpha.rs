//! Alpha-scale atom — multiplies the resolved `style.background`'s alpha
//! by a constant factor.
//!
//! Useful for translucent overlays where the underlying surface should
//! show through. Matches CSS's `opacity` semantics on a single property.
//! Applies uniformly across statuses (no per-state branching).

use std::ops::Add;

use super::{Tweak, TweakSet};

/// Multiply the resolved style's background alpha by `factor`. No-op if
/// the variant has no background. For non-background catalogs (text,
/// rule, etc.), interpretation is per-catalog — each catalog documents
/// its own behaviour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alpha(pub f32);

impl From<Alpha> for Tweak {
    fn from(a: Alpha) -> Self {
        Tweak::Alpha(a.0)
    }
}

impl<R: Into<Tweak>> Add<R> for Alpha {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
