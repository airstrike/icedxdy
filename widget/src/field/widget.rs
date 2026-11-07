//! The widget behind [`Field`](super::Field): it links the field's label to
//! its control and paints the optional card chrome.
//!
//! - **Label to control.** The label sits in a [`marker`] recording a
//!   [`Label`] signal. After every event the field drains those signals and,
//!   when its label was pressed, runs [`control::activate`] over its content:
//!   a checkbox, switch or radio toggles as if clicked, an input takes focus.
//!   This is shadcn's `<FieldLabel htmlFor>`.
//! - **Card.** In card mode the border and background cross-fade between an
//!   inactive and an active style, following [`control::checked`] read from
//!   the content after every event (shadcn's `has-data-checked:` tint). The
//!   fade runs on the same `cubic-bezier(0.0, 0.0, 0.2, 1.0)` curve as the
//!   checkbox and switch, so card and control animate as one change.
//!
//! Style sources stay catalog `StyleFn`s, resolved every frame, so palette
//! swaps flow through.

use crate::animation::cubic_bezier;
use crate::core::animation::Easing;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::time::Instant;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::window;
use crate::core::{
    Animation, Background, Border, Color, Element, Event, Layout, Length, Padding, Rectangle,
    Shell, Size, Vector, Widget, overlay,
};
use crate::marker;
use crate::operation::control;

use iced_widget::container::StyleFn;
use icedxdy_core::Theme;

/// The signal a field's label marker records when pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label;

/// The card chrome: the inactive and active styles to cross-fade between.
struct Card<'a> {
    inactive: StyleFn<'a, Theme>,
    active: StyleFn<'a, Theme>,
    padding: Padding,
}

/// A laid-out field: label, control, description and error, composed by
/// the [`Field`](super::Field) builder.
///
/// Generic over `Renderer` so we don't bake in the active iced-renderer
/// fallback type — `iced_widget::Renderer` is a feature-gated alias whose
/// concrete type changes with build features (and is `()` with no
/// renderer feature).
pub struct Field<'a, Message, Renderer = iced_widget::Renderer>
where
    Renderer: crate::core::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    card: Option<Card<'a>>,
}

impl<'a, Message: 'a, Renderer> Field<'a, Message, Renderer>
where
    Renderer: crate::core::Renderer,
{
    /// Wrap the composed field. Its label must sit in a [`marker`] recording
    /// [`Label`].
    pub fn new(content: Element<'a, Message, Theme, Renderer>) -> Self {
        Self {
            content,
            card: None,
        }
    }

    /// Paint card chrome around the field, cross-fading from `inactive` to
    /// `active` while its control is checked.
    pub fn card(
        mut self,
        inactive: StyleFn<'a, Theme>,
        active: StyleFn<'a, Theme>,
        padding: impl Into<icedxdy_core::Padding>,
    ) -> Self {
        let padding: icedxdy_core::Padding = padding.into();
        self.card = Some(Card {
            inactive,
            active,
            padding: padding.into(),
        });
        self
    }

    fn padding(&self) -> Padding {
        self.card
            .as_ref()
            .map_or(Padding::ZERO, |card| card.padding)
    }
}

/// Widget-tree state: the card tint, read from the control.
struct State {
    animation: Animation<bool>,
    now: Option<Instant>,
    /// Whether the control was checked at the last probe; `None` before the
    /// first one, which snaps instead of fading.
    is_active: Option<bool>,
}

impl State {
    fn is_active(&self) -> bool {
        self.is_active.unwrap_or(false)
    }
}

impl<Message, Renderer> Meta for Field<'_, Message, Renderer> where Renderer: crate::core::Renderer {}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Field<'_, Message, Renderer>
where
    Renderer: crate::core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            // Match the inner checkbox / switch curve:
            // cubic-bezier(0, 0, 0.2, 1) — Tailwind v4's `--ease-out`.
            animation: Animation::new(false)
                .very_quick()
                .easing(Easing::Custom(|t| cubic_bezier(0.0, 0.0, 0.2, 1.0, t))),
            now: None,
            is_active: None,
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let padding = self.padding();
        let limits = limits.shrink(padding);
        let child = &mut tree.children[0];
        self.content.layout(child, renderer, &limits);
        child.translation = Vector::new(padding.left, padding.top);

        let size = child.size;
        tree.size = Size::new(size.width + padding.x(), size.height + padding.y());
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
            let state = tree.state.downcast_mut::<State>();

            if let Event::Window(window::Event::RedrawRequested(now)) = event {
                state.now = Some(*now);

                if state.animation.is_animating(*now) {
                    shell.request_redraw();
                }
            }
        }

        let (child_layout, child_tree) = layout
            .iter_mut(&mut tree.children)
            .next()
            .expect("a field has one child");

        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        let mut labels = marker::drain::<Label>();
        self.content
            .operate(child_tree, child_layout, viewport, renderer, &mut labels);

        if !labels.into_signals().is_empty() {
            // The control acts on the activation at its next event.
            let mut activate = control::activate();
            self.content
                .operate(child_tree, child_layout, viewport, renderer, &mut activate);

            if activate.is_done() {
                shell.request_redraw();
            }
        }

        if self.card.is_none() {
            return;
        }

        let mut checked = control::checked();
        self.content
            .operate(child_tree, child_layout, viewport, renderer, &mut checked);
        let is_active = checked.get().unwrap_or(false);

        let state = tree.state.downcast_mut::<State>();

        match state.is_active {
            Some(was_active) if was_active == is_active => {}
            Some(_) => {
                state.is_active = Some(is_active);

                if let Some(now) = state.now {
                    state.animation.go_mut(is_active, now);
                    shell.request_redraw();
                }
            }
            None => {
                state.is_active = Some(is_active);
                state.animation = Animation::new(is_active)
                    .very_quick()
                    .easing(Easing::Custom(|t| cubic_bezier(0.0, 0.0, 0.2, 1.0, t)));
            }
        }
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
            .expect("a field has one child");

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
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            let (child_layout, child_tree) = layout
                .iter_mut(&mut tree.children)
                .next()
                .expect("a field has one child");

            self.content
                .operate(child_tree, child_layout, viewport, renderer, operation);
        });
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        renderer_style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        let Some(card) = &self.card else {
            let (child_layout, child_tree) = layout
                .iter(&tree.children)
                .next()
                .expect("a field has one child");

            self.content.draw(
                child_tree,
                renderer,
                theme,
                renderer_style,
                child_layout,
                cursor,
                viewport,
            );
            return;
        };

        // Resolve both endpoints every frame so palette swaps land
        // immediately without an animation bookkeeping rebuild.
        let off = (card.inactive)(theme);
        let on = (card.active)(theme);

        // Cross-fade only `background` and `border.color` — the same
        // contract the inner checkbox / switch follow. Border width +
        // radius snap to the on/off target (interpolating widths
        // shimmers; radius interpolation is rarely visible anyway).
        let style = match state.now {
            Some(now) if state.animation.is_animating(now) => {
                let bg_color = |bg: Option<Background>| match bg {
                    Some(Background::Color(c)) => c,
                    _ => Color::TRANSPARENT,
                };

                let background = Background::Color(state.animation.interpolate(
                    bg_color(off.background),
                    bg_color(on.background),
                    now,
                ));

                let target_border = if state.animation.value() {
                    on.border
                } else {
                    off.border
                };
                let border = Border {
                    color: state
                        .animation
                        .interpolate(off.border.color, on.border.color, now),
                    ..target_border
                };

                let target = if state.animation.value() { on } else { off };
                iced_widget::container::Style {
                    background: Some(background),
                    border,
                    ..target
                }
            }
            _ => {
                if state.is_active() {
                    on
                } else {
                    off
                }
            }
        };

        if let Some(clipped_viewport) = bounds.intersection(viewport) {
            // Mirror iced's container draw_background — paint the bg fill
            // and border via a single quad. Matches what `style.background`
            // expects.
            if let Some(background) = style.background {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        border: style.border,
                        ..renderer::Quad::default()
                    },
                    background,
                );
            } else if style.border.width > 0.0 {
                // Border-only (no fill) — paint a transparent quad just for
                // its border. iced's `draw_background` does the same when
                // `background` is `None` but the border is non-zero.
                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        border: style.border,
                        ..renderer::Quad::default()
                    },
                    Background::Color(Color::TRANSPARENT),
                );
            }

            let (child_layout, child_tree) = layout
                .iter(&tree.children)
                .next()
                .expect("a field has one child");

            self.content.draw(
                child_tree,
                renderer,
                theme,
                &renderer::Style {
                    text_color: style.text_color.unwrap_or(renderer_style.text_color),
                },
                child_layout,
                cursor,
                &clipped_viewport,
            );
        }
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
            .expect("a field has one child");

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

impl<'a, Message: 'a, Renderer> From<Field<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer + 'a,
{
    fn from(field: Field<'a, Message, Renderer>) -> Self {
        field._boxed()
    }
}
