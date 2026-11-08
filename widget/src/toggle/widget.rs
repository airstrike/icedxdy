//! The widget behind [`Toggle`](super::Toggle): a two-state button that
//! paints its own surface, keeps an uncontrolled value in its tree and a
//! [`Control`] so a linked label can press it.

use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::touch;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::{
    Background, Color, Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget,
};
use crate::operation::control::Control;

use icedxdy_core::Theme;
use icedxdy_core::theme::button::{Spec, Status};
use icedxdy_core::theme::toggle;

/// A pressable surface around `content` that is either on or off.
pub struct Toggle<'a, Message, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    /// The app's value when controlled.
    pressed: Option<bool>,
    default_pressed: bool,
    on_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    is_disabled: bool,
    style: Spec,
}

impl<'a, Message, Renderer> Toggle<'a, Message, Renderer> {
    /// Wrap `content`, painted with `style`.
    pub fn new(content: Element<'a, Message, Theme, Renderer>, style: Spec) -> Self {
        Self {
            content,
            pressed: None,
            default_pressed: false,
            on_change: None,
            is_disabled: false,
            style,
        }
    }

    /// Control the toggle: it shows `pressed`, and a press only publishes
    /// the change.
    pub fn pressed(mut self, pressed: Option<bool>) -> Self {
        self.pressed = pressed;
        self
    }

    /// The initial value of an uncontrolled toggle.
    pub fn default_pressed(mut self, pressed: bool) -> Self {
        self.default_pressed = pressed;
        self
    }

    /// Publish the new value on every press.
    pub fn on_change(mut self, on_change: Option<Box<dyn Fn(bool) -> Message + 'a>>) -> Self {
        self.on_change = on_change;
        self
    }

    /// Whether the toggle ignores input.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    fn value(&self, state: &State) -> bool {
        self.pressed.unwrap_or(state.value)
    }

    fn toggle(&self, state: &mut State, shell: &mut Shell<'_, Message>) {
        let value = !self.value(state);

        if self.pressed.is_none() {
            state.value = value;
        }

        if let Some(on_change) = &self.on_change {
            shell.publish(on_change(value));
        }

        shell.request_redraw();
    }
}

impl<Message, Renderer> Meta for Toggle<'_, Message, Renderer> {}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Toggle<'_, Message, Renderer>
where
    Renderer: crate::core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            value: self.default_pressed,
            is_down: false,
            is_hovered: false,
            control: Control::new(self.default_pressed),
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let child = &mut tree.children[0];
        self.content.layout(child, renderer, limits);
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
        {
            let (child_layout, child_tree) = layout
                .iter_mut(&mut tree.children)
                .next()
                .expect("a toggle has one child");

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

        let state = tree.state.downcast_mut::<State>();

        if state.control.take_activation() && !self.is_disabled {
            self.toggle(state, shell);
        }

        let is_over = cursor.is_over(layout.bounds());

        if is_over != state.is_hovered {
            state.is_hovered = is_over;
            shell.request_redraw();
        }

        if self.is_disabled {
            state.is_down = false;
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if is_over =>
            {
                state.is_down = true;
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. })
                if state.is_down =>
            {
                state.is_down = false;

                if is_over {
                    self.toggle(state, shell);
                    shell.capture_event();
                } else {
                    shell.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let is_over = cursor.is_over(bounds);

        let status = if self.is_disabled {
            Status::Disabled
        } else if state.is_down && is_over {
            Status::Pressed
        } else if is_over {
            Status::Hovered
        } else {
            Status::Active
        };

        let mut style = self.style.resolve(theme, status);

        if self.value(state) {
            style = toggle::on(theme, style);

            if self.is_disabled {
                style.background = style
                    .background
                    .map(|background| background.scale_alpha(0.5));
                style.text_color = style.text_color.scale_alpha(0.5);
            }
        }

        let style: iced_widget::button::Style = style.into();

        if style.background.is_some() || style.border.width > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    shadow: style.shadow,
                    snap: style.snap,
                },
                style
                    .background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a toggle has one child");

        self.content.draw(
            child_tree,
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color,
            },
            child_layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.is_disabled && cursor.is_over(layout.bounds()) {
            mouse::Interaction::NotAllowed
        } else {
            mouse::Interaction::default()
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let value = self.pressed.unwrap_or(state.value);
        state.control.set_checked(value);
        operation.custom(None, layout.bounds(), &mut state.control);

        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a toggle has one child");

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
            .expect("a toggle has one child");

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

struct State {
    /// The value of an uncontrolled toggle, kept across rebuilds.
    value: bool,
    /// Whether a press started inside the toggle and is still held.
    is_down: bool,
    is_hovered: bool,
    control: Control,
}
