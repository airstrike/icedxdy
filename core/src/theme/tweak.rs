//! Composable style atoms — cross-widget tweak system.
//!
//! Atoms (`Bg`, `Fg`, `Opaque`, etc.) build up a `TweakSet` via `+`.
//! Per-widget `Spec`s (e.g. `theme::button::Spec`) consume tweaks at
//! resolve time, applying what they care about and silently dropping the
//! rest (CSS-style cascade semantics).
//!
//! Some primitives (`Radius`, `Shadow`) participate in this system without
//! a dedicated atom newtype — they live at `core::Radius` / `core::Shadow`
//! and bridge into `Tweak` via the `From` and `Add` impls in this module.
//!
//! See `.claude/PLAN_STYLE_TWEAKS.md` for the full design doc.

use std::ops::Add;

use crate::{Radius, Shadow};

pub mod align;
pub mod alpha;
pub mod bg;
pub mod border;
pub mod family;
pub mod fg;
pub mod italic;
pub mod line_height;
pub mod opaque;
pub mod ring;
pub mod size;
pub mod state;
pub mod token;
pub mod weight;

pub use align::Align;
pub use alpha::Alpha;
pub use bg::Bg;
pub use family::Family;
pub use fg::Fg;
pub use italic::Italic;
pub use line_height::LineHeight;
pub use opaque::Opaque;
pub use ring::Ring;
pub use size::Size;
pub use state::{active, disabled, focus, hover};
pub use token::Token;
pub use weight::Weight;

// Re-export the primitive `radius()` constructor here so call sites that
// use the tweak prelude (`use icedxdy::theme::tweak::*`) get it automatically.
pub use crate::radius;

/// A single style tweak. Each atom struct converts into one of these
/// variants via `From`. Per-widget `Spec`s match on the variants they
/// care about and ignore the rest.
#[derive(Debug, Clone)]
pub enum Tweak {
    Bg(Token),
    Fg(Token),
    BorderColor(Token),
    BorderWidth(f32),
    Radius(Radius),
    Shadow(Shadow),
    Ring {
        color: Token,
        width: f32,
        offset: f32,
    },
    Opaque,
    Alpha(f32),
    Size(iced_core::Pixels),
    Weight(iced_core::font::Weight),
    Family(iced_core::Font),
    LineHeight(f32),
    Italic,
    Align(iced_core::alignment::Horizontal),
    State(Trigger, TweakSet),
}

/// An accumulator of tweaks. Built up via `+` from atom structs.
#[derive(Debug, Clone, Default)]
pub struct TweakSet(pub(crate) Vec<Tweak>);

/// State trigger for conditional tweaks (`hover(...)`, `focus(...)`,
/// `active(...)`, `disabled(...)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    Hover,
    Focus,
    Active,
    Disabled,
}

impl<T: Into<Tweak>> From<T> for TweakSet {
    fn from(t: T) -> TweakSet {
        TweakSet(vec![t.into()])
    }
}

impl<R: Into<Tweak>> Add<R> for TweakSet {
    type Output = TweakSet;
    fn add(mut self, rhs: R) -> TweakSet {
        self.0.push(rhs.into());
        self
    }
}

// --- Primitive bridges -------------------------------------------------------
//
// `Radius` and `Shadow` participate in the tweak system without a dedicated
// atom newtype. The `From` impls let them flow into `Tweak`; the `Add` impls
// let them compose with other atoms via `+` (matches the shape of `Bg`/`Fg`).

impl From<Radius> for Tweak {
    fn from(r: Radius) -> Self {
        Tweak::Radius(r)
    }
}

impl<R: Into<Tweak>> Add<R> for Radius {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}

impl From<Shadow> for Tweak {
    fn from(s: Shadow) -> Self {
        Tweak::Shadow(s)
    }
}

impl<R: Into<Tweak>> Add<R> for Shadow {
    type Output = TweakSet;
    fn add(self, rhs: R) -> TweakSet {
        TweakSet(vec![self.into(), rhs.into()])
    }
}
