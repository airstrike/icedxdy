//! SidebarProvider — owns whether the sidebar is open.
//!
//! An iced `Component` around the whole layout, sidebar and inset alike, so
//! a [`trigger`](super::trigger) anywhere inside it toggles the sidebar, as
//! does `Cmd+B` (`Ctrl+B` elsewhere), shadcn's keyboard shortcut. The body
//! closure receives a [`Handle`] that sizes the sidebar for the current
//! state:
//!
//! ```ignore
//! use icedxdy::widget::{column, row, sidebar};
//!
//! sidebar::provider(|s| {
//!     row![
//!         s.sidebar(sidebar::fixed(app_sidebar())),
//!         sidebar::main(column![sidebar::trigger(), page()]),
//!     ]
//!     .into()
//! })
//! // Uncontrolled and open by default; controlled:
//! // .open(self.sidebar_open).on_open_change(Message::SidebarOpen)
//! ```

use iced_core::Length::Fill;
use iced_core::keyboard;
use iced_core::{Element, Rectangle, mouse};
use iced_widget::component::{self, Component};
use iced_widget::{Action, Renderer, Widget};
use icedxdy_core::Theme;

use super::state::{Collapsible, WIDTH, WIDTH_ICON};
use super::trigger::Toggle;
use crate::compose::{Compose, Inherited, Slot};
use crate::marker;

/// Owns the open state of the sidebar inside it.
pub struct Provider<'a, Message> {
    body: Body<'a, Message>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    collapsible: Collapsible,
    inherited: Inherited,
}

/// The closure that builds a [`Provider`]'s layout.
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// What a [`Provider`]'s closure receives: whether the sidebar is open, and
/// the wrapper that sizes it.
#[derive(Debug, Clone, Copy)]
pub struct Handle {
    is_open: bool,
    collapsible: Collapsible,
}

/// The provider's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// A trigger or the shortcut toggled the sidebar.
    Toggle,
    /// The layout published an app message.
    App(Message),
}

/// The open state of an uncontrolled provider.
#[derive(Debug, Default)]
pub struct State {
    open: Option<bool>,
}

/// Create an uncontrolled [`Provider`], open by default, whose layout is
/// built by `body`.
pub fn provider<'a, Message>(
    body: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> Provider<'a, Message> {
    Provider {
        body: Box::new(body),
        open: None,
        default_open: true,
        on_open_change: None,
        collapsible: Collapsible::default(),
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Provider<'a, Message> {
    /// Control the sidebar: it is open exactly when `open` is, and triggers
    /// only publish [`on_open_change`](Self::on_open_change).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Whether an uncontrolled sidebar starts open (`true` by default).
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Set the message published with the new open state when the sidebar
    /// is toggled.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }

    /// How the sidebar collapses (shadcn's `collapsible` on `Sidebar`).
    pub fn collapsible(mut self, collapsible: Collapsible) -> Self {
        self.collapsible = collapsible;
        self
    }

    fn is_open(&self, state: &State) -> bool {
        self.open.or(state.open).unwrap_or(self.default_open)
    }
}

impl Handle {
    /// Whether the sidebar is open.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// The sidebar's width for the current state.
    pub fn width(&self) -> f32 {
        match (self.is_open, self.collapsible) {
            (true, _) | (false, Collapsible::None) => WIDTH,
            (false, Collapsible::Icon) => WIDTH_ICON,
            (false, Collapsible::Offcanvas) => 0.0,
        }
    }

    /// Size `sidebar` for the current state: its full width while open,
    /// clipped to its icons or hidden while collapsed.
    pub fn sidebar<'a, Message: 'a>(
        &self,
        sidebar: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        let width = self.width();

        if width == 0.0 {
            return Slot::from(iced_widget::Space::new());
        }

        Slot::new(Sized {
            sidebar: sidebar.into(),
            width,
        })
    }
}

/// A sidebar clipped to the width its state allows.
struct Sized<'a, Message> {
    sidebar: Slot<'a, Message>,
    width: f32,
}

impl<'a, Message: 'a> Compose<Message> for Sized<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.sidebar.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        iced_widget::container(self.sidebar.compose())
            .width(self.width)
            .height(Fill)
            .clip(true)
            .boxed()
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Provider<'a, Message> {
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

    fn listen(
        &self,
        _state: &State,
        event: &iced_core::Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Action<Event<Message>> {
        match event {
            iced_core::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if modifiers.command() && matches!(key.as_ref(), keyboard::Key::Character("b")) =>
            {
                Action::publish(Event::Toggle).and_capture()
            }
            _ => Action::none(),
        }
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let handle = Handle {
            is_open: self.is_open(state),
            collapsible: self.collapsible,
        };

        let mut body = (self.body)(&handle);
        body.inherit(&self.inherited);

        marker::owner(body.compose().map(Event::App).boxed(), |Toggle| {
            Event::Toggle
        })
    }
}

impl<'a, Message: 'static> Compose<Message> for Provider<'a, Message> {
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

impl<'a, Message: 'static> From<Provider<'a, Message>> for Slot<'a, Message> {
    fn from(provider: Provider<'a, Message>) -> Self {
        Slot::new(provider)
    }
}

impl<'a, Message: 'static> From<Provider<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(provider: Provider<'a, Message>) -> Self {
        component::component(provider).boxed()
    }
}
