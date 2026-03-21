// Circular spinner widget.
//
// Based on the loading_spinners example from iced by Héctor Ramón Jiménez (hecrj).
// Original: https://github.com/iced-rs/iced/tree/master/examples/loading_spinners
// License: MIT

mod easing;

use easing::Easing;

use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::widget::tree::{self, Tree};
use iced_core::{
    Element, Event, Layout, Length, Radians, Rectangle, Shell, Size, Vector, Widget,
};
use iced_widget::canvas;
use iced_widget::Renderer;

use icedxdy_core::Theme;

use std::f32::consts::PI;
use std::time::Duration;

const MIN_ANGLE: Radians = Radians(PI / 8.0);
const WRAP_ANGLE: Radians = Radians(2.0 * PI - PI / 4.0);
const BASE_ROTATION_SPEED: u32 = u32::MAX / 80;

/// A circular loading spinner.
///
/// ```ignore
/// use icedxdy::spinner;
///
/// spinner()
/// spinner().size(24).bar_height(2.0)
/// ```
pub struct Spinner<'a> {
    size: f32,
    bar_height: f32,
    easing: &'a Easing,
    cycle_duration: Duration,
    rotation_duration: Duration,
}

impl<'a> Spinner<'a> {
    pub fn new() -> Self {
        Self {
            size: 16.0,
            bar_height: 2.0,
            easing: &easing::STANDARD,
            cycle_duration: Duration::from_millis(600),
            rotation_duration: Duration::from_secs(2),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn bar_height(mut self, bar_height: f32) -> Self {
        self.bar_height = bar_height;
        self
    }

    pub fn cycle_duration(mut self, duration: Duration) -> Self {
        self.cycle_duration = duration / 2;
        self
    }

    pub fn rotation_duration(mut self, duration: Duration) -> Self {
        self.rotation_duration = duration;
        self
    }
}

impl Default for Spinner<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a [`Spinner`] with default settings.
pub fn spinner<'a>() -> Spinner<'a> {
    Spinner::new()
}

// --- Animation state machine ---

#[derive(Clone, Copy)]
enum Animation {
    Expanding {
        start: iced_core::time::Instant,
        progress: f32,
        rotation: u32,
        last: iced_core::time::Instant,
    },
    Contracting {
        start: iced_core::time::Instant,
        progress: f32,
        rotation: u32,
        last: iced_core::time::Instant,
    },
}

impl Default for Animation {
    fn default() -> Self {
        let now = iced_core::time::Instant::now();
        Self::Expanding {
            start: now,
            progress: 0.0,
            rotation: 0,
            last: now,
        }
    }
}

impl Animation {
    fn next(&self, additional_rotation: u32, now: iced_core::time::Instant) -> Self {
        match self {
            Self::Expanding { rotation, .. } => Self::Contracting {
                start: now,
                progress: 0.0,
                rotation: rotation.wrapping_add(additional_rotation),
                last: now,
            },
            Self::Contracting { rotation, .. } => Self::Expanding {
                start: now,
                progress: 0.0,
                rotation: rotation.wrapping_add(
                    BASE_ROTATION_SPEED.wrapping_add(
                        (f64::from(WRAP_ANGLE / (2.0 * Radians::PI)) * u32::MAX as f64) as u32,
                    ),
                ),
                last: now,
            },
        }
    }

    fn start(&self) -> iced_core::time::Instant {
        match self {
            Self::Expanding { start, .. } | Self::Contracting { start, .. } => *start,
        }
    }

    fn last(&self) -> iced_core::time::Instant {
        match self {
            Self::Expanding { last, .. } | Self::Contracting { last, .. } => *last,
        }
    }

    fn timed_transition(
        &self,
        cycle_duration: Duration,
        rotation_duration: Duration,
        now: iced_core::time::Instant,
    ) -> Self {
        let elapsed = now.duration_since(self.start());
        let additional_rotation = ((now - self.last()).as_secs_f32()
            / rotation_duration.as_secs_f32()
            * u32::MAX as f32) as u32;

        if elapsed > cycle_duration {
            self.next(additional_rotation, now)
        } else {
            let progress = elapsed.as_secs_f32() / cycle_duration.as_secs_f32();
            match self {
                Self::Expanding { start, rotation, .. } => Self::Expanding {
                    start: *start,
                    progress,
                    rotation: rotation.wrapping_add(additional_rotation),
                    last: now,
                },
                Self::Contracting { start, rotation, .. } => Self::Contracting {
                    start: *start,
                    progress,
                    rotation: rotation.wrapping_add(additional_rotation),
                    last: now,
                },
            }
        }
    }

    fn rotation(&self) -> f32 {
        match self {
            Self::Expanding { rotation, .. } | Self::Contracting { rotation, .. } => {
                *rotation as f32 / u32::MAX as f32
            }
        }
    }
}

#[derive(Default)]
struct State {
    animation: Animation,
    cache: canvas::Cache,
}

// --- Widget impl ---

impl<'a, Message: 'a> Widget<Message, Theme, Renderer> for Spinner<'a> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(self.size),
            height: Length::Fixed(self.size),
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.size, self.size)
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
        let state = tree.state.downcast_mut::<State>();

        if let Event::Window(iced_core::window::Event::RedrawRequested(now)) = event {
            state.animation = state.animation.timed_transition(
                self.cycle_duration,
                self.rotation_duration,
                *now,
            );
            state.cache.clear();
            shell.request_redraw();
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
        use iced_core::Renderer as _;

        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let palette = theme.palette();

        let track_color = iced_core::Color::TRANSPARENT;
        let bar_color: iced_core::Color = palette.foreground.into();

        let geometry = state.cache.draw(renderer, bounds.size(), |frame| {
            let track_radius = frame.width() / 2.0 - self.bar_height;
            let track_path = canvas::Path::circle(frame.center(), track_radius);

            frame.stroke(
                &track_path,
                canvas::Stroke::default()
                    .with_color(track_color)
                    .with_width(self.bar_height),
            );

            let mut builder = canvas::path::Builder::new();
            let start = Radians(state.animation.rotation() * 2.0 * PI);

            match state.animation {
                Animation::Expanding { progress, .. } => {
                    builder.arc(canvas::path::Arc {
                        center: frame.center(),
                        radius: track_radius,
                        start_angle: start,
                        end_angle: start
                            + MIN_ANGLE
                            + WRAP_ANGLE * self.easing.y_at_x(progress),
                    });
                }
                Animation::Contracting { progress, .. } => {
                    builder.arc(canvas::path::Arc {
                        center: frame.center(),
                        radius: track_radius,
                        start_angle: start + WRAP_ANGLE * self.easing.y_at_x(progress),
                        end_angle: start + MIN_ANGLE + WRAP_ANGLE,
                    });
                }
            }

            frame.stroke(
                &builder.build(),
                canvas::Stroke::default()
                    .with_color(bar_color)
                    .with_width(self.bar_height),
            );
        });

        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            use iced_widget::graphics::geometry::Renderer as _;
            renderer.draw_geometry(geometry);
        });
    }
}

impl<'a, Message: 'a> From<Spinner<'a>> for Element<'a, Message, Theme, Renderer> {
    fn from(spinner: Spinner<'a>) -> Self {
        Self::new(spinner)
    }
}
