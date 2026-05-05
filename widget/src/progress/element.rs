//! Public-facing builder for [`Progress`] — wraps the forked
//! [`super::widget::ProgressBar`] in our `IntoElement` system.

use iced_core::Length;
use iced_widget::Renderer;

use icedxdy_core::Theme;
use icedxdy_core::theme::progress_bar;

use crate::component::{Component, IntoElement};

use super::widget;

/// Default girth — `h-1` in nova (4px).
const HEIGHT: f32 = 4.0;

/// A styled progress bar matching shadcn/ui's `Progress` component.
///
/// Animates internally: each render whose `value` differs from the
/// currently-displayed value triggers a 150ms cubic-bezier ease toward
/// the new target.
pub struct Progress<'a, Message> {
    value: f32,
    width: Length,
    style: Option<iced_widget::progress_bar::StyleFn<'a, Theme>>,
    on_idle: Option<Box<dyn Fn(f32) -> Message + 'a>>,
}

/// Create a [`Progress`] with the given value (0..=100).
pub fn progress<'a, Message: 'a>(value: f32) -> Progress<'a, Message> {
    Progress {
        value: value.clamp(0.0, 100.0),
        width: Length::Fill,
        style: None,
        on_idle: None,
    }
}

impl<'a, Message: 'a> Progress<'a, Message> {
    /// Set the current progress value (clamped to `0..=100`).
    pub fn value(mut self, value: impl Into<f32>) -> Self {
        self.value = value.into().clamp(0.0, 100.0);
        self
    }

    /// Set the total width of the bar.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Override the bar's style. Accepts any value convertible into an
    /// `iced_widget::progress_bar::StyleFn` — e.g. our
    /// `theme::progress_bar::Default` variant unit struct or a
    /// `theme::progress_bar::Spec`.
    pub fn style(
        mut self,
        style: impl Into<iced_widget::progress_bar::StyleFn<'a, Theme>>,
    ) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Fire `on_idle(value)` when the easing animation settles at its
    /// target. Useful for gating follow-up actions (dismissing a splash,
    /// navigation, etc.) on the bar reaching a specific value.
    pub fn on_idle(mut self, on_idle: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_idle = Some(Box::new(on_idle));
        self
    }
}

impl<'a, Message: 'a> From<Progress<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(p: Progress<'a, Message>) -> Self {
        let style = p
            .style
            .unwrap_or_else(|| Box::new(|t| progress_bar::default(t).into()));
        let mut bar = widget::ProgressBar::new(0.0..=100.0, p.value)
            .length(p.width)
            .girth(HEIGHT)
            .style(move |t| style(t));
        if let Some(on_idle) = p.on_idle {
            bar = bar.on_idle(move |v| on_idle(v));
        }
        bar.into()
    }
}

impl<'a, Message: 'a> From<Progress<'a, Message>> for Component<'a, Message> {
    fn from(p: Progress<'a, Message>) -> Self {
        Component::new(p)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Progress<'a, Message> {
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        (*self).into()
    }
}
