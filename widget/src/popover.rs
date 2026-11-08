//! Popover: shadcn/ui faithful port (Radix `Popover`).
//!
//! An iced `Component` that owns its open state, over iced's
//! [`anchored`](iced_widget::anchored) overlay. Two closures receive a
//! [`Handle`]: one builds what sits in place (the trigger), the other the
//! panel shown next to it while open. `p.trigger` and `p.close` play
//! `PopoverTrigger asChild` and `PopoverClose asChild`:
//!
//! ```ignore
//! use icedxdy::widget::{button, popover};
//!
//! popover(
//!     |p| p.trigger(button("Open popover").style(button::Outline)),
//!     |_| popover::header("Dimensions", "Set the dimensions for the layer."),
//! )
//! .align(popover::Align::Start)
//! // Controlled: .open(self.is_open).on_open_change(Message::OpenChanged)
//! ```
//!
//! The panel sits 4px below its trigger, centered, flips above it when the
//! window has no room below and slides along it to stay inside. A press
//! outside the panel (the page still receives it) and `Escape` close it;
//! pressing the trigger again does too. The panel is built only while open,
//! so its widgets mount fresh each time, as in Radix.

use iced_core::{Element, Padding, Pixels};
use iced_widget::anchored;
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Space, Widget};
use icedxdy_core::Theme;
use icedxdy_core::theme;
use icedxdy_core::theme::popover::content;

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker::{self, Press};
use crate::{column, text};

pub use iced_widget::anchored::{Align, Side};

/// A closure that builds part of a [`Popover`].
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// A panel anchored to its trigger, opened and closed by it.
pub struct Popover<'a, Message> {
    trigger: Body<'a, Message>,
    content: Body<'a, Message>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    position: anchored::Position,
    width: Option<f32>,
    min_width: f32,
    padding: Padding,
    modal: bool,
    inherited: Inherited,
}

/// What a [`Popover`]'s closures receive: whether it is open, and the
/// wrappers for its triggers and close buttons.
#[derive(Debug, Clone, Copy)]
pub struct Handle {
    is_open: bool,
}

/// What a popover's markers signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    /// A trigger was pressed.
    Toggle,
    /// A close button (or a menu item) was pressed.
    Close,
}

/// The popover's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// A trigger was pressed.
    Toggle,
    /// The popover asked to open or close.
    Set(bool),
    /// The content published an app message.
    App(Message),
}

/// The open state of an uncontrolled popover.
#[derive(Debug, Default)]
pub struct State {
    open: Option<bool>,
}

/// Create a closed, uncontrolled [`Popover`]: `trigger` builds what sits in
/// place, `content` the panel shown while open.
pub fn popover<'a, Message>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> Popover<'a, Message> {
    Popover {
        trigger: Box::new(trigger),
        content: Box::new(content),
        open: None,
        default_open: false,
        on_open_change: None,
        position: anchored::Position::new(Side::Bottom).offset(4),
        // `w-72`.
        width: Some(288.0),
        min_width: 0.0,
        // `p-2.5`.
        padding: Padding::new(10.0),
        modal: false,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Popover<'a, Message> {
    /// Control the popover: it is open exactly when `open` is, and its
    /// triggers and dismissals only publish
    /// [`on_open_change`](Self::on_open_change).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Whether an uncontrolled popover starts open.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Set the message published with the new open state when the popover
    /// is opened or dismissed.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }

    /// Set the side of the trigger the panel prefers (`Bottom` by default).
    pub fn side(mut self, side: Side) -> Self {
        self.position = self.position.side(side);
        self
    }

    /// Set how the panel lines up with its trigger (`Center` by default).
    pub fn align(mut self, align: Align) -> Self {
        self.position = self.position.align(align);
        self
    }

    /// Set the gap between the trigger and the panel (shadcn's
    /// `sideOffset`, 4px by default).
    pub fn side_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.position = self.position.offset(offset);
        self
    }

    /// Move a start- or end-aligned panel inward along the trigger
    /// (`alignOffset`).
    pub fn align_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.position = self.position.align_offset(offset);
        self
    }

    /// Set the panel's width (`w-72`, 288px, by default).
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into().0);
        self
    }

    fn is_open(&self, state: &State) -> bool {
        self.open.or(state.open).unwrap_or(self.default_open)
    }
}

impl Handle {
    /// Whether the popover is open.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Make `child` toggle the popover when pressed, like
    /// `<PopoverTrigger asChild>`. The child keeps its own handlers.
    pub fn trigger<'a, Message: 'a>(
        &self,
        child: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        Slot::new(Marked {
            child: child.into(),
            signal: Signal::Toggle,
            press: Press::Captured,
        })
    }

    /// Make `child` close the popover when pressed, like
    /// `<PopoverClose asChild>`. The child keeps its own handlers.
    pub fn close<'a, Message: 'a>(&self, child: impl Into<Slot<'a, Message>>) -> Slot<'a, Message> {
        Slot::new(Marked {
            child: child.into(),
            signal: Signal::Close,
            press: Press::Captured,
        })
    }
}

/// A `PopoverHeader`: a `PopoverTitle` over a `PopoverDescription`.
pub fn header<'a, Message: 'a>(title: &'a str, description: &'a str) -> Slot<'a, Message> {
    column![
        text::Text::new(title)
            .size(text::size::SM)
            .medium()
            .style(theme::text::Foreground),
        text::muted(description),
    ]
    .spacing(2)
    .into()
}

/// A child that records a [`Signal`] when pressed.
pub(crate) struct Marked<'a, Message> {
    pub(crate) child: Slot<'a, Message>,
    pub(crate) signal: Signal,
    pub(crate) press: Press,
}

impl<'a, Message: 'a> Compose<Message> for Marked<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.child.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        marker::marker(self.child.compose(), self.signal, self.press).into()
    }
}

fn on_signal<Message>(signal: Signal) -> Event<Message> {
    match signal {
        Signal::Toggle => Event::Toggle,
        Signal::Close => Event::Set(false),
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Popover<'a, Message> {
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
        let next = match event {
            Event::App(message) => return Some(message),
            Event::Toggle => !self.is_open(state),
            Event::Set(open) => open,
        };

        if next == self.is_open(state) {
            return None;
        }

        if self.open.is_none() {
            state.open = Some(next);
        }

        self.on_open_change
            .as_ref()
            .map(|on_change| on_change(next))
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let handle = Handle {
            is_open: self.is_open(state),
        };

        let mut trigger = (self.trigger)(&handle);
        trigger.inherit(&self.inherited);
        let trigger = marker::owner(trigger.compose().map(Event::App).boxed(), on_signal);

        let panel: Element<'a, Event<Message>, Theme, Renderer> = if handle.is_open {
            let mut body = (self.content)(&handle);
            body.inherit(&self.inherited);

            let panel = Container::new(body)
                .padding(self.padding)
                .min_width(self.min_width)
                .style(content);
            let panel = match self.width {
                Some(width) => panel.width(width),
                None => panel,
            };

            marker::owner(
                Slot::from(panel).compose().map(Event::App).boxed(),
                on_signal,
            )
            .into()
        } else {
            Space::new().boxed()
        };

        anchored(Element::from(trigger), panel)
            .position(self.position)
            .width(if self.width.is_some() {
                anchored::Width::Content
            } else {
                anchored::Width::AtLeastAnchor
            })
            .open(handle.is_open)
            .modal(self.modal)
            .on_toggle(Event::Set)
    }
}

impl<'a, Message: 'static> Compose<Message> for Popover<'a, Message> {
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

impl<'a, Message: 'static> From<Popover<'a, Message>> for Slot<'a, Message> {
    fn from(popover: Popover<'a, Message>) -> Self {
        Slot::new(popover)
    }
}

impl<'a, Message: 'static> From<Popover<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(popover: Popover<'a, Message>) -> Self {
        component::component(popover).boxed()
    }
}
