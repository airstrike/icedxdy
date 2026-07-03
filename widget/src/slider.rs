//! Slider — vendored iced slider with nova cursor and hover-ring behavior.

use std::ops::RangeInclusive;

pub(crate) mod element;
pub mod widget;

pub use widget::{Catalog, Handle, HandleShape, Rail, Ring, Slider, Status, Style, StyleFn};

/// Create a nova-styled slider.
pub fn slider<'a, T, Message>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> Slider<'a, T, Message, icedxdy_core::Theme>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
{
    Slider::new(range, value, on_change)
}
