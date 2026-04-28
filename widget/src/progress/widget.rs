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
use iced_core::border::Border;
use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::time::Instant;
use iced_core::widget::tree::{self, Tree};
use iced_core::window;
use iced_core::{
    self, Animation, Color, Element, Event, Layout, Length, Rectangle, Shell,
    Size, Widget,
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
pub struct ProgressBar<'a, Theme = iced_widget::Theme>
where
    Theme: progress_bar::Catalog,
{
    range: RangeInclusive<f32>,
    value: f32,
    length: Length,
    girth: Length,
    class: Theme::Class<'a>,
}

impl<'a, Theme> ProgressBar<'a, Theme>
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
    pub fn style(
        mut self,
        style: impl Fn(&Theme) -> progress_bar::Style + 'a,
    ) -> Self
    where
        Theme::Class<'a>:
            From<progress_bar::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style)
            as progress_bar::StyleFn<'a, Theme>)
            .into();
        self
    }

    /// Sets the style class of the [`ProgressBar`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

/// Per-instance animation state stored in the widget tree.
#[derive(Debug, Clone)]
struct State {
    /// The interpolated value being rendered right now.
    animated: Animation<f32>,
    /// Whether `animated` has been initialized to a real prop value yet.
    /// Until the first `diff()` runs (i.e. the first render observes a
    /// concrete prop), we don't know the starting value. Initialization
    /// happens lazily so the first observed value seeds `animated`
    /// directly without triggering a 0 → first-value sweep.
    initialized: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            animated: Animation::new(0.0)
                .easing(Easing::Custom(
                    // cubic-bezier(0.4, 0, 0.2, 1) — shadcn's transition-all
                    |t| crate::animation::cubic_bezier(0.4, 0.0, 0.2, 1.0, t),
                ))
                .duration(TRANSITION_DURATION),
            initialized: false,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ProgressBar<'_, Theme>
where
    Theme: progress_bar::Catalog,
    Renderer: iced_core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
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
        if !state.initialized {
            // First time we're seeing a prop value — seed `animated` so the
            // initial render does NOT sweep from 0 to the first value.
            state.animated = Animation::new(self.value)
                .easing(Easing::Custom(
                    |t| crate::animation::cubic_bezier(0.4, 0.0, 0.2, 1.0, t),
                ))
                .duration(TRANSITION_DURATION);
            state.initialized = true;
        } else if state.animated.value() != self.value {
            // Re-target without snapping: lilt's `transition` interpolates
            // from the *currently displayed* value (which it tracks
            // internally) to the new target.
            state.animated.go_mut(self.value, Instant::now());
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
            let state = tree.state.downcast_ref::<State>();
            if state.animated.is_animating(*now) {
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
            let bar_bounds = Rectangle {
                width: active_progress_length,
                ..bounds
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: bar_bounds,
                    border: Border {
                        color: Color::TRANSPARENT,
                        ..style.border
                    },
                    ..renderer::Quad::default()
                },
                style.bar,
            );
        }
    }
}

impl<'a, Message, Theme, Renderer> From<ProgressBar<'a, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + progress_bar::Catalog,
    Renderer: 'a + iced_core::Renderer,
{
    fn from(
        progress_bar: ProgressBar<'a, Theme>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(progress_bar)
    }
}
