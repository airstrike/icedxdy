//! Slider — vendored iced slider with nova cursor and hover-ring behavior.

use std::ops::RangeInclusive;

pub(crate) mod element;
pub mod widget;

pub use widget::{Catalog, Handle, HandleShape, Rail, Ring, Slider, Status, Style, StyleFn};

/// Create an uncontrolled, nova-styled slider over `range`. Use
/// [`value`](Slider::value) and [`on_value_change`](Slider::on_value_change)
/// to control it, or [`default_value`](Slider::default_value) to seed it.
pub fn slider<'a, T, Message>(
    range: RangeInclusive<T>,
) -> Slider<'a, T, Message, icedxdy_core::Theme>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
{
    Slider::new(range)
}
