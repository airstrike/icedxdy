//! RadioGroup - shadcn/ui faithful port (Radix `RadioGroup`).
//!
//! An iced `Component` that owns the selected value. Its content is a
//! closure receiving a [`Handle`], whose [`item`](Handle::item) builds each
//! `RadioGroupItem` wherever the layout puts it; a [`field`](crate::field)
//! around an item links its label:
//!
//! ```ignore
//! use icedxdy::widget::{column, field, radio_group};
//! use icedxdy::Horizontal;
//!
//! radio_group(|g| {
//!     column![
//!         field(g.item("default")).orientation(Horizontal).label("Default"),
//!         field(g.item("comfortable")).orientation(Horizontal).label("Comfortable"),
//!     ]
//!     .spacing(8)
//!     .into()
//! })
//! .default_value("comfortable")
//! // Controlled: .value(&self.density).on_value_change(Message::Density)
//! ```

use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Border, Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::radio::Status;
use iced_widget::{Renderer, Space, Widget, container};
use icedxdy_core::Theme;
use icedxdy_core::theme::radio;

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker::{self, Press};

/// A set of radio items, one checked at a time.
pub struct Group<'a, Message> {
    body: Body<'a, Message>,
    value: Option<String>,
    default_value: String,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    disabled: bool,
    inherited: Inherited,
}

/// The closure that builds a [`Group`]'s content.
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// What a [`Group`]'s content closure receives: the selected value, and
/// the builder for its items.
#[derive(Debug, Clone)]
pub struct Handle {
    value: String,
    disabled: bool,
}

/// The signal an item records when chosen: its value.
#[derive(Debug, Clone)]
struct Select(String);

/// The group's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// An item was chosen.
    Select(String),
    /// The content published an app message.
    App(Message),
}

/// The selected value of an uncontrolled group.
#[derive(Debug, Default)]
pub struct State {
    value: Option<String>,
}

/// Create an uncontrolled [`Group`] whose content is built by `body`.
pub fn group<'a, Message>(body: impl Fn(&Handle) -> Slot<'a, Message> + 'a) -> Group<'a, Message> {
    Group {
        body: Box::new(body),
        value: None,
        default_value: String::new(),
        on_value_change: None,
        disabled: false,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Group<'a, Message> {
    /// Control the group: `value` is always the checked item, and choosing
    /// another only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The value an uncontrolled group starts on.
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = value.into();
        self
    }

    /// Set the message published with the new value when another item is
    /// chosen.
    pub fn on_value_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Disable every item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn selected<'s>(&'s self, state: &'s State) -> &'s str {
        self.value
            .as_deref()
            .or(state.value.as_deref())
            .unwrap_or(&self.default_value)
    }
}

impl Handle {
    /// The selected value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// A `RadioGroupItem` for `value`.
    pub fn item(&self, value: impl Into<String>) -> Item {
        let value = value.into();

        Item {
            is_checked: value == self.value,
            value,
            disabled: self.disabled,
        }
    }
}

/// One radio of a [`Group`]: a 16px circle, filled while checked.
#[derive(Debug, Clone)]
pub struct Item {
    value: String,
    is_checked: bool,
    disabled: bool,
}

impl Item {
    /// Disable this item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled |= disabled;
        self
    }
}

impl<Message: 'static> Compose<Message> for Item {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Item {
            value,
            is_checked,
            disabled,
        } = *self;

        let status = Status::Active {
            is_selected: is_checked,
        };
        let paint = move |theme: &Theme| {
            if disabled {
                radio::disabled(theme, status)
            } else {
                radio::default(theme, status)
            }
        };

        // `size-2 rounded-full bg-primary-foreground`.
        let dot: Slot<'b, Message> = if is_checked {
            Container::new(Space::new())
                .width(8)
                .height(8)
                .class(Box::new(move |theme: &Theme| {
                    container::Style::default()
                        .background(paint(theme).dot_color)
                        .border(Border::default().rounded(4))
                }) as container::StyleFn<'static, Theme>)
                .into()
        } else {
            Slot::from(Space::new())
        };

        // `size-4 rounded-full border`.
        let circle: Element<'b, Message, Theme, Renderer> = Container::new(dot)
            .width(16)
            .height(16)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .class(Box::new(move |theme: &Theme| {
                let style = paint(theme);

                container::Style::default()
                    .background(style.background)
                    .border(Border {
                        color: style.border_color,
                        width: style.border_width,
                        radius: 8.0.into(),
                    })
            }) as container::StyleFn<'static, Theme>)
            .into();

        if disabled {
            circle
        } else {
            marker::marker(circle, Select(value), Press::Any)
                .control(is_checked)
                .into()
        }
    }
}

impl<'a, Message: 'static> From<Item> for Slot<'a, Message> {
    fn from(item: Item) -> Self {
        Slot::new(item)
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Group<'a, Message> {
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
            Event::Select(value) => {
                if value == self.selected(state) {
                    return None;
                }

                if self.value.is_none() {
                    state.value = Some(value.clone());
                }

                self.on_value_change
                    .as_ref()
                    .map(|on_change| on_change(value))
            }
        }
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let handle = Handle {
            value: self.selected(state).to_owned(),
            disabled: self.disabled,
        };

        let mut body = (self.body)(&handle);
        body.inherit(&self.inherited);

        let body = iced_widget::container(body.compose().map(Event::App)).width(Length::Shrink);

        marker::owner(body.boxed(), |Select(value)| Event::Select(value))
    }
}

impl<'a, Message: 'static> Compose<Message> for Group<'a, Message> {
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

impl<'a, Message: 'static> From<Group<'a, Message>> for Slot<'a, Message> {
    fn from(group: Group<'a, Message>) -> Self {
        Slot::new(group)
    }
}

impl<'a, Message: 'static> From<Group<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(group: Group<'a, Message>) -> Self {
        component::component(group).boxed()
    }
}
