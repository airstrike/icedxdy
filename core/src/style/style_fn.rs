//! Composable style function — pairs a `Palette → Status → Style` inner
//! closure with an active `Tint`.

use std::ops::Add;

use crate::Theme;
use crate::palette::Palette;
use crate::style::Style;
use crate::style::status::Status;
use crate::style::tint::Tint;

/// A style function carrying its active tint.
///
/// `inner` runs at resolve time with the tinted palette and the
/// widget's current status; the returned `Style` then merges onto the
/// widget's intrinsic default at apply time.
pub struct StyleFn<F> {
    pub inner: F,
    pub tint: Tint,
}

impl<F> StyleFn<F>
where
    F: Fn(Palette, Status) -> Style + 'static,
{
    /// Resolve this style against the given theme + status.
    pub fn resolve(&self, theme: &Theme, status: Status) -> Style {
        let palette = theme.tinted(self.tint);
        (self.inner)(palette, status)
    }
}

impl<F> From<F> for StyleFn<F>
where
    F: Fn(Palette, Status) -> Style + 'static,
{
    fn from(inner: F) -> Self {
        Self {
            inner,
            tint: Tint::Neutral,
        }
    }
}

/// Lift a stateless `Fn(Palette) -> Style` into a `StyleFn` that
/// ignores `Status`.
pub fn from_stateless<G>(g: G) -> StyleFn<Box<dyn Fn(Palette, Status) -> Style>>
where
    G: Fn(Palette) -> Style + 'static,
{
    StyleFn {
        inner: Box::new(move |p, _s| g(p)),
        tint: Tint::Neutral,
    }
}

/// `Style::default()` inner — used by `From<Tint>` to lift a bare Tint
/// into a StyleFn with no user-side field overrides.
fn empty_inner(_: Palette, _: Status) -> Style {
    Style::default()
}

/// `.style(destructive)` shorthand — a bare `Tint` lifts into a `StyleFn`
/// with empty inner. The widget's intrinsic default fills against the
/// tinted palette, so `.style(destructive)` produces "this widget's
/// natural look, but in destructive's palette" — shadcn's `variant`-only
/// idiom. Per-widget `From<Tint> for iced_widget::*::StyleFn` impls
/// close the chain so the bare Tint goes straight through `.style(...)`.
impl From<Tint> for StyleFn<fn(Palette, Status) -> Style> {
    fn from(tint: Tint) -> Self {
        StyleFn {
            inner: empty_inner,
            tint,
        }
    }
}

// --- Add impls ---------------------------------------------------------------

impl<F> Add<Tint> for StyleFn<F> {
    type Output = StyleFn<F>;
    fn add(self, t: Tint) -> Self::Output {
        StyleFn {
            inner: self.inner,
            tint: t,
        }
    }
}

impl<F> Add<StyleFn<F>> for Tint
where
    F: Fn(Palette, Status) -> Style + 'static,
{
    type Output = StyleFn<F>;
    fn add(self, sf: StyleFn<F>) -> StyleFn<F> {
        StyleFn {
            inner: sf.inner,
            tint: self,
        }
    }
}

impl<F, G> Add<StyleFn<G>> for StyleFn<F>
where
    F: Fn(Palette, Status) -> Style + 'static,
    G: Fn(Palette, Status) -> Style + 'static,
{
    type Output = StyleFn<Box<dyn Fn(Palette, Status) -> Style>>;
    fn add(self, rhs: StyleFn<G>) -> Self::Output {
        let tint = rhs.tint;
        let lhs_inner = self.inner;
        let rhs_inner = rhs.inner;
        StyleFn {
            inner: Box::new(move |p, s| lhs_inner(p, s) + rhs_inner(p, s)),
            tint,
        }
    }
}

impl Add<Tint> for Tint {
    type Output = Tint;
    fn add(self, rhs: Tint) -> Tint {
        rhs
    }
}
