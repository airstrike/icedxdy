//! Sliders let users set a value by moving an indicator.
//!
//! # Example
//! ```ignore
//! use icedxdy::widget::slider;
//!
//! struct State {
//!    value: f32,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     ValueChanged(f32),
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     slider(0.0..=100.0)
//!         .value(state.value)
//!         .on_value_change(Message::ValueChanged)
//! }
//!
//! fn update(state: &mut State, message: Message) {
//!     match message {
//!         Message::ValueChanged(value) => {
//!             state.value = value;
//!         }
//!     }
//! }
//! ```
use crate::core::animation::Easing;
use crate::core::border::{self, Border};
use crate::core::keyboard;
use crate::core::keyboard::key::{self, Key};
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::time::Instant;
use crate::core::touch;
use crate::core::widget::Meta;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    self, Animation, Background, Color, Event, Layout, Length, Pixels, Point, Rectangle, Shell,
    Size, Theme, Widget,
};

use iced_widget::slider;

use std::ops::RangeInclusive;
use std::time::Duration;

/// An horizontal bar and a handle that selects a single value from a range of
/// values.
///
/// A [`Slider`] will try to fill the horizontal space of its container.
///
/// The [`Slider`] range of numeric values is generic and its step size defaults
/// to 1 unit.
///
/// Note: Under the hood values are converted to/from f64 so only values representable exactly as an f64
/// are possible to select via the slider. However it is likely that the precision of the slider at these
/// scales is already less than the precision lost from the f64 representation.
///
/// # Example
/// ```ignore
/// use icedxdy::widget::slider;
///
/// struct State {
///    value: f32,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ValueChanged(f32),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     slider(0.0..=100.0)
///         .value(state.value)
///         .on_value_change(Message::ValueChanged)
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::ValueChanged(value) => {
///             state.value = value;
///         }
///     }
/// }
/// ```
pub struct Slider<'a, T, Message, Theme = icedxdy_core::Theme>
where
    Theme: Catalog,
{
    range: RangeInclusive<T>,
    step: f64,
    shift_step: Option<f64>,
    value: T,
    /// Whether the value is the app's (`true`) or kept in the widget tree.
    is_controlled: bool,
    default: Option<T>,
    on_change: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_release: Option<Message>,
    width: Length,
    height: f32,
    class: Theme::Class<'a>,
    status: Option<Status>,
    pub(crate) is_disabled: bool,
}

impl<'a, T, Message, Theme> Slider<'a, T, Message, Theme>
where
    T: Copy + PartialOrd,
    Message: Clone,
    Theme: Catalog,
{
    /// The default height of a [`Slider`].
    pub const DEFAULT_HEIGHT: f32 = 16.0;

    /// Creates an uncontrolled [`Slider`] over `range`, starting at its
    /// start.
    pub fn new(range: RangeInclusive<T>) -> Self {
        Slider {
            value: *range.start(),
            is_controlled: false,
            default: None,
            range,
            step: 1.0,
            shift_step: None,
            on_change: None,
            on_release: None,
            width: Length::Fill,
            height: Self::DEFAULT_HEIGHT,
            class: Theme::default(),
            status: None,
            is_disabled: false,
        }
    }

    /// Controls the [`Slider`]: it always shows `value` (clamped to its
    /// range), and dragging only publishes
    /// [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: T) -> Self {
        self.value = self.clamp(value);
        self.is_controlled = true;
        self
    }

    /// The initial value of an uncontrolled [`Slider`]. Later rebuilds keep
    /// the value the user dragged to.
    pub fn default_value(mut self, value: T) -> Self {
        self.value = self.clamp(value);
        self.is_controlled = false;
        self
    }

    /// Sets the message published with the new value while the [`Slider`]
    /// is dragged.
    pub fn on_value_change(mut self, on_change: impl Fn(T) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets whether the [`Slider`] is disabled: it keeps showing its value,
    /// ignores input and dims to half opacity.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    fn clamp(&self, value: T) -> T {
        if value < *self.range.start() {
            *self.range.start()
        } else if value > *self.range.end() {
            *self.range.end()
        } else {
            value
        }
    }

    /// Sets the optional default value for the [`Slider`].
    ///
    /// If set, the [`Slider`] will reset to this value when ctrl-clicked or command-clicked.
    pub fn default(mut self, default: impl Into<T>) -> Self {
        self.default = Some(default.into());
        self
    }

    /// Sets the release message of the [`Slider`].
    /// This is called when the mouse is released from the slider.
    ///
    /// Typically, the user's interaction with the slider is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the slider's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }

    /// Sets the width of the [`Slider`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Slider`].
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().0;
        self
    }

    /// Sets the step size of the [`Slider`].
    pub fn step(mut self, step: impl num_traits::AsPrimitive<f64>) -> Self {
        self.step = step.as_();
        self
    }

    /// Sets the optional "shift" step for the [`Slider`].
    ///
    /// If set, this value is used as the step while the shift key is pressed.
    pub fn shift_step(mut self, shift_step: impl num_traits::AsPrimitive<f64>) -> Self {
        self.shift_step = Some(shift_step.as_());
        self
    }

    /// Sets the style of the [`Slider`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Slider`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<T, Message, Theme> Meta for Slider<'_, T, Message, Theme> where Theme: Catalog {}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Slider<'_, T, Message, Theme>
where
    T: Copy + num_traits::AsPrimitive<f64> + num_traits::FromPrimitive,
    Message: Clone,
    Theme: Catalog,
    Renderer: core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn diff(&mut self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();

        if self.is_controlled {
            state.uncontrolled = None;
        } else if let Some(value) = state.uncontrolled.and_then(T::from_f64) {
            self.value = value;
        }
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Fit,
        }
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) {
        tree.size = layout::atomic(limits, self.width, self.height);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        if self.is_disabled {
            state.is_dragging = false;
        }

        let mut update = || {
            let current_value = self.value;

            let locate = |cursor_position: Point| -> Option<T> {
                let bounds = layout.bounds();

                if cursor_position.x <= bounds.x {
                    Some(*self.range.start())
                } else if cursor_position.x >= bounds.x + bounds.width {
                    Some(*self.range.end())
                } else {
                    let step = if state.keyboard_modifiers.shift() {
                        self.shift_step.unwrap_or(self.step)
                    } else {
                        self.step
                    };

                    let start = (*self.range.start()).as_();
                    let end = (*self.range.end()).as_();

                    let percent = f64::from(cursor_position.x - bounds.x) / f64::from(bounds.width);

                    let steps = (percent * (end - start) / step).round();
                    let value = steps * step + start;

                    T::from_f64(value.min(end))
                }
            };

            let increment = |value: T| -> Option<T> {
                let step = if state.keyboard_modifiers.shift() {
                    self.shift_step.unwrap_or(self.step)
                } else {
                    self.step
                };

                let steps = (value.as_() / step).round();
                let new_value = step * (steps + 1.0);

                if new_value > (*self.range.end()).as_() {
                    return Some(*self.range.end());
                }

                T::from_f64(new_value)
            };

            let decrement = |value: T| -> Option<T> {
                let step = if state.keyboard_modifiers.shift() {
                    self.shift_step.unwrap_or(self.step)
                } else {
                    self.step
                };

                let steps = (value.as_() / step).round();
                let new_value = step * (steps - 1.0);

                if new_value < (*self.range.start()).as_() {
                    return Some(*self.range.start());
                }

                T::from_f64(new_value)
            };

            let change = |new_value: T| {
                if (self.value.as_() - new_value.as_()).abs() > f64::EPSILON {
                    if let Some(on_change) = &self.on_change {
                        shell.publish(on_change(new_value));
                    }

                    if !self.is_controlled || self.on_change.is_some() {
                        self.value = new_value;
                    }
                }
            };

            match &event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if let Some(cursor_position) = cursor.position_over(layout.bounds()) {
                        if state.keyboard_modifiers.command() {
                            let _ = self.default.map(change);
                            state.is_dragging = false;
                        } else {
                            let _ = locate(cursor_position).map(change);
                            state.is_dragging = true;
                        }

                        shell.capture_event();
                    }
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerLifted { .. })
                | Event::Touch(touch::Event::FingerLost { .. })
                    if state.is_dragging =>
                {
                    if let Some(on_release) = self.on_release.clone() {
                        shell.publish(on_release);
                    }
                    state.is_dragging = false;
                }
                Event::Mouse(mouse::Event::CursorMoved { .. })
                | Event::Touch(touch::Event::FingerMoved { .. })
                    if state.is_dragging =>
                {
                    let _ = cursor.observe().position().and_then(locate).map(change);

                    shell.capture_event();
                }
                Event::Mouse(mouse::Event::WheelScrolled { delta })
                    if state.keyboard_modifiers.control() && cursor.is_over(layout.bounds()) =>
                {
                    let delta = match delta {
                        mouse::ScrollDelta::Lines { x: _, y } => y,
                        mouse::ScrollDelta::Pixels { x: _, y } => y,
                    };

                    if *delta < 0.0 {
                        let _ = decrement(current_value).map(change);
                    } else {
                        let _ = increment(current_value).map(change);
                    }

                    shell.capture_event();
                }
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. })
                    if cursor.is_over(layout.bounds()) =>
                {
                    match key {
                        Key::Named(key::Named::ArrowUp) => {
                            let _ = increment(current_value).map(change);
                            shell.capture_event();
                        }
                        Key::Named(key::Named::ArrowDown) => {
                            let _ = decrement(current_value).map(change);
                            shell.capture_event();
                        }
                        _ => (),
                    }
                }
                Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                    state.keyboard_modifiers = *modifiers;
                }
                _ => {}
            }
        };

        if !self.is_disabled {
            update();
        }

        if !self.is_controlled {
            state.uncontrolled = Some(self.value.as_());
        }

        let current_status = if self.is_disabled {
            Status::Disabled
        } else if state.is_dragging {
            Status::Dragged
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered
        } else {
            Status::Active
        };

        let ring_visible = matches!(current_status, Status::Hovered | Status::Dragged);

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = Some(*now);

            if ring_visible != state.ring.value() {
                state.ring.go_mut(ring_visible, *now);
            }

            if state.ring.is_animating(*now) {
                shell.request_redraw();
            }

            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();

        let current_status = self.status.unwrap_or(Status::Active);
        let mut style = theme.style(&self.class, current_status);

        let ring_progress = state
            .now
            .map(|now| state.ring.interpolate(0.0, 1.0, now))
            .unwrap_or_else(|| {
                if matches!(current_status, Status::Hovered | Status::Dragged) {
                    1.0
                } else {
                    0.0
                }
            });

        style.ring = if ring_progress > f32::EPSILON {
            style
                .ring
                .or_else(|| theme.style(&self.class, Status::Hovered).ring)
                .map(|ring| Ring {
                    color: ring.color.scale_alpha(ring_progress),
                    width: ring.width * ring_progress,
                })
        } else {
            None
        };

        let (handle_width, handle_height, handle_border_radius) = match style.handle.shape {
            HandleShape::Circle { radius } => (radius * 2.0, radius * 2.0, radius.into()),
            HandleShape::Rectangle {
                width,
                border_radius,
            } => (f32::from(width), bounds.height, border_radius),
        };

        let value = self.value.as_() as f32;
        let (range_start, range_end) = {
            let (start, end) = self.range.clone().into_inner();

            (start.as_() as f32, end.as_() as f32)
        };

        let offset = if range_start >= range_end {
            0.0
        } else {
            (bounds.width - handle_width) * (value - range_start) / (range_end - range_start)
        };

        let rail_y = bounds.y + bounds.height / 2.0;

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: rail_y - style.rail.width / 2.0,
                    width: offset + handle_width / 2.0,
                    height: style.rail.width,
                },
                border: style.rail.border,
                ..renderer::Quad::default()
            },
            style.rail.backgrounds.0,
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x + offset + handle_width / 2.0,
                    y: rail_y - style.rail.width / 2.0,
                    width: bounds.width - offset - handle_width / 2.0,
                    height: style.rail.width,
                },
                border: style.rail.border,
                ..renderer::Quad::default()
            },
            style.rail.backgrounds.1,
        );

        let handle_bounds = Rectangle {
            x: bounds.x + offset,
            y: rail_y - handle_height / 2.0,
            width: handle_width,
            height: handle_height,
        };

        if let Some(ring) = style.ring
            && ring.width > 0.0
        {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: handle_bounds.x - ring.width,
                        y: handle_bounds.y - ring.width,
                        width: handle_bounds.width + ring.width * 2.0,
                        height: handle_bounds.height + ring.width * 2.0,
                    },
                    border: Border {
                        radius: expand_radius(handle_border_radius, ring.width),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(ring.color),
            );
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds: handle_bounds,
                border: Border {
                    radius: handle_border_radius,
                    width: style.handle.border_width,
                    color: style.handle.border_color,
                },
                ..renderer::Quad::default()
            },
            style.handle.background,
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
}

struct State {
    /// The value of an uncontrolled slider, kept across rebuilds.
    uncontrolled: Option<f64>,
    is_dragging: bool,
    keyboard_modifiers: keyboard::Modifiers,
    ring: Animation<bool>,
    now: Option<Instant>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            uncontrolled: None,
            is_dragging: false,
            keyboard_modifiers: keyboard::Modifiers::default(),
            ring: Animation::new(false)
                .easing(Easing::Custom(|t| {
                    crate::animation::cubic_bezier(0.4, 0.0, 0.2, 1.0, t)
                }))
                .duration(Duration::from_millis(150)),
            now: None,
        }
    }
}

/// The possible status of a [`Slider`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Slider`] can be interacted with.
    Active,
    /// The [`Slider`] is being hovered.
    Hovered,
    /// The [`Slider`] is being dragged.
    Dragged,
    /// The [`Slider`] cannot be interacted with.
    Disabled,
}

/// The appearance of a slider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The colors of the rail of the slider.
    pub rail: Rail,
    /// The appearance of the [`Handle`] of the slider.
    pub handle: Handle,
    /// The hover/focus ring painted around the [`Handle`].
    pub ring: Option<Ring>,
}

impl Style {
    /// Changes the [`HandleShape`] of the [`Style`] to a circle
    /// with the given radius.
    pub fn with_circular_handle(mut self, radius: impl Into<Pixels>) -> Self {
        self.handle.shape = HandleShape::Circle {
            radius: radius.into().0,
        };
        self
    }
}

/// The appearance of a slider rail
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rail {
    /// The backgrounds of the rail of the slider.
    pub backgrounds: (Background, Background),
    /// The width of the stroke of a slider rail.
    pub width: f32,
    /// The border of the rail.
    pub border: Border,
}

/// The appearance of the handle of a slider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Handle {
    /// The shape of the handle.
    pub shape: HandleShape,
    /// The [`Background`] of the handle.
    pub background: Background,
    /// The border width of the handle.
    pub border_width: f32,
    /// The border [`Color`] of the handle.
    pub border_color: Color,
}

/// The appearance of the ring around the handle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    /// The ring color.
    pub color: Color,
    /// The ring width in logical pixels.
    pub width: f32,
}

/// The shape of the handle of a slider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandleShape {
    /// A circular handle.
    Circle {
        /// The radius of the circle.
        radius: f32,
    },
    /// A rectangular shape.
    Rectangle {
        /// The width of the rectangle.
        width: u16,
        /// The border radius of the corners of the rectangle.
        border_radius: border::Radius,
    },
}

/// The theme catalog of a [`Slider`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Slider`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

impl From<slider::Style> for Style {
    fn from(style: slider::Style) -> Self {
        Self {
            rail: Rail {
                backgrounds: style.rail.backgrounds,
                width: style.rail.width,
                border: style.rail.border,
            },
            handle: Handle {
                shape: match style.handle.shape {
                    slider::HandleShape::Circle { radius } => HandleShape::Circle { radius },
                    slider::HandleShape::Rectangle {
                        width,
                        border_radius,
                    } => HandleShape::Rectangle {
                        width,
                        border_radius,
                    },
                },
                background: style.handle.background,
                border_width: style.handle.border_width,
                border_color: style.handle.border_color,
            },
            ring: None,
        }
    }
}

impl From<Style> for slider::Style {
    fn from(style: Style) -> Self {
        Self {
            rail: slider::Rail {
                backgrounds: style.rail.backgrounds,
                width: style.rail.width,
                border: style.rail.border,
            },
            handle: slider::Handle {
                shape: match style.handle.shape {
                    HandleShape::Circle { radius } => slider::HandleShape::Circle { radius },
                    HandleShape::Rectangle {
                        width,
                        border_radius,
                    } => slider::HandleShape::Rectangle {
                        width,
                        border_radius,
                    },
                },
                background: style.handle.background,
                border_width: style.handle.border_width,
                border_color: style.handle.border_color,
            },
        }
    }
}

impl From<Status> for slider::Status {
    fn from(status: Status) -> Self {
        match status {
            Status::Active => slider::Status::Active,
            Status::Hovered => slider::Status::Hovered,
            Status::Dragged => slider::Status::Dragged,
            Status::Disabled => slider::Status::Active,
        }
    }
}

/// The default style of a [`Slider`].
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let color = match status {
        Status::Active | Status::Disabled => palette.primary.base.color,
        Status::Hovered => palette.primary.strong.color,
        Status::Dragged => palette.primary.weak.color,
    };

    Style {
        rail: Rail {
            backgrounds: (color.into(), palette.background.strong.color.into()),
            width: 4.0,
            border: Border {
                radius: 2.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 7.0 },
            background: color.into(),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        },
        ring: None,
    }
}

fn expand_radius(radius: border::Radius, by: f32) -> border::Radius {
    border::Radius {
        top_left: radius.top_left + by,
        top_right: radius.top_right + by,
        bottom_right: radius.bottom_right + by,
        bottom_left: radius.bottom_left + by,
    }
}
