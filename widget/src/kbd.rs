//! Kbd - shadcn/ui faithful port.
//!
//! A keyboard key: `text-xs font-medium` on a 20px muted cap. [`group`]
//! (or `kbd_group!`) lines several keys up with a 4px gap.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::kbd;
//!
//! kbd_group![kbd("⌘"), kbd("⇧"), kbd("⌥"), kbd("⌃")]
//! kbd_group![kbd("Ctrl"), "+", kbd("B")]
//!
//! // Inside tooltip content.
//! kbd("S").style(kbd::Tooltip)
//! ```

use iced_core::Alignment;
use iced_core::alignment::{Horizontal, Vertical};
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme::container::Spec;

use crate::compose::{Compose, Inherited, Slot};
use crate::row::Row;
use crate::{Size, Weight};

pub use icedxdy_core::theme::kbd::{Default, Tooltip};

/// Lay out keys in a kbd group.
///
/// ```ignore
/// kbd_group![kbd("Ctrl"), "+", kbd("B")]
/// ```
#[macro_export]
macro_rules! kbd_group {
    ($($child:expr),+ $(,)?) => {
        $crate::kbd::group([
            $(Into::<$crate::compose::Slot<'_, _>>::into($child)),+
        ])
    };
}

/// A keyboard key.
pub struct Kbd<'a, Message> {
    content: Slot<'a, Message>,
    style: Spec,
}

/// Create a [`Kbd`] showing `content`.
pub fn kbd<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Kbd<'a, Message> {
    Kbd {
        content: content.into(),
        style: Default.into(),
    }
}

impl<'a, Message: 'a> Kbd<'a, Message> {
    /// Set the style: `kbd::Tooltip` inside tooltip content, or a tweaked
    /// `Spec`.
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Kbd<'a, Message> {
    fn compose<'b>(mut self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        // `text-xs font-medium`, icons at `size-3`.
        self.content.inherit(
            &Inherited::default()
                .with_size(Size::Xs)
                .with_weight(Weight::Medium)
                .with_line_height(16.0),
        );

        // `h-5 min-w-5 px-1`, centered.
        crate::container(self.content)
            .height(20)
            .min_width(20)
            .padding([0, 4])
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .class(self.style)
            .into()
    }
}

impl<'a, Message: 'a> From<Kbd<'a, Message>> for Slot<'a, Message> {
    fn from(kbd: Kbd<'a, Message>) -> Self {
        Slot::new(kbd)
    }
}

impl<'a, Message: 'a> From<Kbd<'a, Message>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(kbd: Kbd<'a, Message>) -> Self {
        Box::new(kbd).compose()
    }
}

/// Lay `keys` out in a row with a 4px gap (`KbdGroup`).
pub fn group<'a, Message: 'a>(
    keys: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    Row::with_children(keys)
        .spacing(4)
        .align_y(Alignment::Center)
        .into()
}
