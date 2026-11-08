//! Dialog and sheet panel styling — nova style
//!
//! - [`content`]: `DialogContent`, `rounded-xl bg-popover text-popover-foreground
//!   ring-1 ring-foreground/10`.
//! - [`sheet`]: `SheetContent`, `bg-popover text-popover-foreground shadow-lg`.

use iced_core::Vector;

use super::Theme;
use super::container::Style;
use crate::{Border, Color, Shadow, radius};

/// A dialog's panel.
pub fn content(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.popover.base.into()),
        text_color: Some(p.popover.foreground),
        border: Border {
            radius: radius::XL.into(),
            width: 1.0,
            color: p.neutral.foreground.alpha(0.1),
        },
        ..Style::default()
    }
}

/// A sheet's panel.
pub fn sheet(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.popover.base.into()),
        text_color: Some(p.popover.foreground),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::BLACK.alpha(0.1),
            offset: Vector::new(0.0, 10.0),
            blur_radius: 15.0,
        },
        ..Style::default()
    }
}
