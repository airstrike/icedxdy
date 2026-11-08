//! Composition: the [`Compose`] trait, the type-erased [`Slot`] and the
//! [`Inherited`] props a parent cascades into its children.
//!
//! Every icedxdy widget is a builder that implements [`Compose`]. A parent
//! holds its children as [`Slot`]s, pushes its cascading props into them with
//! [`Slot::inherit`] and only then turns them into iced [`Element`]s with
//! [`Slot::compose`]. That ordering is what lets `button!["Fork", icon]` size
//! its text and icon from the button's [`Size`] without the caller repeating
//! it.
//!
//! Each [`Compose`] implementor also provides `From<Self> for Slot`, so APIs
//! take `impl Into<Slot>`. The stock iced widgets icedxdy composes with are
//! listed at the bottom of this module; any other widget joins through its
//! [`Element`] (`.boxed()`). Neither can be a blanket impl. iced implements
//! `Widget` for `&str`, so a blanket over every iced widget would claim string
//! children and compose them as raw iced text, outside the cascade, and
//! overlap with `From<&str> for Slot`. A blanket `From<C: Compose>` overlaps
//! with `From<Slot> for Slot` and `From<&str>`, because a downstream crate
//! may implement `Compose<ItsMessage>` for either.

use crate::Size;
use crate::Weight;
use crate::text::Text;
use iced_core::Element;
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;

/// The props a parent cascades to its descendants, like inherited CSS.
///
/// An explicit setting on a child always wins over an inherited one, so a
/// child keeps the first value it receives for each field and ignores the
/// rest. `disabled` is the exception: a disabled ancestor disables every
/// descendant, like a disabled `<fieldset>`.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Inherited {
    size: Option<Size>,
    weight: Option<Weight>,
    line_height: Option<f32>,
    disabled: bool,
}

impl Inherited {
    /// Cascade a [`Size`] (text size, control height).
    pub fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Cascade a font [`Weight`].
    pub fn with_weight(mut self, weight: impl Into<Weight>) -> Self {
        self.weight = Some(weight.into());
        self
    }

    /// Cascade an absolute line height in logical pixels (nova `leading-4`
    /// is `16.0`).
    pub fn with_line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(line_height);
        self
    }

    /// Cascade the disabled state.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The cascaded size, if any.
    pub fn size(&self) -> Option<Size> {
        self.size
    }

    /// The cascaded font weight, if any.
    pub fn weight(&self) -> Option<Weight> {
        self.weight
    }

    /// The cascaded absolute line height, if any.
    pub fn line_height(&self) -> Option<f32> {
        self.line_height
    }

    /// Whether an ancestor is disabled.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Fill the fields `self` leaves unset from `fallback`. `self` wins,
    /// `disabled` is the union of both.
    pub fn or(self, fallback: Inherited) -> Inherited {
        Inherited {
            size: self.size.or(fallback.size),
            weight: self.weight.or(fallback.weight),
            line_height: self.line_height.or(fallback.line_height),
            disabled: self.disabled || fallback.disabled,
        }
    }
}

/// A builder that receives cascading props from its parent and turns into an
/// iced [`Element`].
///
/// The trait takes no lifetime so that `Box<dyn Compose + 'a>` (and therefore
/// [`Slot<'a>`]) is covariant in `'a`, like [`Element`]: the produced
/// element's lifetime is picked by the caller through `Self: 'a`.
pub trait Compose<Message, T = Theme, R = Renderer> {
    /// Receive the props an ancestor cascades. Implementors keep their own
    /// explicit settings and fill only what is unset; containers forward
    /// `from` to their children.
    fn inherit(&mut self, _from: &Inherited) {}

    /// Turn into an iced [`Element`].
    fn compose<'a>(self: Box<Self>) -> Element<'a, Message, T, R>
    where
        Self: 'a;
}

/// A type-erased [`Compose`]: the child every icedxdy widget slot carries.
///
/// Covariant in `'a` (just like [`Element`]) so children of different
/// lifetimes can sit side by side in [`column!`](crate::column!) and
/// [`row!`](crate::row!).
pub struct Slot<'a, Message, T = Theme, R = Renderer>(Box<dyn Compose<Message, T, R> + 'a>);

impl<'a, Message: 'a, T: 'a, R: 'a> Slot<'a, Message, T, R> {
    /// Wrap any [`Compose`] implementor.
    pub fn new(child: impl Compose<Message, T, R> + 'a) -> Self {
        Self(Box::new(child))
    }

    /// Push cascading props into the child.
    pub fn inherit(&mut self, from: &Inherited) {
        self.0.inherit(from);
    }

    /// Turn the child into an iced [`Element`].
    pub fn compose(self) -> Element<'a, Message, T, R> {
        self.0.compose()
    }

    /// Map the message type, like [`Element::map`](iced_widget::Widget::map).
    ///
    /// The mapping is deferred to [`compose`](Self::compose), so props pushed
    /// into the mapped slot still reach the child.
    pub fn map<B>(self, f: impl Fn(Message) -> B + 'a) -> Slot<'a, B, T, R>
    where
        Message: 'a,
        B: 'static,
        T: 'static,
        R: iced_core::Renderer + 'static,
    {
        Slot::new(Mapped {
            slot: self,
            mapper: Box::new(f),
        })
    }
}

/// A [`Slot`] whose messages are mapped once it composes.
struct Mapped<'a, A, B, T, R> {
    slot: Slot<'a, A, T, R>,
    mapper: Box<dyn Fn(A) -> B + 'a>,
}

impl<'a, A, B, T, R> Compose<B, T, R> for Mapped<'a, A, B, T, R>
where
    A: 'a,
    B: 'static,
    T: 'static,
    R: iced_core::Renderer + 'static,
{
    fn inherit(&mut self, from: &Inherited) {
        self.slot.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, B, T, R>
    where
        Self: 'b,
    {
        let Mapped { slot, mapper } = *self;
        slot.compose().map(mapper).boxed()
    }
}

impl<'a, Message: 'a, T: 'a, R: 'a> From<Slot<'a, Message, T, R>> for Element<'a, Message, T, R> {
    fn from(slot: Slot<'a, Message, T, R>) -> Self {
        slot.compose()
    }
}

impl<'a, Message: 'a> From<&'a str> for Slot<'a, Message> {
    fn from(s: &'a str) -> Self {
        Slot::new(Text::new(s))
    }
}

impl<'a, Message: 'a> From<String> for Slot<'a, Message> {
    fn from(s: String) -> Self {
        Slot::new(Text::new(s))
    }
}

impl<'a, Message: 'a> From<std::borrow::Cow<'a, str>> for Slot<'a, Message> {
    fn from(s: std::borrow::Cow<'a, str>) -> Self {
        Slot::new(Text::new(s))
    }
}

impl<'a, Message: 'a, T: 'a, R: 'a> From<Element<'a, Message, T, R>> for Slot<'a, Message, T, R> {
    fn from(element: Element<'a, Message, T, R>) -> Self {
        Slot::new(element)
    }
}

impl<'a, Message, T, R> Compose<Message, T, R> for Element<'a, Message, T, R> {
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, T, R>
    where
        Self: 'b,
    {
        // `Self: 'b` gives `'a: 'b`; Element is covariant in its lifetime.
        *self
    }
}

/// Implement [`Compose`] and `From<_> for Slot` for a stock iced widget: no
/// props to receive, boxed on compose.
macro_rules! compose_widget {
    ($ty:ty) => {
        compose_widget!($ty where);
    };
    ($ty:ty where $($bounds:tt)*) => {
        impl<'a, Message: 'a $($bounds)*> Compose<Message> for $ty {
            fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
            where
                Self: 'b,
            {
                (*self).boxed()
            }
        }

        impl<'a, Message: 'a $($bounds)*> From<$ty> for Slot<'a, Message> {
            fn from(widget: $ty) -> Self {
                Slot::new(widget)
            }
        }
    };
}

type El<'a, Message> = Element<'a, Message, Theme, Renderer>;

compose_widget!(iced_widget::Button<'a, Message, El<'a, Message>, Theme> where + Clone);
compose_widget!(iced_widget::Checkbox<'a, Message, Theme, Renderer> where + Clone);
compose_widget!(iced_widget::Column<El<'a, Message>> where + Clone);
compose_widget!(iced_widget::Container<'a, El<'a, Message>, Theme> where + Clone);
compose_widget!(iced_widget::Float<'a, El<'a, Message>, Theme> where + Clone);
compose_widget!(iced_widget::Grid<El<'a, Message>> where + Clone);
compose_widget!(iced_widget::MouseArea<'a, Message, El<'a, Message>> where + Clone);
compose_widget!(iced_widget::Pin<El<'a, Message>> where + Clone);
compose_widget!(iced_widget::ProgressBar<'a, Theme>);
compose_widget!(iced_widget::QRCode<'a, Theme>);
compose_widget!(iced_widget::Radio<'a, Message, Theme> where + Clone);
compose_widget!(iced_widget::Row<El<'a, Message>> where + Clone);
compose_widget!(iced_widget::Rule<'a, Theme>);
compose_widget!(iced_widget::Scrollable<'a, Message, El<'a, Message>, Theme> where + Clone);
compose_widget!(iced_widget::Sensor<'a, (), Message, El<'a, Message>> where + Clone);
compose_widget!(iced_widget::Space);
compose_widget!(iced_widget::Stack<El<'a, Message>> where + Clone);
#[cfg(feature = "svg")]
compose_widget!(iced_widget::Svg<'a, Theme>);
compose_widget!(iced_widget::Text<'a, Theme>);
compose_widget!(iced_widget::TextInput<'a, Message, Theme> where + Clone);
compose_widget!(iced_widget::Toggler<'a, Message, Theme> where + Clone);
compose_widget!(iced_widget::Tooltip<'a, El<'a, Message>, El<'a, Message>, Theme> where + Clone);
compose_widget!(iced_widget::text::Rich<'a, Message, Message, Theme> where + Clone);

impl<'a, T, Message> Compose<Message> for iced_widget::Slider<'a, T, Message, Theme>
where
    T: Copy + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn compose<'b>(self: Box<Self>) -> El<'b, Message>
    where
        Self: 'b,
    {
        (*self).boxed()
    }
}

impl<'a, T, Message> Compose<Message> for crate::slider::Slider<'a, T, Message, Theme>
where
    T: Copy + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn inherit(&mut self, from: &Inherited) {
        self.is_disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> El<'b, Message>
    where
        Self: 'b,
    {
        (*self).boxed()
    }
}

impl<'a, T, L, V, Message> Compose<Message> for iced_widget::PickList<'a, T, L, V, Message, Theme>
where
    T: PartialEq + Clone + 'a,
    L: std::borrow::Borrow<[T]> + 'a,
    V: std::borrow::Borrow<T> + 'a,
    Message: Clone + 'a,
{
    fn compose<'b>(self: Box<Self>) -> El<'b, Message>
    where
        Self: 'b,
    {
        (*self).boxed()
    }
}

impl<'a, T, Message> Compose<Message> for iced_widget::ComboBox<'a, T, Message, Theme>
where
    T: std::fmt::Display + Clone + 'static,
    Message: Clone + 'a,
{
    fn compose<'b>(self: Box<Self>) -> El<'b, Message>
    where
        Self: 'b,
    {
        (*self).boxed()
    }
}

impl<'a, T, Message> From<iced_widget::Slider<'a, T, Message, Theme>> for Slot<'a, Message>
where
    T: Copy + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn from(widget: iced_widget::Slider<'a, T, Message, Theme>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, T, Message> From<crate::slider::Slider<'a, T, Message, Theme>> for Slot<'a, Message>
where
    T: Copy + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
{
    fn from(widget: crate::slider::Slider<'a, T, Message, Theme>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, T, L, V, Message> From<iced_widget::PickList<'a, T, L, V, Message, Theme>>
    for Slot<'a, Message>
where
    T: PartialEq + Clone + 'a,
    L: std::borrow::Borrow<[T]> + 'a,
    V: std::borrow::Borrow<T> + 'a,
    Message: Clone + 'a,
{
    fn from(widget: iced_widget::PickList<'a, T, L, V, Message, Theme>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, T, Message> From<iced_widget::ComboBox<'a, T, Message, Theme>> for Slot<'a, Message>
where
    T: std::fmt::Display + Clone + 'static,
    Message: Clone + 'a,
{
    fn from(widget: iced_widget::ComboBox<'a, T, Message, Theme>) -> Self {
        Slot::new(widget)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::Cell;
    use std::rc::Rc;

    /// Records the size it inherits.
    struct Probe(Rc<Cell<Option<Size>>>);

    impl<M: 'static> Compose<M> for Probe {
        fn inherit(&mut self, from: &Inherited) {
            if self.0.get().is_none() {
                self.0.set(from.size());
            }
        }

        fn compose<'a>(self: Box<Self>) -> El<'a, M>
        where
            Self: 'a,
        {
            iced_widget::Space::new().boxed()
        }
    }

    #[test]
    fn explicit_props_win_and_disabled_is_a_union() {
        let own = Inherited::default().with_size(Size::Sm);
        let parent = Inherited::default()
            .with_size(Size::Lg)
            .with_weight(Weight::Bold)
            .with_disabled(true);

        let merged = own.or(parent);

        assert_eq!(merged.size(), Some(Size::Sm));
        assert_eq!(merged.weight(), Some(Weight::Bold));
        assert!(merged.is_disabled());
        assert!(
            Inherited::default()
                .with_disabled(true)
                .or(Inherited::default())
                .is_disabled()
        );
    }

    #[test]
    fn mapped_slots_still_receive_the_cascade() {
        let seen = Rc::new(Cell::new(None));
        let slot: Slot<'_, u8> = Slot::new(Probe(seen.clone()));
        let mut mapped: Slot<'_, u16> = slot.map(u16::from);

        mapped.inherit(&Inherited::default().with_size(Size::Xs));
        let _ = mapped.compose();

        assert_eq!(seen.get(), Some(Size::Xs));
    }
}
