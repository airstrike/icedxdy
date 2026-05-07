//! Catalog impl bridging our `Theme` into the vendored checkbox widget.
//! Mirrors the role of `text_input::element`.
//!
//! The cross-crate flow for `.style(...)`:
//!
//! 1. User writes `checkbox(...).style(Default + tweak::...)` (the variants
//!    and `Spec` live in core).
//! 2. The wrapper's `.style()` takes `impl Into<StyleFn>` from core — those
//!    `From` impls live in `icedxdy_core` (orphan rule: `From<Default> for
//!    StyleFn` works because both types are local to core).
//! 3. The stored core StyleFn produces a core `Style` with `ring` populated.
//! 4. At conversion time, the wrapper composes a widget `StyleFn` that calls
//!    the core StyleFn and runs `from_core` to land in the vendored widget's
//!    `Style`, preserving the ring across the crate boundary.
//!
//! This module owns step 4's bridge plus the default Catalog impl used when
//! the user calls no `.style(...)` at all. `Status` is the same type in both
//! crates (the vendored module re-exports core's), so no Status conversion is
//! needed.

use icedxdy_core::Theme;
use icedxdy_core::theme::checkbox;

use super::widget;

/// Convert our OKLCH-native core `Style` into the vendored widget's `Style`.
/// Preserves `ring` end-to-end (the iced bridge would silently drop it
/// because iced's Style has no halo field).
pub(crate) fn from_core(s: checkbox::Style) -> widget::Style {
    widget::Style {
        background: s.background.into(),
        icon_color: s.icon_color.into(),
        border: s.border.into(),
        text_color: s.text_color.map(Into::into),
        ring: s.ring.map(|r| widget::Ring {
            color: r.color.into(),
            width: r.width,
        }),
    }
}

/// Default styling: nova `size-4` box via our OKLCH-native resolver.
fn default(theme: &Theme, status: widget::Status) -> widget::Style {
    from_core(checkbox::default(theme, status))
}

impl widget::Catalog for Theme {
    type Class<'a> = widget::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: widget::Status) -> widget::Style {
        class(self, status)
    }
}
