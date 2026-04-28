//! Catalog impl bridging our `Theme` into the vendored text_input widget.
//! Mirrors the role of `button::element`.
//!
//! The cross-crate flow for `.style(...)`:
//!
//! 1. User writes `input.style(Default + tweak::...)` (the variants and
//!    `Spec` live in core).
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
use icedxdy_core::theme;
use icedxdy_core::theme::text_input;

use super::widget;

/// Convert our OKLCH-native core `Style` into the vendored widget's `Style`.
/// Preserves `ring` end-to-end (the iced bridge would silently drop it
/// because iced's Style has no halo field).
pub(crate) fn from_core(s: theme::text_input::Style) -> widget::Style {
    widget::Style {
        background: s.background.into(),
        border: s.border.into(),
        icon: s.icon.into(),
        placeholder: s.placeholder.into(),
        value: s.value.into(),
        selection: s.selection.into(),
        ring: s.ring.map(|r| widget::Ring {
            color: r.color.into(),
            width: r.width,
        }),
    }
}

/// Default styling: nova `h-8` field via our OKLCH-native resolver.
fn default(theme: &Theme, status: widget::Status) -> widget::Style {
    from_core(text_input::default(theme, status))
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
