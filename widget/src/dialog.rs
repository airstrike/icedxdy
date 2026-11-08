//! Dialog — shadcn/ui faithful port (Radix `Dialog`).
//!
//! An iced `Component` that owns its open state. Two closures receive a
//! [`Handle`]: one builds what sits in place (usually the trigger), the
//! other the panel shown above a backdrop while open. `d.trigger` and
//! `d.close` play `DialogTrigger asChild` and `DialogClose asChild`:
//!
//! ```ignore
//! use icedxdy::widget::{button, column, dialog, field, input};
//!
//! dialog(
//!     |d| d.trigger(button("Open Dialog").style(button::Outline)),
//!     |d| {
//!         column![
//!             dialog::header("Edit profile", "Make changes to your profile here."),
//!             field(input("").default_value("Pedro Duarte")).label("Name"),
//!             dialog::footer([
//!                 d.close(button("Cancel").style(button::Outline)),
//!                 button("Save changes").on_press(Message::Save).into(),
//!             ]),
//!         ]
//!         .spacing(16)
//!         .into()
//!     },
//! )
//! // Controlled: .open(self.editing).on_open_change(Message::Editing)
//! ```
//!
//! A press outside the panel, `Escape`, the corner close button and any
//! `d.close(..)` child close it. The panel is built only while open, so its
//! widgets mount fresh each time, as in Radix. Both closures are built in
//! the app's message type and mapped once at the component's boundary
//! (`Message: 'static`).

pub mod modal;

use iced_core::Element;
use iced_core::Length::Fill;
use iced_core::alignment::Horizontal;
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;
use icedxdy_core::theme;
use icedxdy_core::theme::dialog;

use crate::button::{self, button};
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker::{self, Press};
use crate::{Size, column, row, text};

pub use modal::{Placement, Side, Signal};

/// A modal panel opened by its triggers.
pub struct Dialog<'a, Message> {
    trigger: Body<'a, Message>,
    content: Body<'a, Message>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    placement: Placement,
    show_close_button: bool,
    inherited: Inherited,
}

/// A closure that builds part of a [`Dialog`].
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// What a [`Dialog`]'s closures receive: whether it is open, and the
/// wrappers for its triggers and close buttons.
#[derive(Debug, Clone, Copy)]
pub struct Handle {
    is_open: bool,
}

/// The dialog's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// A trigger was pressed.
    Open,
    /// The panel was dismissed.
    Close,
    /// The content published an app message.
    App(Message),
}

/// The open state of an uncontrolled dialog.
#[derive(Debug, Default)]
pub struct State {
    open: Option<bool>,
}

/// Create a closed, uncontrolled [`Dialog`]: `trigger` builds what sits in
/// place, `content` the panel shown while open.
pub fn dialog<'a, Message>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> Dialog<'a, Message> {
    Dialog::new(trigger, content, Placement::Center { max_width: 384.0 })
}

impl<'a, Message> Dialog<'a, Message> {
    pub(crate) fn new(
        trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
        content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
        placement: Placement,
    ) -> Self {
        Self {
            trigger: Box::new(trigger),
            content: Box::new(content),
            open: None,
            default_open: false,
            on_open_change: None,
            placement,
            show_close_button: true,
            inherited: Inherited::default(),
        }
    }

    /// Control the dialog: it is open exactly when `open` is, and its
    /// triggers and close buttons only publish
    /// [`on_open_change`](Self::on_open_change).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Whether an uncontrolled dialog starts open.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Set the message published with the new open state when the dialog
    /// is opened or dismissed.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }

    /// Whether the panel shows its corner close button (shadcn's
    /// `showCloseButton`, on by default).
    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    /// Set the panel's maximum width (shadcn's `sm:max-w-sm`, 384px, by
    /// default). Ignored by sheets, which take their side's extent.
    pub fn max_width(mut self, max_width: f32) -> Self {
        if let Placement::Center { .. } = self.placement {
            self.placement = Placement::Center { max_width };
        }
        self
    }

    pub(crate) fn side(mut self, side: Side) -> Self {
        self.placement = Placement::Side(side);
        self
    }

    fn is_open(&self, state: &State) -> bool {
        self.open.or(state.open).unwrap_or(self.default_open)
    }
}

impl Handle {
    /// Whether the dialog is open.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Make `child` open the dialog when pressed, like
    /// `<DialogTrigger asChild>`. The child keeps its own handlers.
    pub fn trigger<'a, Message: 'a>(
        &self,
        child: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        Slot::new(Marked {
            child: child.into(),
            signal: Signal::Open,
        })
    }

    /// Make `child` close the dialog when pressed, like
    /// `<DialogClose asChild>`. The child keeps its own handlers.
    pub fn close<'a, Message: 'a>(&self, child: impl Into<Slot<'a, Message>>) -> Slot<'a, Message> {
        Slot::new(Marked {
            child: child.into(),
            signal: Signal::Close,
        })
    }
}

/// A `DialogHeader`: a `DialogTitle` over a `DialogDescription`.
pub fn header<'a, Message: 'a>(title: &'a str, description: &'a str) -> Slot<'a, Message> {
    column![
        text::Text::new(title)
            .size(text::size::BASE)
            .medium()
            .style(theme::text::Foreground),
        text::muted(description),
    ]
    .spacing(8)
    .into()
}

/// A `DialogFooter`: its actions in a row, aligned to the end.
pub fn footer<'a, Message: 'a>(
    actions: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    Container::new(row::Row::with_children(actions).spacing(8))
        .width(Fill)
        .align_x(Horizontal::Right)
        .into()
}

/// A child that records a [`Signal`] when pressed.
pub(crate) struct Marked<'a, Message> {
    pub(crate) child: Slot<'a, Message>,
    pub(crate) signal: Signal,
}

impl<'a, Message: 'a> Compose<Message> for Marked<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.child.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        marker::marker(self.child.compose(), self.signal, Press::Captured).into()
    }
}

impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer> for Dialog<'a, Message> {
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
            Event::Open => true,
            Event::Close => false,
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

        let mut base = (self.trigger)(&handle);
        base.inherit(&self.inherited);
        let base = marker::owner(
            base.compose().map(Event::App).boxed(),
            |signal| match signal {
                Signal::Open => Event::Open,
                Signal::Close => Event::Close,
            },
        );

        let panel = handle.is_open.then(|| {
            let mut content = (self.content)(&handle);
            content.inherit(&self.inherited);

            // nova `p-4` on a dialog; a sheet's sections pad themselves.
            let padding = match self.placement {
                Placement::Center { .. } => 16,
                Placement::Side(_) => 0,
            };
            let content = Container::new(content).width(Fill).padding(padding);

            let content: Slot<'a, Message> = if self.show_close_button {
                // nova `absolute top-2 right-2` (sheet: `top-3 right-3`).
                let inset = match self.placement {
                    Placement::Center { .. } => 8,
                    Placement::Side(_) => 12,
                };
                let close = handle.close(
                    button(text::Text::new("\u{00d7}").size(text::size::BASE))
                        .size(Size::Sm)
                        .square()
                        .style(button::Ghost),
                );

                Slot::from(
                    iced_widget::stack![
                        Slot::from(content).compose(),
                        iced_widget::container(close.compose())
                            .width(Fill)
                            .align_right(Fill)
                            .padding(inset),
                    ]
                    .boxed(),
                )
            } else {
                content.into()
            };

            let panel = match self.placement {
                Placement::Center { .. } => {
                    Container::new(content).width(Fill).style(dialog::content)
                }
                // A left or right sheet spans the window's height; a top or
                // bottom one takes its content's (`h-auto`).
                Placement::Side(Side::Left | Side::Right) => Container::new(content)
                    .width(Fill)
                    .height(Fill)
                    .style(dialog::sheet),
                Placement::Side(Side::Top | Side::Bottom) => {
                    Container::new(content).width(Fill).style(dialog::sheet)
                }
            };

            Slot::from(panel).compose().map(Event::App).boxed()
        });

        modal::Modal::new(base.into(), panel, self.placement, Event::Close)
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for Dialog<'a, Message> {
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

impl<'a, Message: Clone + 'static> From<Dialog<'a, Message>> for Slot<'a, Message> {
    fn from(dialog: Dialog<'a, Message>) -> Self {
        Slot::new(dialog)
    }
}

impl<'a, Message: Clone + 'static> From<Dialog<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(dialog: Dialog<'a, Message>) -> Self {
        component::component(dialog).boxed()
    }
}
