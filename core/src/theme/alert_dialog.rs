//! AlertDialog styling - nova style.
//!
//! The panel is a dialog's (`rounded-xl bg-popover ring-1`). Its footer is
//! `bg-muted/50` with the panel's bottom corners, below a `border-t`; its
//! media is `size-10 rounded-md bg-muted`.

use std::ops::Add;

use super::Theme;
use super::container::{Spec, Style};
use super::tweak::Tweak;
use crate::{Border, Radius, radius};

/// The footer band.
pub fn footer(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.alpha(0.5).into()),
        border: Border {
            radius: Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_right: radius::XL,
                bottom_left: radius::XL,
            },
            ..Border::default()
        },
        ..Style::default()
    }
}

/// The media tile beside or above the title.
pub fn media(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.into()),
        text_color: Some(theme.neutral.foreground),
        border: Border {
            radius: radius::MD.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

/// The media tile's style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Media;

impl From<Media> for Spec {
    fn from(_: Media) -> Spec {
        Spec::variant(media)
    }
}

impl<T: Into<Tweak>> Add<T> for Media {
    type Output = Spec;

    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}
