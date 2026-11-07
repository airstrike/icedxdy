//! Catalog impl bridging our `Theme` into the vendored switch widget.
//! Mirrors the role of `checkbox::element` and `text_input::element`.
//!
//! The cross-crate flow for `.style(...)`:
//!
//! 1. User writes `switch(...).style(Default + tweak::...)` (the variants
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
use icedxdy_core::theme::switch;

use super::widget;

/// Convert our OKLCH-native core `Style` into the vendored widget's `Style`.
/// Preserves `ring` end-to-end (the iced bridge would silently drop it
/// because iced's Style has no halo field).
pub(crate) fn from_core(s: switch::Style) -> widget::Style {
    widget::Style {
        background: s.background.into(),
        background_border_width: s.background_border_width,
        background_border_color: s.background_border_color.into(),
        foreground: s.foreground.into(),
        foreground_border_width: s.foreground_border_width,
        foreground_border_color: s.foreground_border_color.into(),
        text_color: s.text_color.map(Into::into),
        border_radius: s.border_radius.map(Into::into),
        padding_ratio: s.padding_ratio,
        ring: s.ring.map(|r| widget::Ring {
            color: r.color.into(),
            width: r.width,
        }),
    }
}

/// Default styling: nova default-size switch via our OKLCH-native resolver.
fn default(theme: &Theme, status: widget::Status) -> widget::Style {
    from_core(switch::default(theme, status))
}

impl widget::Catalog for Theme {
    type Class<'a> = widget::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: widget::Status) -> widget::Style {
        class(self, status)
    }

    fn selection(&self) -> iced_core::Color {
        Theme::selection(self)
    }
}
