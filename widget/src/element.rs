//! Element abstraction for icedxdy
//!
//! Provides [`IntoElement`] and [`Element`] — a layer between user code and
//! iced that lets parent widgets (like Button) push styling (like Size) down
//! to children before the final conversion to `iced::Element`.

use crate::Size;
use icedxdy_core::Theme;
use crate::Weight;
use iced_widget::Renderer;

/// Trait for icedxdy widgets.
///
/// Implementors can receive parent styling hints (e.g. [`Size`]) and
/// ultimately convert into an `iced::Element`.
pub trait IntoElement<'a, Message, T = Theme, R = Renderer>: 'a {
    /// Parent is telling you what size context you live in.
    /// Implementors should respect explicit user overrides (no-op if already set).
    fn size(&mut self, _size: Size) {}

    /// Parent is telling you what font weight to use.
    /// Implementors should respect explicit user overrides (no-op if already set).
    fn weight(&mut self, _weight: Weight) {}

    /// Consume self and produce an iced Element.
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, T, R>;
}

/// Boxed [`IntoElement`] — our equivalent of `iced::Element`.
pub struct Element<'a, Message, T = Theme, R = Renderer>(
    Box<dyn IntoElement<'a, Message, T, R>>,
);

impl<'a, Message: 'a, T: 'a, R: 'a> Element<'a, Message, T, R> {
    /// Wrap any [`IntoElement`] implementor.
    pub fn new(widget: impl IntoElement<'a, Message, T, R>) -> Self {
        Self(Box::new(widget))
    }

    /// Push a size hint down to the wrapped widget.
    pub fn size(&mut self, size: Size) {
        self.0.size(size);
    }

    /// Push a font weight hint down to the wrapped widget.
    pub fn weight(&mut self, weight: Weight) {
        self.0.weight(weight);
    }

    /// Convert into an `iced::Element`.
    pub fn into_iced(self) -> iced_core::Element<'a, Message, T, R> {
        self.0.into_element()
    }
}

// iced::Element → IntoElement (no-op size/weight, just pass through)
impl<'a, Message: 'a, T: 'a, R: iced_core::Renderer + 'a>
    IntoElement<'a, Message, T, R> for iced_core::Element<'a, Message, T, R>
{
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, T, R> {
        *self
    }
}

// iced::Element → our Element
impl<'a, Message: 'a, T: 'a, R: iced_core::Renderer + 'a>
    From<iced_core::Element<'a, Message, T, R>> for Element<'a, Message, T, R>
{
    fn from(e: iced_core::Element<'a, Message, T, R>) -> Self {
        Element::new(e)
    }
}

// iced Text widget → IntoElement (no-op size/weight — icons keep their own font)
impl<'a, Message: 'a> IntoElement<'a, Message> for iced_widget::Text<'a, Theme, Renderer> {
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        (*self).into()
    }
}

// iced Text widget → our Element
impl<'a, Message: 'a> From<iced_widget::Text<'a, Theme, Renderer>> for Element<'a, Message> {
    fn from(t: iced_widget::Text<'a, Theme, Renderer>) -> Self {
        Element::new(t)
    }
}

// &str → our Element (via our Text, so size propagation works)
impl<'a, Message: 'a> From<&'a str> for Element<'a, Message> {
    fn from(s: &'a str) -> Self {
        Element::new(crate::text::Text::new(s))
    }
}

// String → our Element (via our Text, so size propagation works)
impl<'a, Message: 'a> From<String> for Element<'a, Message> {
    fn from(s: String) -> Self {
        Element::new(crate::text::Text::new(s))
    }
}

// Cow<str> → our Element (via our Text, so size propagation works)
impl<'a, Message: 'a> From<std::borrow::Cow<'a, str>> for Element<'a, Message> {
    fn from(s: std::borrow::Cow<'a, str>) -> Self {
        Element::new(crate::text::Text::new(s))
    }
}

// Our Text → our Element
impl<'a, Message: 'a> From<crate::text::Text<'a>> for Element<'a, Message> {
    fn from(t: crate::text::Text<'a>) -> Self {
        Element::new(t)
    }
}

// Our Button → our Element
impl<'a, Message: Clone + 'a> From<crate::button::Button<'a, Message>> for Element<'a, Message> {
    fn from(b: crate::button::Button<'a, Message>) -> Self {
        Element::new(b)
    }
}

// Our Row → our Element
impl<'a, Message: 'a> From<crate::row::Row<'a, Message>> for Element<'a, Message> {
    fn from(r: crate::row::Row<'a, Message>) -> Self {
        Element::new(r)
    }
}

// Spinner → our Element
#[cfg(feature = "spinner")]
impl<'a, Message: 'a> From<crate::spinner::Spinner<'a>> for Element<'a, Message> {
    fn from(s: crate::spinner::Spinner<'a>) -> Self {
        let iced_el: iced_core::Element<'a, Message, Theme, Renderer> = s.into();
        Element::new(iced_el)
    }
}
