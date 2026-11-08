//! NativeSelect - shadcn/ui port over iced's `pick_list`.
//!
//! A bordered field showing the chosen option; pressing it opens the list
//! of options. As with an HTML `<select>`, the first option shows when
//! nothing else is chosen, so a leading option with an empty value works as
//! the prompt:
//!
//! ```ignore
//! use icedxdy::widget::native_select;
//!
//! native_select([
//!     native_select::item("", "Select status"),
//!     native_select::item("todo", "Todo"),
//!     native_select::item("done", "Done"),
//! ])
//! // Controlled: .value(&self.status).on_value_change(Message::Status)
//! ```
//!
//! # Variants
//!
//! `aria-invalid` is a style: `.style(native_select::Invalid)`.

use std::fmt;
use std::rc::Rc;

use iced_core::{Element, Padding};
use iced_widget::component::{self, Component};
use iced_widget::pick_list::{Status, Style, StyleFn};
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;

use crate::Size;
use crate::compose::{Compose, Inherited, Slot};

pub use icedxdy_core::theme::native_select::{Default, Invalid};

/// One choice of a [`NativeSelect`] (`NativeSelectOption`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    value: String,
    label: String,
}

/// Create an [`Item`] for `value`, shown as `label`.
pub fn item(value: impl Into<String>, label: impl Into<String>) -> Item {
    Item {
        value: value.into(),
        label: label.into(),
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label)
    }
}

/// A shared style, cloned into every view the component builds.
type Paint<'a> = Rc<dyn Fn(&Theme, Status) -> Style + 'a>;

/// A field that picks one of a list of options.
pub struct NativeSelect<'a, Message> {
    items: Vec<Item>,
    value: Option<String>,
    default_value: Option<String>,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    disabled: bool,
    size: Size,
    style: Paint<'a>,
}

/// The chosen value of an uncontrolled select.
#[derive(Debug, std::default::Default)]
pub struct State {
    value: Option<String>,
}

/// Create an uncontrolled [`NativeSelect`] over `items`, showing the first.
pub fn native_select<'a, Message>(
    items: impl IntoIterator<Item = Item>,
) -> NativeSelect<'a, Message> {
    let style: StyleFn<'a, Theme> = Default.into();

    NativeSelect {
        items: items.into_iter().collect(),
        value: None,
        default_value: None,
        on_value_change: None,
        disabled: false,
        size: Size::Default,
        style: Rc::from(style),
    }
}

impl<'a, Message> NativeSelect<'a, Message> {
    /// Control the select: it always shows `value`, and choosing another
    /// option only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The value an uncontrolled select starts on (the first option's by
    /// default).
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Set the message published with the newly chosen value.
    pub fn on_value_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Disable the select.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the size: `Sm` (28px) or `Default` (32px).
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the style: `native_select::Invalid`, or any pick list style.
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self {
        self.style = Rc::from(style.into());
        self
    }

    fn selected<'s>(&'s self, state: &'s State) -> Option<&'s Item> {
        let value = self
            .value
            .as_deref()
            .or(state.value.as_deref())
            .or(self.default_value.as_deref());

        value
            .and_then(|value| self.items.iter().find(|item| item.value == value))
            .or(self.items.first())
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for NativeSelect<'a, Message> {
    type State = State;
    type Event = Item;

    fn update(&self, state: &mut State, item: Item, _renderer: &Renderer) -> Option<Message> {
        if self.selected(state) == Some(&item) {
            return None;
        }

        if self.value.is_none() {
            state.value = Some(item.value.clone());
        }

        self.on_value_change
            .as_ref()
            .map(|on_change| on_change(item.value))
    }

    fn view(&self, state: &State) -> impl Widget<Item, Theme, Renderer> + 'a {
        let selected = self.selected(state).cloned();
        let height = self.size.height();
        // `py-1 pl-2.5` around a `text-sm` line, centered in the
        // bordered field's height.
        let vertical = ((height - 2.0 - 18.0) / 2.0).max(0.0);
        let style = Rc::clone(&self.style);

        let pick_list = iced_widget::pick_list(selected, self.items.clone(), Item::to_string)
            .text_size(self.size.text_size())
            .padding(Padding {
                top: vertical,
                bottom: vertical,
                left: 10.0,
                right: 10.0,
            })
            .style(move |theme: &Theme, status| style(theme, status));

        if self.disabled {
            pick_list
        } else {
            pick_list.on_select(|item| item)
        }
    }
}

impl<'a, Message: 'static> Compose<Message> for NativeSelect<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}

impl<'a, Message: 'static> From<NativeSelect<'a, Message>> for Slot<'a, Message> {
    fn from(select: NativeSelect<'a, Message>) -> Self {
        Slot::new(select)
    }
}

impl<'a, Message: 'static> From<NativeSelect<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(select: NativeSelect<'a, Message>) -> Self {
        component::component(select).boxed()
    }
}
