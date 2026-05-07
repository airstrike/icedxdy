//! Component abstraction for icedxdy
//!
//! Provides [`IntoElement`] and [`Component`] — a layer between user code and
//! iced that lets parent widgets (like Button) push styling (like Size) down
//! to children before the final conversion to `iced::Element`.

use crate::Size;
use crate::Weight;
use iced_widget::Renderer;
use icedxdy_core::Theme;

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

/// A generic wrapper around any `icedxdy` component.
pub struct Component<'a, Message, T = Theme, R = Renderer> {
    inner: Box<dyn IntoElement<'a, Message, T, R>>,
    /// Optional click-target message that should fire when this Component's
    /// label is clicked. Populated by `From<Checkbox>` and `From<Switch>` from
    /// the inner widget's `on_toggle(!current_state)`. Field consults this at
    /// mount-time to wrap its rendered label widget in `mouse_area` — matching
    /// shadcn's `<FieldLabel htmlFor>` semantic where the label fires a
    /// synthetic click on the linked control. `None` for non-labeled content
    /// (input, etc.) — Field renders the label widget unwrapped.
    toggle_target: Option<Message>,
}

impl<'a, Message: 'a, T: 'a, R: 'a> Component<'a, Message, T, R> {
    /// Wrap any [`IntoElement`] implementor.
    pub fn new(widget: impl IntoElement<'a, Message, T, R>) -> Self {
        Self {
            inner: Box::new(widget),
            toggle_target: None,
        }
    }

    /// Attach a click-target message. Called by `From<Checkbox>` and
    /// `From<Switch>` with the message that fires `on_toggle(!current_state)`.
    /// Field reads this via [`toggle_target`] to wrap the rendered label in
    /// `mouse_area` — no caller change needed.
    pub fn with_toggle_target(mut self, target: Option<Message>) -> Self {
        self.toggle_target = target;
        self
    }

    /// Return the stored click-target message, if any. Field reads this before
    /// consuming `self` via `into_iced()` so the label can be wrapped in
    /// `mouse_area` without extra builder args on `Field` itself.
    pub fn toggle_target(&self) -> Option<&Message> {
        self.toggle_target.as_ref()
    }

    /// Push a size hint down to the wrapped widget.
    pub fn size(&mut self, size: Size) {
        self.inner.size(size);
    }

    /// Push a font weight hint down to the wrapped widget.
    pub fn weight(&mut self, weight: Weight) {
        self.inner.weight(weight);
    }

    /// Convert into an `iced::Element`.
    pub fn into_iced(self) -> iced_core::Element<'a, Message, T, R> {
        self.inner.into_element()
    }

    /// Map the message type, like `iced::Element::map`.
    pub fn map<B: 'a>(self, f: impl Fn(Message) -> B + 'a) -> Component<'a, B, T, R>
    where
        Message: 'a,
        T: 'a,
        R: iced_core::Renderer + 'a,
    {
        Component::from(self.into_iced().map(f))
    }
}

// iced::Element → IntoElement (no-op size/weight, just pass through)
impl<'a, Message: 'a, T: 'a, R: iced_core::Renderer + 'a> IntoElement<'a, Message, T, R>
    for iced_core::Element<'a, Message, T, R>
{
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, T, R> {
        *self
    }
}

// iced::Element → our Element
impl<'a, Message: 'a, T: 'a, R: iced_core::Renderer + 'a>
    From<iced_core::Element<'a, Message, T, R>> for Component<'a, Message, T, R>
{
    fn from(e: iced_core::Element<'a, Message, T, R>) -> Self {
        Component::new(e)
    }
}

// our Element → iced::Element
impl<'a, Message: 'a, T: 'a, R: iced_core::Renderer + 'a> From<Component<'a, Message, T, R>>
    for iced_core::Element<'a, Message, T, R>
{
    fn from(e: Component<'a, Message, T, R>) -> Self {
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

        impl<'a, Message: 'a $($bounds)*> From<$ty> for Component<'a, Message> {
            fn from(w: $ty) -> Self {
                Component::new(w)
            }
        }
    };
}

into_element!(iced_widget::Text<'a, Theme, Renderer>);
into_element!(iced_widget::text::Rich<'a, Message, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Column<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Row<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Scrollable<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Container<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Space);
into_element!(iced_widget::Button<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Checkbox<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::TextInput<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Tooltip<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Sensor<'a, (), Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Pin<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::MouseArea<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Stack<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Grid<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::ProgressBar<'a, Theme>);
into_element!(iced_widget::Toggler<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Float<'a, Message, Theme, Renderer> where + Clone);

// &str → our Element (via our Text, so size propagation works)
impl<'a, Message: 'a> From<&'a str> for Component<'a, Message> {
    fn from(s: &'a str) -> Self {
        Component::new(crate::text::Text::new(s))
    }
}

// String → our Element (via our Text, so size propagation works)
impl<'a, Message: 'a> From<String> for Component<'a, Message> {
    fn from(s: String) -> Self {
        Component::new(crate::text::Text::new(s))
    }
}

// Slider<T>: T is the value type, not Message
impl<'a, T, Message> IntoElement<'a, Message> for iced_widget::Slider<'a, T, Message, Theme>
where
    T: Copy + Into<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        iced_core::Element::new(*self)
    }
}

impl<'a, T, Message> From<iced_widget::Slider<'a, T, Message, Theme>> for Component<'a, Message>
where
    T: Copy + Into<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn from(w: iced_widget::Slider<'a, T, Message, Theme>) -> Self {
        Component::new(w)
    }
}

// ComboBox<T>
impl<'a, T, Message> IntoElement<'a, Message>
    for iced_widget::ComboBox<'a, T, Message, Theme, Renderer>
where
    T: std::fmt::Display + Clone + 'static,
    Message: Clone + 'a,
{
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        iced_core::Element::new(*self)
    }
}

impl<'a, T, Message> From<iced_widget::ComboBox<'a, T, Message, Theme, Renderer>>
    for Component<'a, Message>
where
    T: std::fmt::Display + Clone + 'static,
    Message: Clone + 'a,
{
    fn from(w: iced_widget::ComboBox<'a, T, Message, Theme, Renderer>) -> Self {
        Component::new(w)
    }
}

// Cow<str> → our Element (via our Text, so size propagation works)
impl<'a, Message: 'a> From<std::borrow::Cow<'a, str>> for Component<'a, Message> {
    fn from(s: std::borrow::Cow<'a, str>) -> Self {
        Component::new(crate::text::Text::new(s))
    }
}
