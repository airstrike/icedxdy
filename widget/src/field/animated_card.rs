//! Animated container for [`Field`]'s card mode.
//!
//! Wraps a single child with a fade-on-toggle border + bg, driven by an
//! `is_active: bool` flag. When the flag flips, both the border color and
//! the background color cross-fade between two `iced::container::Style`
//! values along the same `cubic-bezier(0.0, 0.0, 0.2, 1.0)` curve as the
//! checkbox + switch, so the card reads as one synchronized state change
//! when an inner control toggles.
//!
//! Stock `iced_widget::container` has no animation hook — its `Catalog` is
//! a pure `Theme -> Style` lookup with no time/`Animation` input. This
//! mini-widget is the smallest scope add-on that gives us the parity:
//! Field stays structural; we only reach for a vendored Widget impl when
//! the catalog can't carry the time signal.
//!
//! Style sources are kept as catalog `StyleFn`s (not pre-resolved values)
//! so palette swaps + variant rewrites still flow through the closure
//! every frame.
//!
//! [`Field`]: super::field::Field

use crate::animation::cubic_bezier;
use crate::core::animation::Easing;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::time::Instant;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Animation, Background, Border, Color, Element, Event, Layout, Length, Padding, Rectangle,
    Shell, Size, Vector, Widget, overlay,
};

use iced_widget::container::StyleFn;
use icedxdy_core::Theme;

/// A container that fades its border + background between two
/// `container::Style` resolutions when `is_active` toggles.
///
/// Generic over `Renderer` so we don't bake in the active iced-renderer
/// fallback type — `iced_widget::Renderer` is a feature-gated alias whose
/// concrete type changes with build features (and is `()` with no
/// renderer feature). The bound `core::Renderer` is enough for our
/// `fill_quad` painter and child draw forwarding.
pub struct AnimatedCard<'a, Message, Renderer = iced_widget::Renderer>
where
    Renderer: crate::core::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    is_active: bool,
    inactive: StyleFn<'a, Theme>,
    active: StyleFn<'a, Theme>,
    padding: Padding,
}

impl<'a, Message: 'a, Renderer> AnimatedCard<'a, Message, Renderer>
where
    Renderer: crate::core::Renderer,
{
    /// Create an animated card. `inactive` and `active` resolve to the
    /// container styles to cross-fade between as `is_active` flips.
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        is_active: bool,
        inactive: StyleFn<'a, Theme>,
        active: StyleFn<'a, Theme>,
    ) -> Self {
        Self {
            content: content.into(),
            is_active,
            inactive,
            active,
            padding: Padding::ZERO,
        }
    }

    /// Sets the padding of the card.
    pub fn padding(mut self, padding: impl Into<icedxdy_core::Padding>) -> Self {
        let padding: icedxdy_core::Padding = padding.into();
        self.padding = padding.into();
        self
    }
}

/// Widget-tree state — animation bookkeeping driven off `is_active`.
struct State {
    animation: Animation<bool>,
    now: Option<Instant>,
    last_is_active: bool,
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for AnimatedCard<'_, Message, Renderer>
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
            animation: Animation::new(self.is_active)
                .very_quick()
                .easing(Easing::Custom(|t| cubic_bezier(0.0, 0.0, 0.2, 1.0, t))),
            now: None,
            last_is_active: self.is_active,
        })
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.shrink(self.padding);
        let content_node =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &limits);
        let size = content_node.size();
        let padded_size = Size::new(
            size.width + self.padding.x(),
            size.height + self.padding.y(),
        );
        layout::Node::with_children(
            padded_size,
            vec![content_node.move_to((self.padding.left, self.padding.top))],
        )
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = Some(*now);

            if state.animation.is_animating(*now) {
                shell.request_redraw();
            }
        }

        if self.is_active != state.last_is_active {
            state.last_is_active = self.is_active;

            if let Some(now) = state.now {
                state.animation.go_mut(self.is_active, now);
                shell.request_redraw();
            }
        }

        // Always pass the event through to the child — the card chrome is
        // purely visual; toggling logic lives on the inner widget.
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        // Resolve both endpoints every frame so palette swaps land
        // immediately without an animation bookkeeping rebuild.
        let off = (self.inactive)(theme);
        let on = (self.active)(theme);

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
                if self.is_active {
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

            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                &renderer::Style {
                    text_color: style.text_color.unwrap_or(renderer_style.text_color),
                },
                layout.children().next().unwrap(),
                cursor,
                &clipped_viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a, Renderer> From<AnimatedCard<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer + 'a,
{
    fn from(card: AnimatedCard<'a, Message, Renderer>) -> Self {
        Element::new(card)
    }
}
