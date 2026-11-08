//! Skeleton loading placeholder.
//!
//! Mirrors shadcn's `<Skeleton>` primitive: a rounded muted block whose
//! dimensions are supplied by the call site. shadcn's `animate-pulse` is not
//! ported; the block is static.

use std::marker::PhantomData;

use iced_core::Element;
use iced_core::Length::{self, Shrink};
use iced_widget::Renderer;
use icedxdy_core::theme::container::Style;
use icedxdy_core::{Border, Radius, Theme, radius};

use crate::compose::{Compose, Slot};
use crate::space;

/// A muted placeholder block.
pub struct Skeleton<'a, Message> {
    width: Length,
    height: Length,
    radius: Radius,
    _marker: PhantomData<&'a Message>,
}

/// Create a [`Skeleton`] with `rounded-md` corners and no size; set one
/// with [`width`](Skeleton::width) and [`height`](Skeleton::height).
pub fn skeleton<'a, Message: 'a>() -> Skeleton<'a, Message> {
    Skeleton {
        width: Shrink,
        height: Shrink,
        radius: radius::MD.into(),
        _marker: PhantomData,
    }
}

impl<'a, Message: 'a> Skeleton<'a, Message> {
    /// Set the width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set the corner radius (`u32::MAX as f32` for `rounded-full`).
    pub fn radius(mut self, radius: impl Into<Radius>) -> Self {
        self.radius = radius.into();
        self
    }
}

impl<'a, Message: 'a> From<Skeleton<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(skeleton: Skeleton<'a, Message>) -> Self {
        Box::new(skeleton).compose()
    }
}

impl<'a, Message: 'a> From<Skeleton<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Skeleton<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Skeleton<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let radius = self.radius;

        crate::container(space::vertical())
            .width(self.width)
            .height(self.height)
            .style(move |theme| style(theme, radius))
            .into()
    }
}

fn style(theme: &Theme, radius: Radius) -> Style {
    Style {
        background: Some(theme.muted.base.into()),
        border: Border {
            radius,
            ..Default::default()
        },
        ..Default::default()
    }
}
