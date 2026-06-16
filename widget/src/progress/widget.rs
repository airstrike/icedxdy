//! Forked progress-bar widget with an internal value-easing animation.
//!
//! This is a sweetened version of [`iced`'s `ProgressBar`] that owns its
//! own [`Animation<f32>`]: each time the parent re-renders with a new
//! `value`, the bar smoothly interpolates from its currently-displayed
//! value to the new target over 150ms using
//! `cubic-bezier(0.4, 0, 0.2, 1)` — Tailwind's `transition-all` default,
//! same easing as our [`button`] hover transitions.
//!
//! Style integrates with [`progress_bar::Catalog`] so the
//! existing OKLCH-native catalog in `theme::progress_bar` continues to
//! drive the visuals. No widget-level palette work happens here — the
//! widget only owns the value transition.
//!
//! [`iced`'s `ProgressBar`]: https://docs.iced.rs/iced/widget/progress_bar/
//! [`button`]: crate::button

use std::ops::RangeInclusive;
use std::time::Duration;

use iced_core::animation::Easing;
use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::time::Instant;
use iced_core::widget::tree::{self, Tree};
use iced_core::window;
use iced_core::{
    self, Animation, Color, Element, Event, Layout, Length, Rectangle, Shell, Size, Widget,
};
use iced_widget::progress_bar;

/// Duration of the value-eased transition. Matches Tailwind's
/// `transition-all` default — what shadcn's `<Progress>` uses on its
/// `<Indicator>` element.
const TRANSITION_DURATION: Duration = Duration::from_millis(150);

/// A self-animating progress bar.
///
/// Whenever the [`value`] prop changes between renders, the widget eases
/// from the currently-displayed value to the new target over 150ms.
/// Reads its style from [`progress_bar::Catalog`] — the same
/// catalog iced's stock `progress_bar` uses, so OKLCH-native styles in
/// `theme::progress_bar` apply unchanged.
///
/// [`value`]: ProgressBar::value
pub struct ProgressBar<'a, Message, Theme = iced_widget::Theme>
where
    Theme: progress_bar::Catalog,
{
    range: RangeInclusive<f32>,
    value: f32,
    length: Length,
    girth: Length,
    class: Theme::Class<'a>,
    /// Optional callback fired when the easing animation finishes — i.e.
    /// when the bar settles at its target value. Receives the at-rest value.
    on_idle: Option<Box<dyn Fn(f32) -> Message + 'a>>,
}

impl<'a, Message, Theme> ProgressBar<'a, Message, Theme>
where
    Theme: progress_bar::Catalog,
{
    /// Default girth (4px) — nova's `h-1`.
    pub const DEFAULT_GIRTH: f32 = 4.0;

    /// Creates a new [`ProgressBar`] over the given range with the given
    /// initial value.
    pub fn new(range: RangeInclusive<f32>, value: f32) -> Self {
        ProgressBar {
            value: value.clamp(*range.start(), *range.end()),
            range,
            length: Length::Fill,
            girth: Length::from(Self::DEFAULT_GIRTH),
            class: Theme::default(),
            on_idle: None,
        }
    }

    /// Sets the bar's length (along the value axis).
    pub fn length(mut self, length: impl Into<Length>) -> Self {
        self.length = length.into();
        self
    }

    /// Sets the bar's girth (across the value axis).
    pub fn girth(mut self, girth: impl Into<Length>) -> Self {
        self.girth = girth.into();
        self
    }

    /// Sets the style of the [`ProgressBar`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> progress_bar::Style + 'a) -> Self
    where
        Theme::Class<'a>: From<progress_bar::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as progress_bar::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`ProgressBar`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets a callback to fire when the easing animation finishes. The
    /// callback receives the at-rest target value. Useful for gating
    /// follow-up actions (dismiss splash, navigate, etc.) on the bar
    /// reaching a particular value.
    #[must_use]
    pub fn on_idle(mut self, on_idle: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_idle = Some(Box::new(on_idle));
        self
    }
}

/// Per-instance animation state stored in the widget tree.
#[derive(Debug, Clone)]
struct State {
    /// The interpolated value being rendered right now.
    animated: Animation<f32>,
    /// Value `on_idle` was last fired for, or `None` while a transition
    /// is awaiting its acknowledging fire. Compared against `self.value`
    /// at idle each frame so the fire survives a missed frame between
    /// `diff` and the next `RedrawRequested`.
    settled_at: Option<f32>,
}

impl State {
    /// Seed the animation at `value` with the shadcn `transition-all` easing.
    /// Called from [`Widget::state`] so the first mount renders at the
    /// initial prop value rather than 0.0 — iced's `Tree::new` does NOT call
    /// `Widget::diff`, so any lazy-init via `diff()` would silently miss.
    fn seeded(value: f32) -> Self {
        Self {
            animated: Animation::new(value)
                .easing(Easing::Custom(
                    // cubic-bezier(0.4, 0, 0.2, 1) — shadcn's transition-all
                    |t| crate::animation::cubic_bezier(0.4, 0.0, 0.2, 1.0, t),
                ))
                .duration(TRANSITION_DURATION),
            settled_at: Some(value),
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ProgressBar<'_, Message, Theme>
where
    Theme: progress_bar::Catalog,
    Renderer: iced_core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::seeded(self.value))
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.length,
            height: self.girth,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.length, self.girth)
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();
        if state.animated.value() != self.value {
            // Re-target without snapping: lilt's `transition` interpolates
            // from the *currently displayed* value (which it tracks
            // internally) to the new target. State::seeded already initialised
            // the at-rest value when this widget first mounted, so we never
            // get here with a stale `0.0` baseline.
            state.animated.go_mut(self.value, Instant::now());
            state.settled_at = None;
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let state = tree.state.downcast_mut::<State>();
            let now_animating = state.animated.is_animating(*now);
            if !now_animating && state.settled_at != Some(self.value) {
                if let Some(on_idle) = &self.on_idle {
                    shell.publish(on_idle(self.value));
                }
                state.settled_at = Some(self.value);
            }
            if now_animating {
                shell.request_redraw();
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let (range_start, range_end) = self.range.clone().into_inner();

        let state = tree.state.downcast_ref::<State>();
        let now = Instant::now();
        let displayed = state
            .animated
            .interpolate_with(|v| v, now)
            .clamp(range_start, range_end);

        let active_progress_length = if range_start >= range_end {
            0.0
        } else {
            bounds.width * (displayed - range_start) / (range_end - range_start)
        };

        let style = theme.style(&self.class);

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle { ..bounds },
                border: style.border,
                ..renderer::Quad::default()
            },
            style.background,
        );

        if active_progress_length > 0.0 {
            let is_complete = displayed >= range_end;
            let mut bar_border = style.border;
            bar_border.color = Color::TRANSPARENT;

            if !is_complete {
                bar_border.radius.top_right = 0.0;
                bar_border.radius.bottom_right = 0.0;
            }

            let bar_bounds = Rectangle {
                width: active_progress_length,
                ..bounds
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: bar_bounds,
                    border: bar_border,
                    ..renderer::Quad::default()
                },
                style.bar,
            );
        }
    }
}

impl<'a, Message, Theme, Renderer> From<ProgressBar<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + progress_bar::Catalog,
    Renderer: 'a + iced_core::Renderer,
{
    fn from(
        progress_bar: ProgressBar<'a, Message, Theme>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(progress_bar)
    }
}
