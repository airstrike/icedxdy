//! Skeleton loading placeholder.
//!
//! Mirrors shadcn's `<Skeleton>` primitive: a rounded muted block whose
//! dimensions are supplied by the call site.

use std::marker::PhantomData;

use iced_core::Element;
use iced_core::Length::{self, Shrink};
use iced_widget::Renderer;
use icedxdy_core::theme::container::Style;
use icedxdy_core::{Border, Radius, Theme, radius};

use crate::compose::{Compose, Slot};
use crate::space;

pub struct Skeleton<'a, Message> {
    width: Length,
    height: Length,
    radius: Radius,
    _marker: PhantomData<&'a Message>,
}

pub fn skeleton<'a, Message: 'a>() -> Skeleton<'a, Message> {
    Skeleton {
        width: Shrink,
        height: Shrink,
        radius: radius::MD.into(),
        _marker: PhantomData,
    }
}

impl<'a, Message: 'a> Skeleton<'a, Message> {
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

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
