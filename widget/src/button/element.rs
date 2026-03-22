use iced_core::text::Span;
use iced_core::Font;
use iced_widget::text::Rich;

use crate::element::{Element, IntoElement};
use icedxdy_core::theme;
use crate::Size;
use icedxdy_core::Theme;

type StyleFn = fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style;

use super::widget;

fn default(theme: &Theme, status: widget::Status) -> widget::Style {
    theme::button::default(theme, status.into()).into()
}

impl widget::Catalog for Theme {
    type Class<'a> = widget::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: widget::Status) -> widget::Style {
        class(self, status)
    }
}

/// A button that wraps our [`Element`] system.
///
/// Holds a [`Size`] which is propagated to its content when converting
/// to an `iced::Element`. This means a `Size::Lg` button automatically
/// gets larger text, larger icons, etc.
pub struct Button<'a, Message, Theme = icedxdy_core::Theme, Renderer = iced_widget::Renderer>
where
    Renderer: iced_core::Renderer,
    Theme: iced_widget::button::Catalog,
{
    content: Element<'a, Message, Theme, Renderer>,
    size: Size,
    weight: crate::Weight,
    square: bool,
    rounded: bool,
    style: Theme::Class<'a>,
    on_press: Option<Message>,
}

/// Create a button with default size and style
pub fn button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Button<'a, Message> {
    Button {
        content: content.into(),
        size: Size::Default,
        weight: crate::Weight::Medium,
        square: false,
        rounded: false,
        style: Box::new(theme::button::default),
        on_press: None,
    }
}

impl<'a, Message: Clone + 'a> Button<'a, Message> {
    /// Set the size variant
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Make the button square (equal width and height).
    pub fn square(mut self) -> Self {
        self.square = true;
        self
    }

    /// Make the button fully rounded (pill/circle shape).
    pub fn rounded(mut self) -> Self {
        self.rounded = true;
        self
    }

    /// Set the font weight
    pub fn weight(mut self, weight: impl Into<crate::Weight>) -> Self {
        self.weight = weight.into();
        self
    }

    /// Set the style
    pub fn style(mut self, style: StyleFn) -> Self {
        self.style = Box::new(style);
        self
    }

    /// Set the on_press message
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>>
    for iced_core::Element<'a, Message, Theme, iced_widget::Renderer>
{
    fn from(mut btn: Button<'a, Message>) -> Self {
        btn.content.size(btn.size);
        btn.content.weight(btn.weight);

        let size = btn.size;
        let content = btn.content.into_iced();

        let h = size.height();
        let mut iced_btn = if btn.square {
            super::widget::Button::new(iced_widget::center(content))
                .width(h)
                .height(h)
                .padding(0)
        } else {
            super::widget::Button::new(content)
                .height(h)
                .padding(size.padding())
        };

        let rounded = btn.rounded;
        iced_btn = iced_btn.style(move |t: &Theme, s: super::widget::Status| {
            let iced_status: iced_widget::button::Status = s.into();
            let mut style: super::widget::Style = (btn.style)(t, iced_status).into();
            if rounded {
                style.border.radius = (u32::MAX as f32).into();
            }
            style
        });

        if let Some(on_press) = btn.on_press {
            iced_btn = iced_btn.on_press(on_press);
        }

        iced_btn.into()
    }
}

impl<'a, Message: Clone + 'a> IntoElement<'a, Message> for Button<'a, Message> {
    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, iced_widget::Renderer> {
        (*self).into()
    }
}

/// A link widget that looks like a text link with underline on hover
pub struct Link {
    label: String,
}

impl Link {
    pub fn new(label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
        }
    }

    pub fn on_press<'a, Message: Clone + 'a>(
        self,
        message: Message,
    ) -> Rich<'a, Message, Message, Theme> {
        let span: Span<'a, Message, Font> = Span::new(self.label).link(message);
        Rich::with_spans([span])
            .on_link_click(|msg| msg)
            .style(theme::text::link)
    }
}
