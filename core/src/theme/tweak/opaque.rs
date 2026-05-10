//! Opaque-background atom — replaces the variant's (possibly translucent)
//! REST-state background with the page's neutral base color.
//! Hover/Pressed/Disabled keep the variant's natural transitions so the
//! interactive feedback isn't lost.
//!
//! Matches the pre-refactor closure form (extended to preserve hover):
//!
//! ```ignore
//! Style { background: Some(theme.neutral.base.into()), ..base }
//! ```
//!
//! Used on overlay buttons (like the View Code button) where the variant's
//! translucent bg would let the underlying surface bleed through.

use std::ops::Add;

use super::{Tweak, TweakSet};

/// Replace the resolved style's background with `palette.background`.
/// No-op if the variant has no background to begin with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opaque;

impl From<Opaque> for Tweak {
    fn from(_: Opaque) -> Self {
        Tweak::Opaque
    }
}

impl<R: Into<Tweak>> Add<R> for Opaque {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
