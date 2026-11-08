//! ToggleGroup - shadcn/ui faithful port (Radix `ToggleGroup`).
//!
//! An iced `Component` that owns the pressed set. Its items come from a
//! closure (a [`Slot`] cannot be cloned, and the component rebuilds its
//! view), and it lays them out as one segmented control, or apart with
//! [`spacing`](ToggleGroup::spacing):
//!
//! ```ignore
//! use icedxdy::widget::toggle_group;
//!
//! // Uncontrolled, any subset pressed.
//! toggle_group(|| {
//!     [
//!         toggle_group::item("bold", icon::bold()),
//!         toggle_group::item("italic", icon::italic()),
//!     ]
//! })
//! .mode(toggle_group::Mode::Multiple)
//! .default_value(["bold"])
//! .style(toggle_group::Outline)
//!
//! // Controlled, at most one pressed.
//! toggle_group(|| [toggle_group::item("all", "All"), toggle_group::item("missed", "Missed")])
//!     .value([self.filter.as_str()])
//!     .on_value_change(Message::Filter)
//! ```
//!
//! Either way the group applies its [`Mode`] and publishes the complete new
//! pressed set.
//!
//! # Variants
//!
//! shadcn's `variant` prop is a style: `.style(toggle_group::Outline)` on
//! the group, or on one item to override the group's.

use iced_core::Length::{self, Shrink};
use iced_core::{Alignment, Element};
use iced_widget::button::StyleFn;
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedxdy_core::theme::button::Spec;
use icedxdy_core::theme::toggle;
use icedxdy_core::{Orientation, Theme};

use crate::button::{self, group};
use crate::compose::{Compose, Inherited, Slot};
use crate::{Button, Size};

pub use icedxdy_core::theme::toggle::{Default, Outline};

/// How many items can be pressed at once (Radix's `type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::default::Default)]
pub enum Mode {
    /// At most one; pressing it again releases it.
    #[default]
    Single,
    /// Any subset.
    Multiple,
}

impl Mode {
    /// The pressed set after `value` is pressed.
    fn press(self, pressed: &[String], value: &str) -> Vec<String> {
        let is_on = pressed.iter().any(|on| on == value);

        match (self, is_on) {
            (Mode::Single, true) => Vec::new(),
            (Mode::Single, false) => vec![value.to_owned()],
            (Mode::Multiple, true) => pressed.iter().filter(|on| *on != value).cloned().collect(),
            (Mode::Multiple, false) => {
                let mut pressed = pressed.to_vec();
                pressed.push(value.to_owned());
                pressed
            }
        }
    }
}

/// One toggle in a [`ToggleGroup`].
pub struct Item<'a, Message> {
    value: String,
    content: Slot<'a, Message>,
    disabled: bool,
    style: Option<Spec>,
}

/// Create a toggle group item for `value`, showing `content`.
pub fn item<'a, Message: 'a>(
    value: impl Into<String>,
    content: impl Into<Slot<'a, Message>>,
) -> Item<'a, Message> {
    Item {
        value: value.into(),
        content: content.into(),
        disabled: false,
        style: None,
    }
}

impl<'a, Message: 'a> Item<'a, Message> {
    /// Disable this item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Override the group's style for this item.
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = Some(style.into());
        self
    }
}

/// The closure that builds a [`ToggleGroup`]'s items.
type Items<'a, Message> = Box<dyn Fn() -> Vec<Item<'a, Message>> + 'a>;

/// A row (or column) of toggles sharing one style, size and pressed set.
pub struct ToggleGroup<'a, Message> {
    items: Items<'a, Message>,
    mode: Mode,
    value: Option<Vec<String>>,
    default_value: Vec<String>,
    on_value_change: Option<Box<dyn Fn(Vec<String>) -> Message + 'a>>,
    disabled: bool,
    style: Spec,
    size: Size,
    orientation: Orientation,
    spacing: f32,
    width: Length,
    inherited: Inherited,
}

/// The group's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// An item was pressed.
    Press(String),
    /// An item's content published an app message.
    App(Message),
}

/// The pressed set of an uncontrolled group.
#[derive(Debug, std::default::Default)]
pub struct State {
    value: Option<Vec<String>>,
}

/// Create an uncontrolled, single-mode [`ToggleGroup`] of the items
/// `items` builds.
pub fn toggle_group<'a, Message, I>(items: impl Fn() -> I + 'a) -> ToggleGroup<'a, Message>
where
    I: IntoIterator<Item = Item<'a, Message>>,
{
    ToggleGroup {
        items: Box::new(move || items().into_iter().collect()),
        mode: Mode::Single,
        value: None,
        default_value: Vec::new(),
        on_value_change: None,
        disabled: false,
        style: Default.into(),
        size: Size::Default,
        orientation: Orientation::Horizontal,
        spacing: 0.0,
        width: Shrink,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> ToggleGroup<'a, Message> {
    /// Set how many items can be pressed at once.
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// Control the group: `value` is always the pressed set, and a press
    /// only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.value = Some(value.into_iter().map(Into::into).collect());
        self
    }

    /// The pressed set an uncontrolled group starts with.
    pub fn default_value(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.default_value = value.into_iter().map(Into::into).collect();
        self
    }

    /// Set the message published with the complete new pressed set.
    pub fn on_value_change(mut self, on_change: impl Fn(Vec<String>) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Disable every item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the style of every item: a variant (`toggle_group::Outline`) or
    /// a tweaked `Spec`.
    pub fn style(mut self, style: impl Into<Spec>) -> Self {
        self.style = style.into();
        self
    }

    /// Set the size of every item.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Lay the items out in a row or a column.
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }

    /// Set the gap between items. At `0` the items join into one segmented
    /// control.
    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Set the group's width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    fn pressed<'s>(&'s self, state: &'s State) -> &'s [String] {
        self.value
            .as_deref()
            .or(state.value.as_deref())
            .unwrap_or(&self.default_value)
    }
}

impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer>
    for ToggleGroup<'a, Message>
{
    type State = State;
    type Event = Event<Message>;

    fn diff(&mut self, state: &mut State) {
        if self.value.is_none() && state.value.is_none() {
            state.value = Some(self.default_value.clone());
        }
    }

    fn update(
        &self,
        state: &mut State,
        event: Event<Message>,
        _renderer: &Renderer,
    ) -> Option<Message> {
        match event {
            Event::App(message) => Some(message),
            Event::Press(value) => {
                let pressed = self.mode.press(self.pressed(state), &value);

                if self.value.is_none() {
                    state.value = Some(pressed.clone());
                }

                self.on_value_change
                    .as_ref()
                    .map(|on_change| on_change(pressed))
            }
        }
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let pressed = self.pressed(state);
        let disabled = self.disabled || self.inherited.is_disabled();
        let size = self.size;

        let buttons: Vec<Button<'a, Event<Message>>> = (self.items)()
            .into_iter()
            .map(|item| {
                let is_on = pressed.contains(&item.value);
                let spec = item.style.unwrap_or_else(|| self.style.clone());
                let paint: StyleFn<'a, Theme> = Box::new(move |theme, status| {
                    let style = spec.resolve(theme, status);

                    if is_on {
                        toggle::on(theme, style).into()
                    } else {
                        style.into()
                    }
                });

                button::button(item.content.map(Event::App))
                    .size(size)
                    .translate(false)
                    .style(paint)
                    .on_press(Event::Press(item.value))
                    .disabled(disabled || item.disabled)
            })
            .collect();

        let element: Element<'a, Event<Message>, Theme, Renderer> = if self.spacing == 0.0 {
            group::group(buttons.into_iter().map(group::Item::from))
                .orientation(self.orientation)
                .width(self.width)
                .into()
        } else {
            let children = buttons.into_iter().map(Slot::from);

            match self.orientation {
                Orientation::Horizontal => crate::Row::with_children(children)
                    .spacing(self.spacing)
                    .align_y(Alignment::Center)
                    .width(self.width)
                    .into(),
                Orientation::Vertical => crate::Column::with_children(children)
                    .spacing(self.spacing)
                    .width(self.width)
                    .into(),
            }
        };

        iced_widget::container(element)
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for ToggleGroup<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}

impl<'a, Message: Clone + 'static> From<ToggleGroup<'a, Message>> for Slot<'a, Message> {
    fn from(group: ToggleGroup<'a, Message>) -> Self {
        Slot::new(group)
    }
}

impl<'a, Message: Clone + 'static> From<ToggleGroup<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(group: ToggleGroup<'a, Message>) -> Self {
        component::component(group).boxed()
    }
}
