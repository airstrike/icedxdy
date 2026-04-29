pub mod widget;
mod element;

use crate::component::{Component, IntoElement};
use crate::Size;
use icedxdy_core::Theme;
use iced_core::{Length, Padding, Pixels};
use iced_widget::container::StyleFn;
use iced_widget::Renderer;

/// Create a [`Container`] wrapping the given content.
///
/// ```ignore
/// container(text::h1("Hello")).padding(16).center_x(Fill)
/// ```
pub struct Container<'a, Message> {
    content: Component<'a, Message>,
    width: Length,
    height: Length,
    max_width: f32,
    min_width: f32,
    min_height: f32,
    padding: Padding,
    center_x: Option<Length>,
    center_y: Option<Length>,
    style: Option<StyleFn<'static, Theme>>,
}

impl<'a, Message: 'a> Container<'a, Message> {
    pub fn new(content: impl Into<Component<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: f32::INFINITY,
            min_width: 0.0,
            min_height: 0.0,
            padding: Padding::ZERO,
            center_x: None,
            center_y: None,
            style: None,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    pub fn min_width(mut self, min_width: impl Into<Pixels>) -> Self {
        self.min_width = min_width.into().0;
        self
    }

    pub fn min_height(mut self, min_height: impl Into<Pixels>) -> Self {
        self.min_height = min_height.into().0;
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn center_x(mut self, width: impl Into<Length>) -> Self {
        self.center_x = Some(width.into());
        self
    }

    pub fn center_y(mut self, height: impl Into<Length>) -> Self {
        self.center_y = Some(height.into());
        self
    }

    pub fn style(
        mut self,
        style: impl Into<StyleFn<'static, Theme>>,
    ) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Center this container horizontally and vertically inside the
    /// available space. Shortcut for `.center_x(Fill).center_y(Fill)`.
    pub fn center(self, length: impl Into<Length>) -> Self {
        let l = length.into();
        self.center_x(l).center_y(l)
    }
}

impl<'a, Message: 'a> From<Container<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(c: Container<'a, Message>) -> Self {
        Box::new(c).into_element()
    }
}

impl<'a, Message: 'a> From<Container<'a, Message>> for crate::Component<'a, Message> {
    fn from(c: Container<'a, Message>) -> Self {
        crate::Component::new(c)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Container<'a, Message> {
    fn size(&mut self, size: Size) {
        self.content.size(size);
    }

    fn weight(&mut self, weight: crate::Weight) {
        self.content.weight(weight);
    }

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        let mut c = widget::Container::new(self.content.into_iced())
            .width(self.width)
            .height(self.height)
            .max_width(self.max_width)
            .min_width(self.min_width)
            .min_height(self.min_height)
            .padding(self.padding);

        if let Some(w) = self.center_x {
            c = c.center_x(w);
        }
        if let Some(h) = self.center_y {
            c = c.center_y(h);
        }
        if let Some(style) = self.style {
            // Bridge: wrapper's `style` is `iced_widget::container::StyleFn`
            // (so existing variants like `theme::container::CardPreview`
            // flow naturally via their existing `From<X> for iced::StyleFn`
            // impls). At construction we wrap that closure to land in the
            // vendored widget's `Style` via the field-identical `From`.
            let bridge: widget::StyleFn<'static, Theme> =
                Box::new(move |t: &Theme| widget::Style::from((style)(t)));
            c = c.class(bridge);
        }

        c.into()
    }
}
