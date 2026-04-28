//! Progress component — shadcn/ui faithful port.
//!
//! A pill-shaped progress indicator, 4px tall (`h-1`), with a `bg-muted`
//! track and `bg-primary` fill. Mirrors nova's `Progress` exactly.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::progress;
//!
//! progress(value).width(Length::Fill)
//! ```

use crate::component::{Component, IntoElement};
use icedxdy_core::Theme;
use icedxdy_core::theme::progress_bar;
use iced_core::Length;
use iced_widget::Renderer;

/// Default girth — `h-1` in nova (4px).
const HEIGHT: f32 = 4.0;

/// A styled progress bar matching shadcn/ui's `Progress` component.
pub struct Progress<'a, Message> {
    value: f32,
    width: Length,
    style: Option<iced_widget::progress_bar::StyleFn<'a, Theme>>,
    _marker: std::marker::PhantomData<Message>,
}

/// Create a [`Progress`] with the given value (0..=100).
pub fn progress<'a, Message: 'a>(value: f32) -> Progress<'a, Message> {
    Progress {
        value: value.clamp(0.0, 100.0),
        width: Length::Fill,
        style: None,
        _marker: std::marker::PhantomData,
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
}

impl<'a, Message: 'a> From<Progress<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(p: Progress<'a, Message>) -> Self {
        let style =
            p.style.unwrap_or_else(|| Box::new(|t| progress_bar::default(t).into()));
        iced_widget::progress_bar(0.0..=100.0, p.value)
            .length(p.width)
            .girth(HEIGHT)
            .style(move |t| style(t))
            .into()
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
