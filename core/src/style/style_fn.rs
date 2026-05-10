//! `StyleFn<F>` — a `Palette × Status → Style` closure paired with the
//! `Tint` that selects which palette is active at resolve time.

use std::ops::Add;

use crate::Theme;
use crate::palette::Palette;
use crate::style::Style;
use crate::style::status::Status;
use crate::style::tint::Tint;

/// Composable style. `inner` produces a partial `Style`; `tint` selects which
/// palette `resolve()` hands to `inner` at apply time. The `Add` impls below
/// let users compose this against more `StyleFn`s, against `Tint` (which
/// swaps the tint slot), and against other styling atoms.
pub struct StyleFn<F> {
    pub inner: F,
    pub tint: Tint,
}

impl<F> StyleFn<F>
where
    F: Fn(Palette, Status) -> Style + 'static,
{
    /// Look up `tint`'s palette on `theme` and run `inner`.
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

/// Lift a stateless `Fn(Palette) -> Style` into the universal status-aware
/// shape. Stateless atoms (`bg`, `fg`, ...) use this to ignore `Status`.
pub fn from_stateless<G>(g: G) -> StyleFn<Box<dyn Fn(Palette, Status) -> Style>>
where
    G: Fn(Palette) -> Style + 'static,
{
    StyleFn {
        inner: Box::new(move |p, _s| g(p)),
        tint: Tint::Neutral,
    }
}

fn empty_inner(_: Palette, _: Status) -> Style {
    Style::default()
}

/// `.style(<tint>)` shorthand. Produces a `StyleFn` with no field overrides
/// — the widget's intrinsic default carries the look; the tint just swaps
/// which palette resolves it. Per-widget `From<Tint> for iced_widget::*::
/// StyleFn` impls close the chain so call sites can pass a bare `Tint`.
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
