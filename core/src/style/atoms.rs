//! Atom fns — surgical single-property overrides plus state combinators.
//!
//! Each atom produces a boxed `StyleFn` that sets exactly one field;
//! they compose with `+` against variants, tints, and each other. State
//! combinators (`hover`, `focus`, `active`, `disabled`) gate an inner
//! atom on a matching `Status`, returning `Style::default()` otherwise.

use crate::Background;
use crate::Color;
use crate::Radius;
use crate::palette::Palette;
use crate::style::Style;
use crate::style::status::Status;
use crate::style::style_fn::StyleFn;
use crate::style::style_fn::from_stateless;

type BoxedStyleFn = StyleFn<Box<dyn Fn(Palette, Status) -> Style>>;

pub fn bg(c: impl Into<Background>) -> BoxedStyleFn {
    let bg = c.into();
    from_stateless(move |_p| Style {
        fill: Some(bg),
        ..Style::default()
    })
}

pub fn fg(c: impl Into<Color>) -> BoxedStyleFn {
    let color = c.into();
    from_stateless(move |_p| Style {
        text_color: Some(color),
        ..Style::default()
    })
}

pub fn radius(r: impl Into<Radius>) -> BoxedStyleFn {
    let r = r.into();
    from_stateless(move |_p| Style {
        border_radius: Some(r),
        ..Style::default()
    })
}

pub fn border(c: impl Into<Color>) -> BoxedStyleFn {
    let color = c.into();
    from_stateless(move |_p| Style {
        border_color: Some(color),
        ..Style::default()
    })
}

pub fn ring(c: impl Into<Color>) -> BoxedStyleFn {
    let color = c.into();
    from_stateless(move |_p| Style {
        ring: Some(color),
        ..Style::default()
    })
}

fn gate(inner: BoxedStyleFn, gate_on: Status) -> BoxedStyleFn {
    let tint = inner.tint;
    let inner_fn = inner.inner;
    StyleFn {
        inner: Box::new(move |p, s| {
            if s == gate_on {
                inner_fn(p, s)
            } else {
                Style::default()
            }
        }),
        tint,
    }
}

pub fn hover(s: BoxedStyleFn) -> BoxedStyleFn {
    gate(s, Status::Hovered)
}

pub fn focus(s: BoxedStyleFn) -> BoxedStyleFn {
    gate(s, Status::Focused)
}

pub fn active(s: BoxedStyleFn) -> BoxedStyleFn {
    gate(s, Status::Pressed)
}

pub fn disabled(s: BoxedStyleFn) -> BoxedStyleFn {
    gate(s, Status::Disabled)
}
