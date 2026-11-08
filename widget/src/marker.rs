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
//! [`Owner`] does exactly that and publishes a message per signal, which is
//! how an iced `Component` turns its markers into its private events.
//!
//! Signals stay in the marker until an owner drains them, so a marker deeper
//! in the tree is drained by its nearest owner: an owner updates its content
//! (and with it every nested owner) before it drains. Signals are typed, so
//! owners of different kinds never take each other's.
//!
//! A marker can also stand in for a form control ([`Marker::control`]): it
//! then exposes a [`Control`] to the control operations, so a linked
//! [`field`](crate::field) label presses it like a click and a field card
//! reads whether it is checked.
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
use crate::operation::control::Control;

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
    /// Whether the marker is a form control, and if so whether it is
    /// checked.
    control: Option<bool>,
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
        control: None,
    }
}

impl<S, Message, Theme, Renderer> Marker<'_, S, Message, Theme, Renderer> {
    /// Make the marker a form control, `checked` or not: a linked label's
    /// `control::activate` records the signal
    /// as a press would.
    pub fn control(mut self, checked: bool) -> Self {
        self.control = Some(checked);
        self
    }
}

/// The signals a marker holds until its owner drains them.
struct State<S> {
    is_pressed: bool,
    signals: Signals<S>,
    control: Control,
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
            control: Control::new(self.control.unwrap_or(false)),
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

        if state.control.take_activation() && self.control.is_some() {
            state.signals.0.push(self.signal.clone());
        }

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

        if let Some(checked) = self.control {
            state.control.set_checked(checked);
            operation.custom(None, layout.bounds(), &mut state.control);
        }

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

/// Wraps the content of a marker's owner: after every event it drains the
/// content's signals of type `S` and publishes `on_signal` for each, in the
/// same event.
pub struct Owner<'a, S, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    on_signal: Box<dyn Fn(S) -> Message + 'a>,
}

/// Wrap `content` in an [`Owner`] publishing `on_signal` for the signals its
/// markers record.
pub fn owner<'a, S, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    on_signal: impl Fn(S) -> Message + 'a,
) -> Owner<'a, S, Message, Theme, Renderer> {
    Owner {
        content: content.into(),
        on_signal: Box::new(on_signal),
    }
}

impl<S, Message, Theme, Renderer> Meta for Owner<'_, S, Message, Theme, Renderer> {}

impl<S, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Owner<'_, S, Message, Theme, Renderer>
where
    S: Send + 'static,
    Renderer: crate::core::Renderer,
{
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
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("an owner has one child");

        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        let mut drain = drain::<S>();
        self.content
            .operate(child_tree, child_layout, viewport, renderer, &mut drain);

        for signal in drain.into_signals() {
            shell.publish((self.on_signal)(signal));
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
            .expect("an owner has one child");

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
            .expect("an owner has one child");

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
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("an owner has one child");

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
            .expect("an owner has one child");

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

impl<'a, S, Message, Theme, Renderer> From<Owner<'a, S, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    S: Send + 'static,
    Message: 'a,
    Theme: 'a,
    Renderer: crate::core::Renderer + 'a,
{
    fn from(owner: Owner<'a, S, Message, Theme, Renderer>) -> Self {
        owner._boxed()
    }
}

/// Wraps content whose markers live in its overlays (a submenu's items):
/// after every overlay event it takes their signals of type `S` and holds
/// them in its own state, where its nearest owner drains them like a
/// marker's.
pub struct Relay<'a, S, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    _signal: std::marker::PhantomData<S>,
}

/// Wrap `content` in a [`Relay`] for signals of type `S`.
pub fn relay<'a, S, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Relay<'a, S, Message, Theme, Renderer> {
    Relay {
        content: content.into(),
        _signal: std::marker::PhantomData,
    }
}

impl<S, Message, Theme, Renderer> Meta for Relay<'_, S, Message, Theme, Renderer> {}

impl<S, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Relay<'_, S, Message, Theme, Renderer>
where
    S: Send + 'static,
    Renderer: crate::core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Signals<S>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Signals::<S>(Vec::new()))
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
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a relay has one child");

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
            .expect("a relay has one child");

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
            .expect("a relay has one child");

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
        let signals = tree.state.downcast_mut::<Signals<S>>();
        operation.custom(None, layout.bounds(), signals);

        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a relay has one child");

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
        let signals = tree.state.downcast_mut::<Signals<S>>();
        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a relay has one child");

        let overlays = self.content.overlay(
            child_tree,
            child_layout,
            renderer,
            viewport,
            translation,
            window,
        );

        if overlays.is_empty() {
            return overlays;
        }

        vec![overlay::Element::new(Box::new(Relayed {
            overlays,
            signals,
        }))]
    }
}

impl<'a, S, Message, Theme, Renderer> From<Relay<'a, S, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    S: Send + 'static,
    Message: 'a,
    Theme: 'a,
    Renderer: crate::core::Renderer + 'a,
{
    fn from(relay: Relay<'a, S, Message, Theme, Renderer>) -> Self {
        relay._boxed()
    }
}

/// A [`Relay`]'s overlays, passed through untouched, whose signals it takes
/// after every event.
struct Relayed<'a, 'b, S, Message, Theme, Renderer> {
    overlays: Vec<overlay::Element<'a, Message, Theme, Renderer>>,
    signals: &'b mut Signals<S>,
}

impl<S, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Relayed<'_, '_, S, Message, Theme, Renderer>
where
    S: Send + 'static,
    Renderer: crate::core::Renderer,
{
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        for overlay in &self.overlays {
            overlay.as_overlay().draw(renderer, theme, style, cursor);
        }
    }

    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn Operation) {
        for overlay in &mut self.overlays {
            overlay.as_overlay_mut().operate(renderer, operation);
        }
    }

    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        for overlay in &mut self.overlays {
            if shell.is_event_captured() {
                break;
            }

            overlay
                .as_overlay_mut()
                .update(event, cursor, renderer, shell);
        }

        let mut drain = drain::<S>();

        for overlay in &mut self.overlays {
            overlay.as_overlay_mut().operate(renderer, &mut drain);
        }

        let mut signals = drain.into_signals();

        if !signals.is_empty() {
            self.signals.0.append(&mut signals);
            // The owner drains on its next event; ask for one now.
            shell.request_redraw();
        }
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        self.overlays
            .iter()
            .map(|overlay| overlay.as_overlay().mouse_interaction(cursor, renderer))
            .max()
            .unwrap_or_default()
    }

    fn overlay<'c>(
        &'c mut self,
        renderer: &Renderer,
    ) -> Vec<overlay::Element<'c, Message, Theme, Renderer>> {
        self.overlays
            .iter_mut()
            .flat_map(|overlay| overlay.as_overlay_mut().overlay(renderer))
            .collect()
    }

    fn index(&self) -> f32 {
        self.overlays
            .iter()
            .map(|overlay| overlay.as_overlay().index())
            .fold(1.0, f32::max)
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
