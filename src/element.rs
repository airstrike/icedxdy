//! Element abstraction for icedxdy
//!
//! Provides [`IntoElement`] and [`Element`] — a layer between user code and
//! iced that lets parent widgets (like Button) push styling (like Size) down
//! to children before the final conversion to `iced::Element`.

use crate::Size;
use crate::Theme;
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

/// Wrapper for any `iced::Element` that doesn't participate in our styling.
struct Opaque<'a, Message, T, R>(iced_core::Element<'a, Message, T, R>);

impl<'a, Message: 'a, T: 'a, R: iced_core::Renderer + 'a>
    IntoElement<'a, Message, T, R> for Opaque<'a, Message, T, R>
{
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, T, R> {
        self.0
    }
}

// Any iced::Element → our Element (opaque, no styling hooks)
impl<'a, Message: 'a, T: 'a, R: iced_core::Renderer + 'a>
    From<iced_core::Element<'a, Message, T, R>> for Element<'a, Message, T, R>
{
    fn from(e: iced_core::Element<'a, Message, T, R>) -> Self {
        Element::new(Opaque(e))
    }
}

/// Wrap any iced widget into our Element (opaque, no size propagation).
///
/// Use this when passing raw iced widgets (Row, iced text, icons, etc.)
/// to our button helpers:
///
/// ```ignore
/// button::primary(wrap(row![icon::mail(), text("Login")]))
/// ```
pub fn wrap<'a, Message: 'a>(
    widget: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message> {
    Element::new(Opaque(widget.into()))
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

// iced Text widget → our Element (opaque)
impl<'a, Message: 'a> From<iced_widget::Text<'a, Theme, Renderer>> for Element<'a, Message> {
    fn from(t: iced_widget::Text<'a, Theme, Renderer>) -> Self {
        wrap(t)
    }
}

// Our Text → our Element
impl<'a, Message: 'a> From<crate::text::Text<'a>> for Element<'a, Message> {
    fn from(t: crate::text::Text<'a>) -> Self {
        Element::new(t)
    }
}

