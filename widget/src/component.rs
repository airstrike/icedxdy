//! Component abstraction for icedxdy
//!
//! Provides [`IntoElement`] and [`Component`] — a layer between user code and
//! iced that lets parent widgets (like Button) push styling (like Size) down
//! to children before the final conversion to `iced::Element`.

use crate::Size;
use crate::Weight;
use iced_core::Color;
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;

/// The boxed child element every icedxdy widget slot carries.
type El<'a, Message> = iced_core::Element<'a, Message, Theme, Renderer>;

/// Trait for icedxdy widgets.
///
/// Implementors can receive parent styling hints (e.g. [`Size`]) and
/// ultimately convert into an `iced::Element`.
///
/// The trait deliberately does NOT take a lifetime parameter so that
/// `Box<dyn IntoElement<...> + 'a>` (and therefore [`Component<'a>`]) is
/// covariant in `'a`. The lifetime of the produced element comes from the
/// late-bound `'a` on [`into_element`] together with the `Self: 'a` bound:
/// callers pick a shorter lifetime when needed, matching how
/// [`iced_core::Element`] works.
pub trait IntoElement<Message, T = Theme, R = Renderer> {
    /// Parent is telling you what size context you live in.
    /// Implementors should respect explicit user overrides (no-op if already set).
    fn size(&mut self, _size: Size) {}

    /// Parent is telling you what font weight to use.
    /// Implementors should respect explicit user overrides (no-op if already set).
    fn weight(&mut self, _weight: Weight) {}

    /// Parent is telling you what absolute line-height (in logical pixels)
    /// to use. Used by Badge to push `text-xs`'s nova `leading-4` (16px)
    /// down to its content so the rendered pill height matches shadcn.
    /// Implementors should respect explicit user overrides (no-op if already set).
    fn line_height(&mut self, _line_height: f32) {}

    fn foreground_color(&mut self, _color: Color) {}

    /// Consume self and produce an iced Element.
    fn into_element<'a>(self: Box<Self>) -> iced_core::Element<'a, Message, T, R>
    where
        Self: 'a;
}

/// A generic wrapper around any `icedxdy` component.
///
/// Covariant in `'a` (just like [`iced_core::Element`]) so heterogeneous-
/// lifetime children can co-exist in containers like [`column!`]/[`row!`].
pub struct Component<'a, Message, T = Theme, R = Renderer> {
    inner: Box<dyn IntoElement<Message, T, R> + 'a>,
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
    pub fn new(widget: impl IntoElement<Message, T, R> + 'a) -> Self {
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

    /// Push an absolute line-height (in logical pixels) down to the wrapped
    /// widget. See [`IntoElement::line_height`].
    pub fn line_height(&mut self, line_height: f32) {
        self.inner.line_height(line_height);
    }

    pub fn foreground_color(&mut self, color: impl Into<Color>) {
        self.inner.foreground_color(color.into());
    }

    /// Convert into an `iced::Element`.
    pub fn into_iced(self) -> iced_core::Element<'a, Message, T, R> {
        self.inner.into_element()
    }

    /// Map the message type, like `iced::Element::map`.
    pub fn map<B: 'static>(self, f: impl Fn(Message) -> B + 'a) -> Component<'a, B, T, R>
    where
        Message: 'a,
        T: 'static,
        R: iced_core::Renderer + 'static,
    {
        Component::from(self.into_iced().map(f).boxed())
    }
}

// iced::Element → IntoElement (no-op size/weight, just pass through)
impl<'a, Message: 'a, T: 'a, R: iced_core::Renderer + 'a> IntoElement<Message, T, R>
    for iced_core::Element<'a, Message, T, R>
{
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, T, R>
    where
        Self: 'b,
    {
        // Element is covariant in its lifetime; the `Self: 'b` bound gives us
        // `'a: 'b`, so subtyping converts `Element<'a, …>` to `Element<'b, …>`.
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
        impl<'a, Message: 'a $($bounds)*> IntoElement<Message> for $ty {
            fn into_element<'b>(
                self: Box<Self>,
            ) -> iced_core::Element<'b, Message, Theme, Renderer>
            where
                Self: 'b,
            {
                (*self).boxed()
            }
        }

        impl<'a, Message: 'a $($bounds)*> From<$ty> for Component<'a, Message> {
            fn from(w: $ty) -> Self {
                Component::new(w)
            }
        }
    };
}

into_element!(iced_widget::Button<'a, Message, El<'a, Message>, Theme> where + Clone);
into_element!(iced_widget::Checkbox<'a, Message, Theme, Renderer> where + Clone);
into_element!(iced_widget::Column<El<'a, Message>> where + Clone);
into_element!(iced_widget::Container<'a, El<'a, Message>, Theme> where + Clone);
into_element!(iced_widget::Float<'a, El<'a, Message>, Theme> where + Clone);
into_element!(iced_widget::Grid<El<'a, Message>> where + Clone);
into_element!(iced_widget::MouseArea<'a, Message, El<'a, Message>> where + Clone);
into_element!(iced_widget::Pin<El<'a, Message>> where + Clone);
into_element!(iced_widget::ProgressBar<'a, Theme>);
into_element!(iced_widget::QRCode<'a, Theme>);
into_element!(iced_widget::Radio<'a, Message, Theme> where + Clone);
into_element!(iced_widget::Row<El<'a, Message>> where + Clone);
into_element!(iced_widget::Rule<'a, Theme>);
into_element!(iced_widget::Scrollable<'a, Message, El<'a, Message>, Theme> where + Clone);
into_element!(iced_widget::Sensor<'a, (), Message, El<'a, Message>> where + Clone);
into_element!(iced_widget::Space);
into_element!(iced_widget::Stack<El<'a, Message>> where + Clone);
#[cfg(feature = "svg")]
into_element!(iced_widget::Svg<'a, Theme>);
into_element!(iced_widget::TextInput<'a, Message, Theme> where + Clone);
into_element!(iced_widget::Toggler<'a, Message, Theme> where + Clone);
into_element!(iced_widget::Tooltip<'a, El<'a, Message>, El<'a, Message>, Theme> where + Clone);
into_element!(iced_widget::text::Rich<'a, Message, Message, Theme> where + Clone);

// iced's Text gets a manual impl so `foreground_color` overrides its style
// closure with the supplied color. The macro's blanket impl would no-op the
// hint, which would silently ignore color pushes onto icon glyphs
// (`iced_lucide` produces them as iced's Text, not icedxdy's wrapper).
impl<'a, Message: 'a> IntoElement<Message> for iced_widget::Text<'a, Theme> {
    fn foreground_color(&mut self, color: Color) {
        let widget = std::mem::replace(self, iced_widget::text(""));
        *self = widget.color(color);
    }

    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).boxed()
    }
}

impl<'a, Message: 'a> From<iced_widget::Text<'a, Theme>> for Component<'a, Message> {
    fn from(w: iced_widget::Text<'a, Theme>) -> Self {
        Component::new(w)
    }
}

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
impl<'a, T, Message> IntoElement<Message> for iced_widget::Slider<'a, T, Message, Theme>
where
    T: Copy + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).boxed()
    }
}

impl<'a, T, Message> From<iced_widget::Slider<'a, T, Message, Theme>> for Component<'a, Message>
where
    T: Copy + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn from(w: iced_widget::Slider<'a, T, Message, Theme>) -> Self {
        Component::new(w)
    }
}

impl<'a, T, Message> IntoElement<Message> for crate::slider::Slider<'a, T, Message, Theme>
where
    T: Copy + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).boxed()
    }
}

impl<'a, T, Message> From<crate::slider::Slider<'a, T, Message, Theme>> for Component<'a, Message>
where
    T: Copy + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn from(w: crate::slider::Slider<'a, T, Message, Theme>) -> Self {
        Component::new(w)
    }
}

// PickList<T>
impl<'a, T, L, V, Message> IntoElement<Message>
    for iced_widget::PickList<'a, T, L, V, Message, Theme>
where
    T: PartialEq + Clone + 'a,
    L: std::borrow::Borrow<[T]> + 'a,
    V: std::borrow::Borrow<T> + 'a,
    Message: Clone + 'a,
{
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).boxed()
    }
}

impl<'a, T, L, V, Message> From<iced_widget::PickList<'a, T, L, V, Message, Theme>>
    for Component<'a, Message>
where
    T: PartialEq + Clone + 'a,
    L: std::borrow::Borrow<[T]> + 'a,
    V: std::borrow::Borrow<T> + 'a,
    Message: Clone + 'a,
{
    fn from(w: iced_widget::PickList<'a, T, L, V, Message, Theme>) -> Self {
        Component::new(w)
    }
}

// ComboBox<T>
impl<'a, T, Message> IntoElement<Message> for iced_widget::ComboBox<'a, T, Message, Theme>
where
    T: std::fmt::Display + Clone + 'static,
    Message: Clone + 'a,
{
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).boxed()
    }
}

impl<'a, T, Message> From<iced_widget::ComboBox<'a, T, Message, Theme>> for Component<'a, Message>
where
    T: std::fmt::Display + Clone + 'static,
    Message: Clone + 'a,
{
    fn from(w: iced_widget::ComboBox<'a, T, Message, Theme>) -> Self {
        Component::new(w)
    }
}

// Cow<str> → our Element (via our Text, so size propagation works)
impl<'a, Message: 'a> From<std::borrow::Cow<'a, str>> for Component<'a, Message> {
    fn from(s: std::borrow::Cow<'a, str>) -> Self {
        Component::new(crate::text::Text::new(s))
    }
}
