//! ContextMenu: shadcn/ui faithful port (Radix `ContextMenu`).
//!
//! A menu opened at the pointer by a right press inside its trigger area,
//! over iced's [`anchored`](iced_widget::anchored) overlay anchored to that
//! point. Build the menu from [`menu`](crate::menu) parts; selecting an item
//! closes it, as do `Escape` and any press outside (the menu is modal).
//!
//! ```ignore
//! use icedxdy::widget::{column, context_menu, menu, text};
//!
//! context_menu(
//!     |_| text("Right click here").into(),
//!     |_| {
//!         menu::group([
//!             menu::item("Back").shortcut("⌘[").into(),
//!             menu::item("Reload").on_select(Message::Reload).into(),
//!         ])
//!     },
//! )
//! .width(192)
//! ```

use iced_core::Length::Fill;
use iced_core::{Element, Length, Pixels, Point};
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Space, Widget, anchored, mouse_area};
use icedxdy_core::Theme;
use icedxdy_core::theme::popover::content;

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker;
use crate::popover::{Handle, Signal};

/// A closure that builds part of a [`ContextMenu`].
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// A menu opened at the pointer by a right press.
pub struct ContextMenu<'a, Message> {
    trigger: Body<'a, Message>,
    content: Body<'a, Message>,
    width: Option<f32>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    inherited: Inherited,
}

/// The context menu's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// The pointer moved over the trigger area, to this point in it.
    Moved(Point),
    /// The trigger area was right-pressed.
    Open,
    /// The menu asked to close.
    Close,
    /// The content published an app message.
    App(Message),
}

/// Where the pointer last was over the trigger area, and where the menu is
/// open.
#[derive(Debug, Default)]
pub struct State {
    pointer: Point,
    open_at: Option<Point>,
}

/// Create a [`ContextMenu`]: `trigger` builds the area that opens it,
/// `content` the menu.
pub fn context_menu<'a, Message>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> ContextMenu<'a, Message> {
    ContextMenu {
        trigger: Box::new(trigger),
        content: Box::new(content),
        width: None,
        on_open_change: None,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> ContextMenu<'a, Message> {
    /// Set the menu's width (`className="w-48"`); by default it fits its
    /// items, at least 144px wide (`min-w-36`).
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into().0);
        self
    }

    /// Set the message published when the menu opens or closes.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }
}

impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer>
    for ContextMenu<'a, Message>
{
    type State = State;
    type Event = Event<Message>;

    fn update(
        &self,
        state: &mut State,
        event: Event<Message>,
        _renderer: &Renderer,
    ) -> Option<Message> {
        let open = match event {
            Event::App(message) => return Some(message),
            Event::Moved(pointer) => {
                state.pointer = pointer;
                return None;
            }
            Event::Open => {
                state.open_at = Some(state.pointer);
                true
            }
            Event::Close => {
                let _ = state.open_at.take()?;
                false
            }
        };

        self.on_open_change
            .as_ref()
            .map(|on_change| on_change(open))
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let handle = Handle::new(state.open_at.is_some());

        let mut area = (self.trigger)(&handle);
        area.inherit(&self.inherited);
        let area = mouse_area(area.compose().map(Event::App))
            .on_move(Event::Moved)
            .on_right_press(Event::Open);

        let menu: Element<'a, Event<Message>, Theme, Renderer> = if state.open_at.is_some() {
            let mut body = (self.content)(&handle);
            body.inherit(&self.inherited);

            // `min-w-36 p-1 rounded-lg bg-popover shadow-md ring-1`.
            let panel = Container::new(body)
                .padding(4)
                .width(self.width.map_or(Fill, Length::Fixed))
                .style(content);

            marker::owner(
                Slot::from(panel).compose().map(Event::App).boxed(),
                |signal| match signal {
                    Signal::Toggle | Signal::Close => Event::Close,
                },
            )
            .into()
        } else {
            Space::new().boxed()
        };

        // Radix opens the menu to the right of the pointer, its top level
        // with it, 2px away.
        anchored(area.boxed(), menu)
            .at(state.open_at.unwrap_or(Point::ORIGIN))
            .side(anchored::Side::Right)
            .align(anchored::Align::Start)
            .offset(2)
            .min_width(144)
            .modal(true)
            .open(state.open_at.is_some())
            .on_toggle(|open| if open { Event::Open } else { Event::Close })
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for ContextMenu<'a, Message> {
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

impl<'a, Message: Clone + 'static> From<ContextMenu<'a, Message>> for Slot<'a, Message> {
    fn from(menu: ContextMenu<'a, Message>) -> Self {
        Slot::new(menu)
    }
}

impl<'a, Message: Clone + 'static> From<ContextMenu<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(menu: ContextMenu<'a, Message>) -> Self {
        component::component(menu).boxed()
    }
}
