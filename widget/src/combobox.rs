//! Combobox: shadcn/ui faithful port (`Combobox`).
//!
//! An input that filters a list of items anchored below it. Typing or
//! focusing the input opens the list, which shows the items containing the
//! typed text (ignoring case) or `ComboboxEmpty`'s message when none do;
//! choosing one fills the input with it and closes the list. An iced
//! `Component` that owns the typed text, the open state and, uncontrolled,
//! the value:
//!
//! ```ignore
//! use icedxdy::widget::combobox;
//!
//! combobox(["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"])
//!     .placeholder("Select a framework")
//!     .width(256)
//! // Controlled: .value(self.framework.clone()).on_value_change(Message::Framework)
//! ```
//!
//! The list is as wide as the input, non-modal (the page keeps the
//! pointer), and closes on `Escape` or a press outside it.

use iced_core::Length::Fill;
use iced_core::{Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Space, Widget, anchored};
use icedxdy_core::Theme;
use icedxdy_core::theme::popover::content;

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::input_group::{self, input_group};
use crate::{column, input, menu, scrollable, text};

/// An input choosing one of its items by filtering them as you type.
pub struct Combobox<'a, Message> {
    items: Vec<String>,
    placeholder: String,
    empty: String,
    value: Option<String>,
    default_value: Option<String>,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    disabled: bool,
    width: Length,
    icon: Option<Box<dyn Fn() -> iced_widget::Text<'a, Theme> + 'a>>,
    inherited: Inherited,
}

/// The combobox's private events.
#[derive(Debug, Clone)]
pub enum Event {
    /// The input was edited.
    Query(String),
    /// The input took focus.
    Focus,
    /// The list asked to open or close.
    Set(bool),
    /// An item was chosen.
    Choose(String),
}

/// What a combobox holds: the typed text, whether the list is open and,
/// uncontrolled, the value.
#[derive(Debug, Default)]
pub struct State {
    query: Option<String>,
    open: bool,
    value: Option<String>,
}

/// Create an uncontrolled [`Combobox`] over `items`.
pub fn combobox<'a, Message>(
    items: impl IntoIterator<Item = impl Into<String>>,
) -> Combobox<'a, Message> {
    Combobox {
        items: items.into_iter().map(Into::into).collect(),
        placeholder: String::new(),
        empty: "No items found.".to_owned(),
        value: None,
        default_value: None,
        on_value_change: None,
        disabled: false,
        width: Length::Fixed(256.0),
        icon: None,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Combobox<'a, Message> {
    /// Set the input's placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the message shown when nothing matches (`ComboboxEmpty`).
    pub fn empty(mut self, empty: impl Into<String>) -> Self {
        self.empty = empty.into();
        self
    }

    /// Control the combobox: it holds exactly `value`, and choosing an item
    /// only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The value an uncontrolled combobox starts with.
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Set the message published with the chosen item.
    pub fn on_value_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Disable the combobox.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the input's width (256px by default).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the trigger chevron at the input's end (lucide's `chevron-down`
    /// in shadcn); a text glyph by default.
    pub fn icon(mut self, icon: impl Fn() -> iced_widget::Text<'a, Theme> + 'a) -> Self {
        self.icon = Some(Box::new(icon));
        self
    }

    fn current<'s>(&'s self, state: &'s State) -> Option<&'s str> {
        self.value
            .as_deref()
            .or(state.value.as_deref())
            .or(self.default_value.as_deref())
    }
}

impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer>
    for Combobox<'a, Message>
{
    type State = State;
    type Event = Event;

    fn update(&self, state: &mut State, event: Event, _renderer: &Renderer) -> Option<Message> {
        match event {
            Event::Query(query) => {
                state.query = Some(query);
                state.open = true;
                None
            }
            Event::Focus => {
                state.open = true;
                None
            }
            Event::Set(open) => {
                state.open = open;
                None
            }
            Event::Choose(value) => {
                state.open = false;
                state.query = Some(value.clone());

                if self.value.is_none() {
                    state.value = Some(value.clone());
                }

                self.on_value_change
                    .as_ref()
                    .map(|on_change| on_change(value))
            }
        }
    }

    fn view(&self, state: &State) -> impl Widget<Event, Theme, Renderer> + 'a {
        let current = self.current(state).map(str::to_owned);
        let query = state
            .query
            .clone()
            .or_else(|| current.clone())
            .unwrap_or_default();

        let chevron: Slot<'a, Event> = match &self.icon {
            Some(icon) => icon().size(16).into(),
            None => text::muted("\u{2304}").into(),
        };

        let mut field: Slot<'a, Event> = Container::new(
            input_group(
                input(self.placeholder.clone())
                    .value(query.clone())
                    .on_input(Event::Query)
                    .on_focus(Event::Focus)
                    .disabled(self.disabled),
            )
            .addon(input_group::addon(chevron).align(input_group::Align::InlineEnd))
            .disabled(self.disabled),
        )
        .width(self.width)
        .into();
        field.inherit(&self.inherited);

        let is_open = state.open && !self.disabled;

        let list: Element<'a, Event, Theme, Renderer> = if is_open {
            // A query equal to the chosen item shows every item, so the
            // list reopens whole after a choice.
            let needle = query.to_lowercase();
            let shows_all = current.as_deref() == Some(query.as_str());

            let matches: Vec<Slot<'a, Event>> = self
                .items
                .iter()
                .filter(|item| shows_all || item.to_lowercase().contains(&needle))
                .map(|item| {
                    let chosen = current.as_deref() == Some(item.as_str());
                    let value = item.clone();

                    Slot::from(
                        menu::checkbox_item(text::small(item.clone()), chosen, move |_| {
                            Event::Choose(value.clone())
                        })
                        .closes(false),
                    )
                })
                .collect();

            let body: Slot<'a, Event> = if matches.is_empty() {
                // `ComboboxEmpty`: `py-2 text-center text-sm text-muted-foreground`.
                Container::new(text::muted(self.empty.clone()))
                    .width(Fill)
                    .center_x(Fill)
                    .padding([8, 0])
                    .into()
            } else {
                column::Column::with_children(matches).width(Fill).into()
            };

            let panel = Container::new(
                scrollable(Container::new(body).width(Fill).padding(4))
                    .width(Fill)
                    .height(Length::Shrink),
            )
            .width(Fill)
            .max_height(252)
            .style(content);

            Slot::from(panel).compose()
        } else {
            Space::new().boxed()
        };

        anchored(field.compose(), list)
            .side(anchored::Side::Bottom)
            .offset(6)
            .width(anchored::Width::AtLeastAnchor)
            .open(is_open)
            .on_toggle(Event::Set)
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for Combobox<'a, Message> {
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

impl<'a, Message: Clone + 'static> From<Combobox<'a, Message>> for Slot<'a, Message> {
    fn from(combobox: Combobox<'a, Message>) -> Self {
        Slot::new(combobox)
    }
}

impl<'a, Message: Clone + 'static> From<Combobox<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(combobox: Combobox<'a, Message>) -> Self {
        component::component(combobox).boxed()
    }
}
