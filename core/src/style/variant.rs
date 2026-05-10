//! Variant consts — `outline`, `ghost`, `link`, `filled`.
//!
//! Each variant sets only the structural fields it owns; the widget's
//! intrinsic default fills the rest at apply time. Intrinsic tints
//! follow shadcn: outline/ghost are neutral, link/filled are primary.

#![allow(non_upper_case_globals)]

use crate::Background;
use crate::Color;
use crate::palette::Palette;
use crate::style::Style;
use crate::style::status::Status;
use crate::style::style_fn::StyleFn;
use crate::style::tint::Tint;

fn outline_impl(p: Palette, _s: Status) -> Style {
    Style {
        fill: Some(Background::Color(Color::TRANSPARENT)),
        border_width: Some(1.0),
        border_color: Some(p.border),
        text_color: Some(p.tint_text),
        ..Style::default()
    }
}

fn ghost_impl(_p: Palette, _s: Status) -> Style {
    Style::default()
}

fn link_impl(p: Palette, _s: Status) -> Style {
    Style {
        text_color: Some(p.tint_text),
        ..Style::default()
    }
}

fn filled_impl(_p: Palette, _s: Status) -> Style {
    Style::default()
}

pub const outline: StyleFn<fn(Palette, Status) -> Style> = StyleFn {
    inner: outline_impl,
    tint: Tint::Neutral,
};

pub const ghost: StyleFn<fn(Palette, Status) -> Style> = StyleFn {
    inner: ghost_impl,
    tint: Tint::Neutral,
};

pub const link: StyleFn<fn(Palette, Status) -> Style> = StyleFn {
    inner: link_impl,
    tint: Tint::Primary,
};

pub const filled: StyleFn<fn(Palette, Status) -> Style> = StyleFn {
    inner: filled_impl,
    tint: Tint::Primary,
};
