//! Tooltip content styling - nova style.
//!
//! `rounded-md bg-foreground text-background`.

use super::Theme;
use super::container::Style;
use crate::{Border, radius};

/// A tooltip's content panel.
pub fn content(theme: &Theme) -> Style {
    Style {
        background: Some(theme.neutral.foreground.into()),
        text_color: Some(theme.neutral.base),
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}
