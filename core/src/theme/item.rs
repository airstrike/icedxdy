//! Item styling.
//!
//! Nova `itemVariants`: `default` is `border-transparent`, `outline` is
//! `border-border`, `muted` is `border-transparent bg-muted/50`, all on a
//! `rounded-lg border` box. `itemMediaVariants`: `default` and `icon` leave
//! the content unframed; `image` is `size-10 overflow-hidden rounded-sm`.

use std::ops::Add;

use super::Theme;
use super::container::{Spec, Style, transparent};
use super::media::Frame;
use super::tweak::Tweak;
use crate::{Border, Color, radius};

/// Default item: a transparent 1px border on a large radius.
pub fn default(_theme: &Theme) -> Style {
    Style {
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: Color::TRANSPARENT,
        },
        ..Style::default()
    }
}

/// Outline item: the default box with a `border-border` stroke.
pub fn outline(theme: &Theme) -> Style {
    let mut style = default(theme);
    style.border.color = theme.neutral.border;
    style
}

/// Muted item: the default box on `bg-muted/50`.
pub fn muted(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.alpha(0.5).into()),
        ..default(theme)
    }
}

/// Image media paint: `rounded-sm`.
pub fn image(_theme: &Theme) -> Style {
    Style {
        border: Border {
            radius: radius::SM.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

/// Default item variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Outline item variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outline;

/// Muted item variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Muted;

/// Icon media variant: the content unframed, centered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Icon;

/// Image media variant: a 40px tile with small rounded corners that clips
/// its content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Image;

impl From<Default> for Spec {
    fn from(_: Default) -> Spec {
        Spec::variant(default)
    }
}

impl From<Outline> for Spec {
    fn from(_: Outline) -> Spec {
        Spec::variant(outline)
    }
}

impl From<Muted> for Spec {
    fn from(_: Muted) -> Spec {
        Spec::variant(muted)
    }
}

impl From<Icon> for Frame {
    fn from(_: Icon) -> Frame {
        Frame::new(Spec::variant(transparent), None, false)
    }
}

impl From<Image> for Frame {
    fn from(_: Image) -> Frame {
        Frame::new(Spec::variant(image), Some(40.0), true)
    }
}

macro_rules! item_add {
    ($($variant:ident => $output:ident),* $(,)?) => {
        $(
            impl<T: Into<Tweak>> Add<T> for $variant {
                type Output = $output;

                fn add(self, rhs: T) -> $output {
                    $output::from(self) + rhs
                }
            }
        )*
    };
}

item_add!(Default => Spec, Outline => Spec, Muted => Spec, Icon => Frame, Image => Frame);

impl<'a> From<Default> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(variant: Default) -> Self {
        Spec::from(variant).into()
    }
}

impl<'a> From<Outline> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(variant: Outline) -> Self {
        Spec::from(variant).into()
    }
}

impl<'a> From<Muted> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(variant: Muted) -> Self {
        Spec::from(variant).into()
    }
}
