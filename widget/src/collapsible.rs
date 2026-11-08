//! Collapsible — shadcn/ui faithful port (Radix `Collapsible`).
//!
//! An iced `Component` that owns its open state. Its content is a closure
//! receiving a [`Handle`], which places the trigger and the content wherever
//! the layout puts them, like Radix's `CollapsibleTrigger asChild` and
//! `CollapsibleContent`:
//!
//! ```ignore
//! use icedxdy::widget::{button, collapsible, column, row, text};
//!
//! collapsible(|c| {
//!     column![
//!         row![
//!             text::small("Order #4189").semibold(),
//!             c.trigger(button(icon::chevrons_up_down()).square().style(button::Ghost)),
//!         ],
//!         c.content(column![address(), items()].spacing(8)),
//!     ]
//!     .spacing(8)
//! })
//! // Uncontrolled: .default_open(false), the default.
//! // Controlled:   .open(self.is_open).on_open_change(Message::OpenChanged)
//! ```
//!
//! The closure is built in the app's message type, so the content's own
//! messages flow out unchanged; the component maps it once at its boundary
//! (`Message: 'static`). Cascading props the collapsible inherits reach
//! the closure's content.

use iced_core::{Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;

use crate::compose::{Compose, Inherited, Slot};
use crate::marker::{self, Press};

/// The closure that builds a [`Collapsible`]'s content.
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// A disclosure that shows its content while open.
pub struct Collapsible<'a, Message> {
    body: Body<'a, Message>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    width: Length,
    inherited: Inherited,
}

/// What a [`Collapsible`]'s content closure receives: whether it is open,
/// and the wrappers for its trigger and content.
#[derive(Debug, Clone, Copy)]
pub struct Handle {
    is_open: bool,
}

/// The signal a trigger records when pressed.
#[derive(Debug, Clone, Copy)]
struct Toggle;

/// The collapsible's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// A trigger was pressed.
    Toggle,
    /// The content published an app message.
    App(Message),
}

/// The open state of an uncontrolled collapsible.
#[derive(Debug, Default)]
pub struct State {
    open: Option<bool>,
}

/// Create a closed, uncontrolled [`Collapsible`] whose content is built by
/// `body`.
pub fn collapsible<'a, Message>(
    body: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> Collapsible<'a, Message> {
    Collapsible {
        body: Box::new(body),
        open: None,
        default_open: false,
        on_open_change: None,
        width: Length::Shrink,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Collapsible<'a, Message> {
    /// Control the collapsible: it is open exactly when `open` is, and a
    /// trigger only publishes [`on_open_change`](Self::on_open_change).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Whether an uncontrolled collapsible starts open.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Set the message published with the new open state when a trigger is
    /// pressed.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }

    /// Set the width of the collapsible.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    fn is_open(&self, state: &State) -> bool {
        self.open.or(state.open).unwrap_or(self.default_open)
    }
}

impl Handle {
    /// Whether the collapsible is open, for content that changes with it (a
    /// chevron that flips).
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Make `child` toggle the collapsible when pressed, like
    /// `<CollapsibleTrigger asChild>`. The child keeps its own handlers.
    pub fn trigger<'a, Message: 'a>(
        &self,
        child: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        Slot::new(Trigger {
            child: child.into(),
        })
    }

    /// Show `child` only while the collapsible is open, like
    /// `<CollapsibleContent>`.
    pub fn content<'a, Message: 'a>(
        &self,
        child: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        if self.is_open {
            child.into()
        } else {
            Slot::from(iced_widget::Space::new())
        }
    }
}

/// A child that records [`Toggle`] when pressed.
struct Trigger<'a, Message> {
    child: Slot<'a, Message>,
}

impl<'a, Message: 'a> Compose<Message> for Trigger<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.child.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        marker::marker(self.child.compose(), Toggle, Press::Captured).into()
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Collapsible<'a, Message> {
    type State = State;
    type Event = Event<Message>;

    fn diff(&mut self, state: &mut State) {
        if self.open.is_none() && state.open.is_none() {
            state.open = Some(self.default_open);
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
            Event::Toggle => {
                let next = !self.is_open(state);

                if self.open.is_none() {
                    state.open = Some(next);
                }

                self.on_open_change
                    .as_ref()
                    .map(|on_change| on_change(next))
            }
        }
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let handle = Handle {
            is_open: self.is_open(state),
        };

        let mut body = (self.body)(&handle);
        body.inherit(&self.inherited);

        let body = iced_widget::container(body.compose().map(Event::App)).width(self.width);

        marker::owner(body.boxed(), |Toggle| Event::Toggle)
    }
}

impl<'a, Message: 'static> Compose<Message> for Collapsible<'a, Message> {
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

impl<'a, Message: 'static> From<Collapsible<'a, Message>> for Slot<'a, Message> {
    fn from(collapsible: Collapsible<'a, Message>) -> Self {
        Slot::new(collapsible)
    }
}

impl<'a, Message: 'static> From<Collapsible<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(collapsible: Collapsible<'a, Message>) -> Self {
        component::component(collapsible).boxed()
    }
}
