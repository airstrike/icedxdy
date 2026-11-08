//! Tabs — shadcn/ui faithful port (Radix `Tabs`).
//!
//! An iced `Component` that owns the selected value. Its content is a
//! closure receiving a [`Handle`], which builds the `TabsList`, its
//! triggers and the `TabsContent` panels wherever the layout puts them:
//!
//! ```ignore
//! use icedxdy::widget::{column, tabs};
//!
//! tabs(|t| {
//!     column![
//!         t.list([
//!             t.trigger("overview", "Overview"),
//!             t.trigger("analytics", "Analytics"),
//!         ]),
//!         t.content("overview", overview()),
//!         t.content("analytics", analytics()),
//!     ]
//!     .spacing(8)
//!     .into()
//! })
//! .default_value("overview")
//! // Controlled: .value(&self.tab).on_value_change(Message::TabChanged)
//! ```
//!
//! Only the active panel is mounted, as in Radix. Panels share no widget
//! state, so switching tabs resets whatever a panel kept in its own tree.

use iced_core::{Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;
use icedxdy_core::theme::tabs;

use crate::Size;
use crate::button::button;
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker::{self, Press};
use crate::row::Row;

/// A set of panels, one shown at a time, chosen by its triggers.
pub struct Tabs<'a, Message> {
    body: Body<'a, Message>,
    value: Option<String>,
    default_value: String,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    width: Length,
    inherited: Inherited,
}

/// The closure that builds a [`Tabs`]'s content.
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// What a [`Tabs`]'s content closure receives: the selected value, and the
/// builders for the list, its triggers and the panels.
#[derive(Debug, Clone)]
pub struct Handle {
    value: String,
}

/// The signal a trigger records when pressed: the value it selects.
#[derive(Debug, Clone)]
struct Select(String);

/// The tabs' private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// A trigger selected a value.
    Select(String),
    /// The content published an app message.
    App(Message),
}

/// The selected value of uncontrolled tabs.
#[derive(Debug, Default)]
pub struct State {
    value: Option<String>,
}

/// Create uncontrolled [`Tabs`] whose content is built by `body`.
pub fn tabs<'a, Message>(body: impl Fn(&Handle) -> Slot<'a, Message> + 'a) -> Tabs<'a, Message> {
    Tabs {
        body: Box::new(body),
        value: None,
        default_value: String::new(),
        on_value_change: None,
        width: Length::Shrink,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Tabs<'a, Message> {
    /// Control the tabs: `value` is always the selected one, and a trigger
    /// only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The value uncontrolled tabs start on.
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = value.into();
        self
    }

    /// Set the message published with the new value when a trigger selects
    /// another tab.
    pub fn on_value_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Set the width of the tabs.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
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

    /// The `TabsList`: the muted pill holding the triggers.
    pub fn list<'a, Message: 'a>(
        &self,
        triggers: impl IntoIterator<Item = Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        Container::new(Row::with_children(triggers))
            .padding([4, 3])
            .style(tabs::list)
            .into()
    }

    /// A `TabsTrigger` selecting `value`, labelled by `label`.
    pub fn trigger<'a, Message: Clone + 'a>(
        &self,
        value: impl Into<String>,
        label: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        let value = value.into();
        let is_active = value == self.value;

        // The list is `h-8`; triggers fill it at `text-sm`, so the label takes
        // the default text size before the compact button pushes its own.
        let mut label = label.into();
        label.inherit(&Inherited::default().with_size(Size::Default));

        Slot::new(Trigger {
            button: button(label)
                .size(Size::Xs)
                .translate(false)
                .style(Box::new(move |theme: &Theme, status| {
                    tabs::trigger(theme, status, is_active).into()
                })
                    as iced_widget::button::StyleFn<'a, Theme>),
            value,
        })
    }

    /// A `TabsContent` panel, shown only while `value` is selected.
    pub fn content<'a, Message: 'a>(
        &self,
        value: &str,
        child: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        if value == self.value {
            child.into()
        } else {
            Slot::from(iced_widget::Space::new())
        }
    }
}

/// A trigger button that records [`Select`] when pressed.
struct Trigger<'a, Message> {
    button: crate::Button<'a, Message>,
    value: String,
}

impl<'a, Message: Clone + 'a> Compose<Message> for Trigger<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        Compose::inherit(&mut self.button, from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Trigger { button, value } = *self;

        marker::marker(Slot::from(button).compose(), Select(value), Press::Captured).into()
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Tabs<'a, Message> {
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
        };

        let mut body = (self.body)(&handle);
        body.inherit(&self.inherited);

        let body = iced_widget::container(body.compose().map(Event::App)).width(self.width);

        marker::owner(body.boxed(), |Select(value)| Event::Select(value))
    }
}

impl<'a, Message: 'static> Compose<Message> for Tabs<'a, Message> {
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

impl<'a, Message: 'static> From<Tabs<'a, Message>> for Slot<'a, Message> {
    fn from(tabs: Tabs<'a, Message>) -> Self {
        Slot::new(tabs)
    }
}

impl<'a, Message: 'static> From<Tabs<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(tabs: Tabs<'a, Message>) -> Self {
        component::component(tabs).boxed()
    }
}
