//! State combinator free fns: `hover`, `focus`, `active`, `disabled`.
//!
//! Each takes any `impl Into<TweakSet>` and wraps it in a state-conditional
//! `Tweak::State(trigger, set)`. The blanket `From<T: Into<Tweak>> for
//! TweakSet` in `super` lets these accept either a single atom or a `+`
//! composite seamlessly.

use super::{Trigger, Tweak, TweakSet};

/// Hover-state tweak: applied when status is `Hovered` or `Pressed`.
pub fn hover(t: impl Into<TweakSet>) -> Tweak {
    Tweak::State(Trigger::Hover, t.into())
}

/// Focus-state tweak: applied when the widget reports a focused status
/// (Input, etc.). Button has no focus tracking yet — silently dropped there.
pub fn focus(t: impl Into<TweakSet>) -> Tweak {
    Tweak::State(Trigger::Focus, t.into())
}

/// Active-state tweak: applied when the widget is pressed.
pub fn active(t: impl Into<TweakSet>) -> Tweak {
    Tweak::State(Trigger::Active, t.into())
}

/// Disabled-state tweak: applied when the widget is disabled.
pub fn disabled(t: impl Into<TweakSet>) -> Tweak {
    Tweak::State(Trigger::Disabled, t.into())
}
