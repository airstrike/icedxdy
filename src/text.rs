//! Typography components
//!
//! Provides a Text wrapper with semantic helpers and `.font("inter")` API.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::text;
//!
//! // Semantic helpers with pre-configured styling
//! text::h1("Page Title")
//! text::muted("Secondary text")
//!
//! // Custom font by name (looked up in icedxdy_fonts registry)
//! text::p("Body text").font("inter")
//!
//! // Chainable weight methods
//! text::p("Bold text").bold()
//! text::p("Custom font").font("inter").semibold()
//! ```

use std::borrow::Cow;

use iced_core::font::Weight;
use iced_core::text::{Fragment, IntoFragment};
use iced_core::{Element, Font as IcedFont, Pixels};
use iced_widget::text::Style;
use iced_widget::Renderer;

use crate::fonts;
use crate::theme;
use crate::Theme;

/// Font sizes based on Tailwind/shadcn scale
pub mod size {
    /// 12px - text-xs
    pub const XS: f32 = 12.0;
    /// 14px - text-sm
    pub const SM: f32 = 14.0;
    /// 16px - text-base
    pub const BASE: f32 = 16.0;
    /// 18px - text-lg
    pub const LG: f32 = 18.0;
    /// 20px - text-xl
    pub const XL: f32 = 20.0;
    /// 24px - text-2xl
    pub const XL2: f32 = 24.0;
    /// 30px - text-3xl
    pub const XL3: f32 = 30.0;
    /// 36px - text-4xl
    pub const XL4: f32 = 36.0;
}

/// A font specification that can be resolved to an iced Font
#[derive(Debug, Clone)]
pub enum FontSpec {
    /// Use a named font from icedxdy_fonts registry
    Named(Cow<'static, str>, Weight),
    /// Use a specific iced Font directly
    Iced(IcedFont),
}

impl FontSpec {
    /// Resolve to an iced Font, returning None if lookup fails
    pub fn resolve(&self) -> Option<IcedFont> {
        match self {
            FontSpec::Named(name, weight) => {
                let result = fonts::lookup_weight(name, *weight);
                if result.is_none() {
                    eprintln!("icedxdy: font '{}' not found, using default", name);
                }
                result
            }
            FontSpec::Iced(font) => Some(*font),
        }
    }
}

impl From<&'static str> for FontSpec {
    fn from(name: &'static str) -> Self {
        FontSpec::Named(Cow::Borrowed(name), Weight::Normal)
    }
}

impl From<String> for FontSpec {
    fn from(name: String) -> Self {
        FontSpec::Named(Cow::Owned(name), Weight::Normal)
    }
}

impl From<IcedFont> for FontSpec {
    fn from(font: IcedFont) -> Self {
        FontSpec::Iced(font)
    }
}

/// Text widget wrapper with semantic styling and font resolution
///
/// Wraps content and styling, resolving fonts when converted to Element.
pub struct Text<'a> {
    fragment: Fragment<'a>,
    size: Option<f32>,
    font: Option<FontSpec>,
    style: Option<fn(&Theme) -> Style>,
}

impl<'a> Text<'a> {
    /// Create a new Text with the given content
    pub fn new(content: impl IntoFragment<'a>) -> Self {
        Self {
            fragment: content.into_fragment(),
            size: None,
            font: None,
            style: None,
        }
    }

    /// Set the font size
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().0);
        self
    }

    /// Set the font by name (e.g., "inter") or iced Font
    ///
    /// Named fonts are looked up in icedxdy_fonts registry.
    /// If not found, falls back to iced default and prints a debug message.
    pub fn font(mut self, font: impl Into<FontSpec>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Set font weight to medium
    pub fn medium(mut self) -> Self {
        if let Some(FontSpec::Named(name, _)) = self.font.take() {
            self.font = Some(FontSpec::Named(name, Weight::Medium));
        } else {
            self.font = Some(FontSpec::Named(Cow::Borrowed("inter"), Weight::Medium));
        }
        self
    }

    /// Set font weight to semibold
    pub fn semibold(mut self) -> Self {
        if let Some(FontSpec::Named(name, _)) = self.font.take() {
            self.font = Some(FontSpec::Named(name, Weight::Semibold));
        } else {
            self.font = Some(FontSpec::Named(Cow::Borrowed("inter"), Weight::Semibold));
        }
        self
    }

    /// Set font weight to bold
    pub fn bold(mut self) -> Self {
        if let Some(FontSpec::Named(name, _)) = self.font.take() {
            self.font = Some(FontSpec::Named(name, Weight::Bold));
        } else {
            self.font = Some(FontSpec::Named(Cow::Borrowed("inter"), Weight::Bold));
        }
        self
    }

    /// Set the text style
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

        if let Some(font_spec) = t.font {
            if let Some(font) = font_spec.resolve() {
                widget = widget.font(font);
            }
        }

        if let Some(style) = t.style {
            widget = widget.style(style);
        }

        widget.into()
    }
}

/// H1 heading - largest heading style (36px, bold)
pub fn h1<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL4)
        .bold()
        .style(theme::text::foreground)
}

/// H2 heading - section heading (30px, semibold)
pub fn h2<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL3)
        .semibold()
        .style(theme::text::foreground)
}

/// H3 heading - subsection heading (24px, semibold)
pub fn h3<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL2)
        .semibold()
        .style(theme::text::foreground)
}

/// H4 heading - small heading (20px, semibold)
pub fn h4<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL)
        .semibold()
        .style(theme::text::foreground)
}

/// Paragraph text - default body text (16px)
pub fn p<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::BASE)
        .style(theme::text::foreground)
}

/// Lead text - intro paragraph style (20px, muted)
pub fn lead<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XL)
        .style(theme::text::muted)
}

/// Large text - emphasized text (18px, semibold)
pub fn large<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::LG)
        .semibold()
        .style(theme::text::foreground)
}

/// Small text - fine print (14px)
pub fn small<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .style(theme::text::foreground)
}

/// Muted text - secondary/helper text (14px, muted color)
pub fn muted<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::SM)
        .style(theme::text::muted)
}

/// Extra small text - fine print, labels (12px)
pub fn xs<'a>(content: impl IntoFragment<'a>) -> Text<'a> {
    Text::new(content)
        .size(size::XS)
        .style(theme::text::foreground)
}
