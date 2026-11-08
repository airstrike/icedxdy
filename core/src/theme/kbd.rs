//! Kbd styling - nova style.
//!
//! `rounded-sm bg-muted text-muted-foreground`; inside tooltip content,
//! `bg-background/20 text-background` (`dark:bg-background/10`).

use std::ops::Add;

use super::Theme;
use super::container::{Spec, Style};
use super::tweak::Tweak;
use crate::{Border, radius};

/// A key on its own: a muted rounded cap.
pub fn default(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.into()),
        text_color: Some(theme.muted.foreground),
        border: Border {
            radius: radius::SM.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

/// A key inside tooltip content, translucent over the tooltip's
/// `bg-foreground`.
pub fn tooltip(theme: &Theme) -> Style {
    let is_dark = theme.neutral.base.l < 0.5;
    let alpha = if is_dark { 0.1 } else { 0.2 };

    Style {
        background: Some(theme.neutral.base.scale_alpha(alpha).into()),
        text_color: Some(theme.neutral.base),
        ..default(theme)
    }
}

/// Default kbd style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Kbd style for keys shown inside a tooltip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tooltip;

impl From<Default> for Spec {
    fn from(_: Default) -> Spec {
        Spec::variant(default)
    }
}

impl From<Tooltip> for Spec {
    fn from(_: Tooltip) -> Spec {
        Spec::variant(tooltip)
    }
}

impl<T: Into<Tweak>> Add<T> for Default {
    type Output = Spec;

    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Tooltip {
    type Output = Spec;

    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}
