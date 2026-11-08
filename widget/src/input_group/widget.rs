//! The surface behind [`InputGroup`](super::InputGroup): it paints the
//! Input's border, background and focus ring around its content and tracks
//! whether a control inside it is focused, which is how shadcn's
//! `has-[[data-slot=input-group-control]:focus-visible]` reads.

use std::any::Any;

use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme::text_input::{Status, StyleFn};

use crate::core::Renderer as _;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget::operation::Focusable;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Id, Meta, Operation};
use crate::core::{
    Background, Border, Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget,
};

/// Wraps the group's content in the Input's surface.
pub struct Surface<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    style: StyleFn<'a, Theme>,
    is_disabled: bool,
}

impl<'a, Message> Surface<'a, Message> {
    /// Paint `style` around `content`.
    pub fn new(content: Element<'a, Message, Theme, Renderer>, style: StyleFn<'a, Theme>) -> Self {
        Self {
            content,
            style,
            is_disabled: false,
        }
    }

    /// Paint the disabled surface.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

impl<Message> Meta for Surface<'_, Message> {}

impl<Message> Widget<Message, Theme, Renderer> for Surface<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
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
            .expect("an input group has one child");

        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        let mut focused = Focused(false);
        self.content
            .operate(child_tree, child_layout, viewport, renderer, &mut focused);

        let state = tree.state.downcast_mut::<State>();
        let is_hovered = cursor.is_over(layout.bounds());

        if focused.0 != state.is_focused || is_hovered != state.is_hovered {
            state.is_focused = focused.0;
            state.is_hovered = is_hovered;
            shell.request_redraw();
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
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        let status = if self.is_disabled {
            Status::Disabled
        } else if state.is_focused {
            Status::Focused {
                is_hovered: state.is_hovered,
            }
        } else if state.is_hovered {
            Status::Hovered
        } else {
            Status::Active
        };

        let surface = (self.style)(theme, status);
        let border: Border = surface.border.into();

        if let Some(ring) = surface.ring
            && ring.width > 0.0
        {
            let mut radius = border.radius;
            radius.top_left += ring.width;
            radius.top_right += ring.width;
            radius.bottom_right += ring.width;
            radius.bottom_left += ring.width;

            renderer.fill_quad(
                renderer::Quad {
                    bounds: bounds.expand(ring.width),
                    border: Border {
                        radius,
                        ..Border::default()
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(ring.color.into()),
            );
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border,
                ..renderer::Quad::default()
            },
            Background::from(surface.background),
        );

        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("an input group has one child");

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
            .expect("an input group has one child");

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
            .expect("an input group has one child");

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
            .expect("an input group has one child");

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

#[derive(Debug, Default)]
struct State {
    is_focused: bool,
    is_hovered: bool,
}

/// An [`Operation`] that finds whether any focusable widget is focused.
struct Focused(bool);

impl Operation for Focused {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        self.0 |= state.is_focused();
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn Any) {}
}
