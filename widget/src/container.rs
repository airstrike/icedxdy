use crate::element::{Element, IntoElement};
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
    content: Element<'a, Message>,
    width: Length,
    height: Length,
    max_width: f32,
    padding: Padding,
    center_x: Option<Length>,
    center_y: Option<Length>,
    style: Option<StyleFn<'static, Theme>>,
}

impl<'a, Message: 'a> Container<'a, Message> {
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: f32::INFINITY,
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
        style: impl Fn(&Theme) -> iced_widget::container::Style + 'static,
    ) -> Self {
        self.style = Some(Box::new(style));
        self
    }
}

impl<'a, Message: 'a> From<Container<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(c: Container<'a, Message>) -> Self {
        Box::new(c).into_element()
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
        let mut c = iced_widget::container(self.content.into_iced())
            .width(self.width)
            .height(self.height)
            .max_width(self.max_width)
            .padding(self.padding);

        if let Some(w) = self.center_x {
            c = c.center_x(w);
        }
        if let Some(h) = self.center_y {
            c = c.center_y(h);
        }
        if let Some(style) = self.style {
            c = c.style(style);
        }

        c.into()
    }
}
