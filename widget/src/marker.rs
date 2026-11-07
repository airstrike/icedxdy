//! Markers: let a child inside user content signal the widget that owns it.
//!
//! Radix's `asChild` triggers (`CollapsibleTrigger`, `DialogClose`,
//! `SidebarTrigger`, a `FieldLabel`'s `htmlFor`) wrap an element the user
//! builds, in the user's own message type. That element cannot publish the
//! owner's private event, so the owner wraps it in a [`Marker`] instead. The
//! marker watches the events its child receives and, when the child is
//! pressed, records a signal in its own tree state. The owner forwards every
//! event to its content and then runs [`drain`] over it, an [`Operation`]
//! that takes the recorded signals, and acts on them in the same event.
//!
//! Signals stay in the marker until an owner drains them, so a marker deeper
//! in the tree is drained by its nearest owner: an owner updates its content
//! (and with it every nested owner) before it drains. Signals are typed, so
//! owners of different kinds never take each other's.
//!
//! A child's own handlers still fire. `c.trigger(button("Close").on_press(m))`
//! publishes `m` and toggles the owner, like Radix's `composeEventHandlers`.

use std::any::Any;

use crate::core::keyboard;
use crate::core::keyboard::key;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::touch;
use crate::core::widget::Id;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::{Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget};

/// What counts as pressing a [`Marker`]'s child.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Press {
    /// The child is interactive (a button): it must capture the press, or
    /// `Enter`/`Space` while focused. A disabled child never signals.
    Captured,
    /// The child is passive (a label's text): any press released over it
    /// that nothing inside captured.
    Any,
}

/// Wraps a child and records `signal` when the child is pressed.
pub struct Marker<'a, S, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    signal: S,
    press: Press,
}

/// Wrap `content` in a [`Marker`] recording `signal` when pressed.
pub fn marker<'a, S, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    signal: S,
    press: Press,
) -> Marker<'a, S, Message, Theme, Renderer> {
    Marker {
        content: content.into(),
        signal,
        press,
    }
}

/// The signals a marker holds until its owner drains them.
struct State<S> {
    is_pressed: bool,
    signals: Signals<S>,
}

/// Signals recorded by a [`Marker`], handed to [`Drain`] through
/// [`Operation::custom`].
struct Signals<S>(Vec<S>);

impl<S, Message, Theme, Renderer> Meta for Marker<'_, S, Message, Theme, Renderer> {}

impl<S, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Marker<'_, S, Message, Theme, Renderer>
where
    S: Clone + Send + 'static,
    Renderer: crate::core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<S>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<S> {
            is_pressed: false,
            signals: Signals(Vec::new()),
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let was_captured = shell.is_event_captured();

        {
            let (child_layout, child_tree) = layout
                .iter_mut(&mut tree.children)
                .next()
                .expect("a marker has one child");

            self.content.update(
                child_tree,
                event,
                child_layout,
                cursor,
                renderer,
                shell,
                viewport,
            );
        }

        let child_captured = !was_captured && shell.is_event_captured();
        let is_over = cursor.is_over(layout.bounds());
        let state = tree.state.downcast_mut::<State<S>>();

        let pressed = match self.press {
            Press::Captured => child_captured,
            Press::Any => !shell.is_event_captured(),
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                state.is_pressed = pressed && is_over;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if std::mem::take(&mut state.is_pressed) && is_over {
                    state.signals.0.push(self.signal.clone());
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                state.is_pressed = false;
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. })
                if self.press == Press::Captured
                    && child_captured
                    && matches!(
                        key.as_ref(),
                        keyboard::Key::Named(key::Named::Enter | key::Named::Space)
                    ) =>
            {
                state.signals.0.push(self.signal.clone());
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a marker has one child");

        self.content.draw(
            child_tree,
            renderer,
            theme,
            style,
            child_layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a marker has one child");

        self.content
            .mouse_interaction(child_tree, child_layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State<S>>();
        operation.custom(None, layout.bounds(), &mut state.signals);

        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a marker has one child");

        self.content
            .operate(child_tree, child_layout, viewport, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a marker has one child");

        self.content.overlay(
            child_tree,
            child_layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}

impl<'a, S, Message, Theme, Renderer> From<Marker<'a, S, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    S: Clone + Send + 'static,
    Message: 'a,
    Theme: 'a,
    Renderer: crate::core::Renderer + 'a,
{
    fn from(marker: Marker<'a, S, Message, Theme, Renderer>) -> Self {
        marker._boxed()
    }
}

/// An [`Operation`] that takes every signal of type `S` recorded by the
/// markers it reaches.
pub struct Drain<S> {
    signals: Vec<S>,
}

/// Produces a [`Drain`] for signals of type `S`.
pub fn drain<S>() -> Drain<S> {
    Drain {
        signals: Vec::new(),
    }
}

impl<S> Drain<S> {
    /// The signals taken, in tree order.
    pub fn into_signals(self) -> Vec<S> {
        self.signals
    }
}

impl<S: Send + 'static> Operation for Drain<S> {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Any) {
        if let Some(signals) = state.downcast_mut::<Signals<S>>() {
            self.signals.append(&mut signals.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Toggle;

    #[derive(Debug, Clone, PartialEq)]
    struct Close;

    #[test]
    fn drain_takes_its_own_signals_once() {
        let mut toggles = Signals(vec![Toggle, Toggle]);
        let mut closes = Signals(vec![Close]);
        let mut drain = drain::<Toggle>();

        drain.custom(None, Rectangle::default(), &mut toggles);
        drain.custom(None, Rectangle::default(), &mut closes);

        assert_eq!(drain.into_signals(), vec![Toggle, Toggle]);
        assert!(toggles.0.is_empty());
        assert_eq!(closes.0, vec![Close], "other signal types are left alone");
    }
}
