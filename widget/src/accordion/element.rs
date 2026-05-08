//! Catalog impl bridging our `Theme` into the vendored accordion widget.
//! Mirrors the role of `switch::element` / `text_input::element`.
//!
//! Cross-crate flow for `.style(...)`:
//!
//! 1. User writes `accordion(...).style(Default + tweak::...)` (variants
//!    + `Spec` live in core).
//! 2. The wrapper's `.style()` takes `impl Into<core::accordion::StyleFn>`
//!    via the existing `From<Default> for StyleFn` (orphan rule: both
//!    types local to core).
//! 3. The stored core StyleFn produces a core `Style`.
//! 4. At conversion time the wrapper composes a widget `StyleFn` that
//!    calls the core StyleFn — the widget's `Style` IS core's
//!    `accordion::Style` (no separate widget-side style type to bridge).
//!
//! Step 4 collapses into nothing for accordion because the widget reuses
//! the core's `accordion::Style` directly — there's no per-frame field
//! shape the widget needs that core's catalog can't already produce. The
//! `from_core` shim is therefore identity, but kept named for symmetry
//! with switch/text_input where it IS a real translation.

use icedxdy_core::Theme;
use icedxdy_core::theme::accordion;

use super::widget;

/// Default styling: nova default accordion via our OKLCH-native resolver.
fn default_style(theme: &Theme, status: accordion::Status) -> accordion::Style {
    accordion::default(theme, status)
}

impl widget::Catalog for Theme {
    type Class<'a> = widget::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default_style)
    }

    fn style(&self, class: &Self::Class<'_>, status: accordion::Status) -> accordion::Style {
        class(self, status)
    }
}
