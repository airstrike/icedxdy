//! Typography components
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::text;
//!
//! text::h1("Page Title")
//! text::muted("Secondary text")
//! text::p("Bold text").bold()
//! ```

use iced_core::font::Weight;
use iced_core::text::{Fragment, IntoFragment};
use iced_widget::text::Style;
use iced_widget::Renderer;

use crate::element::IntoElement;
use crate::theme;
use crate::Size;
use crate::Theme;

/// Font sizes based on Tailwind/shadcn scale
pub mod size {
    pub const XS: f32 = 12.0;
    pub const SM: f32 = 14.0;
    pub const BASE: f32 = 16.0;
    pub const LG: f32 = 18.0;
    pub const XL: f32 = 20.0;
    pub const XL2: f32 = 24.0;
    pub const XL3: f32 = 30.0;
    pub const XL4: f32 = 36.0;
}

/// Text widget wrapper with semantic styling
pub struct Text<'a> {
    fragment: Fragment<'a>,
    font: Option<iced_core::Font>,
    size: Option<f32>,
    weight: Option<Weight>,
    style: Option<fn(&Theme) -> Style>,
}

impl<'a> Text<'a> {
    pub fn new(content: impl IntoFragment<'a>) -> Self {
        Self {
            fragment: content.into_fragment(),
            font: None,
            size: None,
            weight: None,
            style: None,
        }
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into().text_size());
        self
    }

    pub fn weight(mut self, weight: impl Into<crate::Weight>) -> Self {
        self.weight = Some(weight.into().into());
        self
    }

    pub fn medium(self) -> Self {
        self.weight(crate::Weight::Medium)
    }

    pub fn semibold(self) -> Self {
        self.weight(crate::Weight::Semibold)
    }

    pub fn bold(self) -> Self {
        self.weight(crate::Weight::Bold)
    }

    pub fn style(mut self, style: fn(&Theme) -> Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Build the iced text widget
    fn build(self) -> iced_widget::Text<'a, Theme, Renderer> {
        let mut widget = iced_widget::text(self.fragment);

        if let Some(size) = self.size {
            widget = widget.size(size);
        }

        if let Some(font) = self.font {
            widget = widget.font(font);
        }

        if let Some(weight) = self.weight {
            widget = widget.weight(weight);
        }

        if let Some(style) = self.style {
            widget = widget.style(style);
        }

        widget
    }
}

impl<'a, Message: 'a> From<Text<'a>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(t: Text<'a>) -> Self {
        t.build().into()
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Text<'a> {
    fn size(&mut self, size: Size) {
        if self.size.is_none() {
            self.size = Some(size.text_size());
        }
    }

    fn weight(&mut self, weight: crate::Weight) {
        if self.weight.is_none() {
            self.weight = Some(weight.into());
        }
    }

    fn into_element(
        self: Box<Self>,
    ) -> iced_core::Element<'a, Message, Theme, Renderer> {
        self.build().into()
    }
}

pub fn h1<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL4)
        .bold()
        .style(theme::text::foreground)
}

pub fn h2<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL3)
        .semibold()
        .style(theme::text::foreground)
}

pub fn h3<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL2)
        .semibold()
        .style(theme::text::foreground)
}

pub fn h4<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL)
        .semibold()
        .style(theme::text::foreground)
}

pub fn p<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::BASE)
        .style(theme::text::foreground)
}

pub fn lead<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL)
        .style(theme::text::muted)
}

pub fn large<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::LG)
        .semibold()
        .style(theme::text::foreground)
}

pub fn small<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .style(theme::text::foreground)
}

pub fn muted<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .style(theme::text::muted)
}

pub fn xs<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XS)
        .style(theme::text::foreground)
}
