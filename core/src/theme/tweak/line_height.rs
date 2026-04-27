//! Line-height atom.
//!
//! Composite-name carve-out: kept as `LineHeight` (single composite-name
//! atom) to align with iced's user-facing API (the `LineHeight` type in
//! `iced::widget::text`). See `.claude/PLAN_STYLE_TWEAKS.md` §1 for the
//! rationale — this is the single intentional exception to the
//! no-composite-names rule, justified by iced-vocabulary muscle memory.

use std::ops::Add;

use super::{Tweak, TweakSet};

/// Line-height multiplier (relative to font size). Silently dropped on
/// non-text widgets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineHeight(pub f32);

impl From<LineHeight> for Tweak {
    fn from(lh: LineHeight) -> Self {
        Tweak::LineHeight(lh.0)
    }
}

impl<R: Into<Tweak>> Add<R> for LineHeight {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
