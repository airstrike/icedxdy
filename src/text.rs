//! Typography components
//!
//! Semantic helpers for headings, paragraphs, and more.
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
use iced_core::{Element, Font as IcedFont, Pixels};
use iced_widget::text::Style;
use iced_widget::Renderer;

use crate::theme;
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
    size: Option<f32>,
    weight: Option<Weight>,
    style: Option<fn(&Theme) -> Style>,
}

impl<'a> Text<'a> {
    pub fn new(content: impl IntoFragment<'a>) -> Self {
        Self {
            fragment: content.into_fragment(),
            size: None,
            weight: None,
            style: None,
        }
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().0);
        self
    }

    pub fn medium(mut self) -> Self {
        self.weight = Some(Weight::Medium);
        self
    }

    pub fn semibold(mut self) -> Self {
        self.weight = Some(Weight::Semibold);
        self
    }

    pub fn bold(mut self) -> Self {
        self.weight = Some(Weight::Bold);
        self
    }

    pub fn style(mut self, style: fn(&Theme) -> Style) -> Self {
        self.style = Some(style);
        self
    }
}

impl<'a, Message: 'a> From<Text<'a>> for Element<'a, Message, Theme, Renderer> {
    fn from(t: Text<'a>) -> Self {
        let mut widget = iced_widget::text(t.fragment);

        if let Some(size) = t.size {
            widget = widget.size(size);
        }

        if let Some(weight) = t.weight {
            widget = widget.font(IcedFont {
                weight,
                ..IcedFont::DEFAULT
            });
        }

        if let Some(style) = t.style {
            widget = widget.style(style);
        }

        widget.into()
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
