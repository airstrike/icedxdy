// Circular spinner widget.
//
// Based on the loading_spinners example from iced by Héctor Ramón Jiménez (hecrj).
// Original: https://github.com/iced-rs/iced/tree/master/examples/loading_spinners
// License: MIT

use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::widget::tree::{self, Tree};
use iced_core::{
    Color, Element, Event, Layout, Length, Radians, Rectangle, Shell, Size, Vector, Widget,
};
use iced_widget::Renderer;
use iced_widget::canvas;

use icedxdy_core::Theme;

use std::f32::consts::PI;
use std::time::Duration;

/// The style of a [`Spinner`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    /// The track color (background circle). `None` = transparent.
    pub track_color: Option<Color>,
    /// The bar color (spinning arc). `None` = inherit parent text color.
    pub bar_color: Option<Color>,
}

/// The theme catalog of a [`Spinner`].
pub trait Catalog {
    /// The style class.
    type Class<'a>;

    /// The default class.
    fn default<'a>() -> Self::Class<'a>;

    /// The style for a class.
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// A styling function for a [`Spinner`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// Default spinner style — inherits parent text color.
pub fn default(_theme: &Theme) -> Style {
    Style::default()
}

/// A circular loading spinner.
///
/// ```ignore
/// use icedxdy::widget::spinner;
///
/// spinner()
/// spinner().size(24).bar_height(2.0)
/// ```
pub struct Spinner<'a, Theme = icedxdy_core::Theme>
where
    Theme: Catalog,
{
    size: f32,
    bar_height: f32,
    rotation_duration: Duration,
    class: Theme::Class<'a>,
}

impl<'a> Spinner<'a> {
    pub fn new() -> Self {
        Self {
            size: 16.0,
            bar_height: 2.0,
            rotation_duration: Duration::from_millis(1200),
            class: <Theme as Catalog>::default(),
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

    pub fn rotation_duration(mut self, duration: Duration) -> Self {
        self.rotation_duration = duration;
        self
    }

    /// Set the style.
    ///
    /// Accepts any value that converts into the spinner's [`StyleFn`] —
    /// typically `Box::new(closure)` for ad-hoc styles.
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self {
        self.class = style.into();
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

// --- Animation ---

#[derive(Clone, Copy)]
struct Animation {
    rotation: u32,
    last: iced_core::time::Instant,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            rotation: 0,
            last: iced_core::time::Instant::now(),
        }
    }
}

impl Animation {
    fn advance(&mut self, rotation_duration: Duration, now: iced_core::time::Instant) {
        let dt = (now - self.last).as_secs_f32();
        let additional = (dt / rotation_duration.as_secs_f32() * u32::MAX as f32) as u32;
        self.rotation = self.rotation.wrapping_add(additional);
        self.last = now;
    }

    fn angle(&self) -> f32 {
        self.rotation as f32 / u32::MAX as f32
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
            state.animation.advance(self.rotation_duration, *now);
            state.cache.clear();
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        use iced_core::Renderer as _;

        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let catalog_style = theme.style(&self.class);

        let bar_color = catalog_style.bar_color.unwrap_or(style.text_color);
        let track_color = catalog_style.track_color.unwrap_or(Color::TRANSPARENT);

        let geometry = state.cache.draw(renderer, bounds.size(), |frame| {
            let track_radius = frame.width() / 2.0 - self.bar_height;
            let track_path = canvas::Path::circle(frame.center(), track_radius);

            frame.stroke(
                &track_path,
                canvas::Stroke::default()
                    .with_color(track_color)
                    .with_width(self.bar_height),
            );

            // Fixed arc length (3/4 circle), just rotates
            let start = Radians(state.animation.angle() * 2.0 * PI);
            let arc_length = Radians(PI * 1.75);

            let mut builder = canvas::path::Builder::new();
            builder.arc(canvas::path::Arc {
                center: frame.center(),
                radius: track_radius,
                start_angle: start,
                end_angle: start + arc_length,
            });

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

impl<'a, Message: 'a> From<Spinner<'a>> for crate::Component<'a, Message> {
    fn from(s: Spinner<'a>) -> Self {
        crate::Component::new(Element::new(s))
    }
}
