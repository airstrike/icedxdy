//! Style API — composable widget styling.
//!
//! Phase 1 lands the type scaffolding (`Tint`, `Status`, `Style`,
//! `StyleFn`) plus `Add` impls. Variants, tint consts, atoms, and
//! per-widget bridges arrive in Phases 2 and 3.

pub mod status;
pub mod style_fn;
pub mod tint;

use std::ops::Add;

use iced_core::Padding;

use crate::{Background, Color, Radius, Shadow};

pub use status::Status;
pub use style_fn::StyleFn;
pub use tint::Tint;

/// A partial widget style — every field is optional so "untouched"
/// stays distinguishable from "explicitly set".
#[derive(Default, Clone, Debug)]
pub struct Style {
    pub fill: Option<Background>,
    pub text_color: Option<Color>,
    pub border_width: Option<f32>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Radius>,
    pub shadow: Option<Shadow>,
    pub ring: Option<Color>,
    pub ring_width: Option<f32>,
    pub ring_offset: Option<f32>,
    pub padding: Option<Padding>,
}

impl Add for Style {
    type Output = Style;
    fn add(self, rhs: Style) -> Style {
        Style {
            fill: rhs.fill.or(self.fill),
            text_color: rhs.text_color.or(self.text_color),
            border_width: rhs.border_width.or(self.border_width),
            border_color: rhs.border_color.or(self.border_color),
            border_radius: rhs.border_radius.or(self.border_radius),
            shadow: rhs.shadow.or(self.shadow),
            ring: rhs.ring.or(self.ring),
            ring_width: rhs.ring_width.or(self.ring_width),
            ring_offset: rhs.ring_offset.or(self.ring_offset),
            padding: rhs.padding.or(self.padding),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Theme;
    use crate::palette::Palette;

    fn solid_fill(c: Color) -> Style {
        Style {
            fill: Some(Background::Color(c)),
            ..Style::default()
        }
    }

    /// Compose a stateless inner closure with `+ Tint`, resolve against a
    /// theme, and confirm the resolution palette tracks the swapped tint.
    #[test]
    fn style_fn_resolves_against_swapped_tint() {
        let theme = Theme::light();
        let inner = |p: Palette, _s: Status| solid_fill(p.base);
        let sf: StyleFn<_> = StyleFn::from(inner);
        let tinted = sf + Tint::Destructive;
        let resolved = tinted.resolve(&theme, Status::Active);

        let expected = theme.destructive.base;
        match resolved.fill {
            Some(Background::Color(got)) => {
                assert!((got.l - expected.l).abs() < 1e-6);
                assert!((got.c - expected.c).abs() < 1e-6);
                assert!((got.h - expected.h).abs() < 1e-6);
                assert!((got.a - expected.a).abs() < 1e-6);
            }
            other => panic!("expected solid fill, got {other:?}"),
        }
    }

    #[test]
    fn style_add_rhs_wins_per_field() {
        let lhs = Style {
            fill: Some(Background::Color(Color::BLACK)),
            border_width: Some(1.0),
            ..Style::default()
        };
        let rhs = Style {
            fill: Some(Background::Color(Color::WHITE)),
            ..Style::default()
        };
        let merged = lhs + rhs;
        match merged.fill {
            Some(Background::Color(c)) => assert_eq!(c, Color::WHITE),
            other => panic!("expected white fill, got {other:?}"),
        }
        assert_eq!(merged.border_width, Some(1.0));
    }
}
