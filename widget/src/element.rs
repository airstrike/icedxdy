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

    /// Map the message type, like `iced::Element::map`.
    pub fn map<B: 'a>(
        self,
        f: impl Fn(Message) -> B + 'a,
    ) -> Element<'a, B, T, R>
    where
        Message: 'a,
        T: 'a,
        R: iced_core::Renderer + 'a,
    {
        Element::from(self.into_iced().map(f))
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

// our Element → iced::Element
impl<'a, Message: 'a, T: 'a, R: iced_core::Renderer + 'a>
    From<Element<'a, Message, T, R>> for iced_core::Element<'a, Message, T, R>
{
    fn from(e: Element<'a, Message, T, R>) -> Self {
        e.into_iced()
    }
}

/// Implement [`IntoElement`] and `From<W> for Element` for an iced widget type.
///
/// This gives the widget no-op size/weight propagation and wraps it
/// in an `iced::Element` on conversion.
macro_rules! into_element {
    ($ty:ty) => {
        into_element!($ty where);
    };
    ($ty:ty where $($bounds:tt)*) => {
        impl<'a, Message: 'a $($bounds)*> IntoElement<'a, Message> for $ty {
            fn into_element(
                self: Box<Self>,
            ) -> iced_core::Element<'a, Message, Theme, Renderer> {
                iced_core::Element::new(*self)
            }
        }

        impl<'a, Message: 'a $($bounds)*> From<$ty> for Element<'a, Message> {
            fn from(w: $ty) -> Self {
                Element::new(w)
            }
        }
    };
}

into_element!(iced_widget::Text<'a, Theme, Renderer>);
into_element!(iced_widget::text::Rich<'a, Message, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Column<'a, Message, Theme, Renderer>);
into_element!(iced_widget::Scrollable<'a, Message, Theme, Renderer>);
into_element!(iced_widget::Sensor<'a, (), Message, Theme, Renderer>);

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

