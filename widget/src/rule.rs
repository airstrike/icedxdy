//! Rules divide space horizontally or vertically.
//!
//! Our Rule extends iced's with `Background` support (gradients).

use iced_core::border;
use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::widget::{Meta, Tree};
use iced_core::{
    Background, Border, Color, Element, Layout, Length, Pixels, Rectangle, Size, Widget,
};

/// Creates a horizontal [`Rule`] with the given height.
pub fn horizontal<'a, Theme: Catalog>(height: impl Into<Pixels>) -> Rule<'a, Theme> {
    Rule {
        thickness: Length::Fixed(height.into().0),
        is_vertical: false,
        class: Theme::default(),
    }
}

/// Creates a vertical [`Rule`] with the given width.
pub fn vertical<'a, Theme: Catalog>(width: impl Into<Pixels>) -> Rule<'a, Theme> {
    Rule {
        thickness: Length::Fixed(width.into().0),
        is_vertical: true,
        class: Theme::default(),
    }
}

/// A horizontal or vertical rule for dividing content.
pub struct Rule<'a, Theme = icedxdy_core::Theme>
where
    Theme: Catalog,
{
    thickness: Length,
    is_vertical: bool,
    class: Theme::Class<'a>,
}

impl<'a, Theme: Catalog> Rule<'a, Theme> {
    /// Sets the style of the [`Rule`].
    ///
    /// Accepts any value that converts into the rule's [`StyleFn`] —
    /// typically a variant unit struct, a `Spec`, or `Box::new(closure)`
    /// for ad-hoc styles.
    #[must_use]
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = style.into().into();
        self
    }
}

impl<Theme> Meta for Rule<'_, Theme> where Theme: Catalog {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Rule<'_, Theme>
where
    Renderer: iced_core::Renderer,
    Theme: Catalog,
{
    fn size(&self) -> Size<Length> {
        if self.is_vertical {
            Size {
                width: self.thickness,
                height: Length::Fill,
            }
        } else {
            Size {
                width: Length::Fill,
                height: self.thickness,
            }
        }
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) {
        let size = <Self as Widget<(), Theme, Renderer>>::size(self);
        tree.size = layout::atomic(limits, size.width, size.height);
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let style = theme.style(&self.class);

        let mut bounds = if self.is_vertical {
            let (offset, line_height) = style.fill_mode.fill(bounds.height);
            Rectangle {
                x: bounds.x,
                y: bounds.y + offset,
                width: bounds.width,
                height: line_height,
            }
        } else {
            let (offset, line_width) = style.fill_mode.fill(bounds.width);
            Rectangle {
                x: bounds.x + offset,
                y: bounds.y,
                width: line_width,
                height: bounds.height,
            }
        };

        let edge = style.border.width.max(0.0);
        let inset = if self.is_vertical {
            edge.min(bounds.height / 2.0)
        } else {
            edge.min(bounds.width / 2.0)
        };

        if style.border.color.a > 0.0 && inset > 0.0 {
            let (first, second) = if self.is_vertical {
                (
                    Rectangle {
                        height: inset,
                        ..bounds
                    },
                    Rectangle {
                        y: bounds.y + bounds.height - inset,
                        height: inset,
                        ..bounds
                    },
                )
            } else {
                (
                    Rectangle {
                        width: inset,
                        ..bounds
                    },
                    Rectangle {
                        x: bounds.x + bounds.width - inset,
                        width: inset,
                        ..bounds
                    },
                )
            };

            let background = Background::Color(style.border.color);
            renderer.fill_quad(
                renderer::Quad {
                    bounds: first,
                    border: border::rounded(style.border.radius),
                    snap: style.snap,
                    ..renderer::Quad::default()
                },
                background,
            );
            renderer.fill_quad(
                renderer::Quad {
                    bounds: second,
                    border: border::rounded(style.border.radius),
                    snap: style.snap,
                    ..renderer::Quad::default()
                },
                background,
            );
        }

        if self.is_vertical {
            bounds.y += inset;
            bounds.height = (bounds.height - 2.0 * inset).max(0.0);
        } else {
            bounds.x += inset;
            bounds.width = (bounds.width - 2.0 * inset).max(0.0);
        }

        if bounds.width == 0.0 || bounds.height == 0.0 {
            return;
        }

        if style.snap {
            let unit = 1.0 / renderer.hint_factor().unwrap_or(1.0);
            bounds.width = bounds.width.max(unit);
            bounds.height = bounds.height.max(unit);
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: border::rounded(style.radius),
                snap: style.snap,
                ..renderer::Quad::default()
            },
            style.background,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Rule<'a, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + iced_core::Renderer,
{
    fn from(rule: Rule<'a, Theme>) -> Element<'a, Message, Theme, Renderer> {
        rule._boxed()
    }
}

/// The appearance of a rule.
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    /// The background of the rule (supports gradients).
    pub background: Background,
    /// The radius of the line corners.
    pub radius: border::Radius,
    /// The caps at both ends of the rule's long axis.
    ///
    /// The rule draws its fill inside this width. Leave the color transparent
    /// to keep separators from painting over a parent border; set a color to
    /// intentionally paint the extra pixels at the ends.
    pub border: Border,
    /// The [`FillMode`] of the rule.
    pub fill_mode: FillMode,
    /// Whether the rule should be snapped to the pixel grid.
    pub snap: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: Background::Color(Color::TRANSPARENT),
            radius: 0.0.into(),
            border: Border {
                width: 1.0,
                color: Color::TRANSPARENT,
                ..Border::default()
            },
            fill_mode: FillMode::Full,
            snap: true,
        }
    }
}

/// The fill mode of a rule.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillMode {
    /// Fill the whole length of the container.
    Full,
    /// Fill a percent of the length of the container (centered).
    Percent(f32),
    /// Uniform offset from each end.
    Padded(u16),
    /// Different offset on each end. First = top or left.
    AsymmetricPadding(u16, u16),
}

impl FillMode {
    /// Return the starting offset and length of the rule.
    pub fn fill(&self, space: f32) -> (f32, f32) {
        match *self {
            FillMode::Full => (0.0, space),
            FillMode::Percent(percent) => {
                if percent >= 100.0 {
                    (0.0, space)
                } else {
                    let w = (space * percent / 100.0).round();
                    (((space - w) / 2.0).round(), w)
                }
            }
            FillMode::Padded(padding) => {
                let p = padding as f32;
                let w = (space - p * 2.0).max(0.0);
                (p, w)
            }
            FillMode::AsymmetricPadding(first, second) => {
                let w = (space - first as f32 - second as f32).max(0.0);
                (first as f32, w)
            }
        }
    }
}

/// The theme catalog of a [`Rule`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// A styling function for a [`Rule`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for icedxdy_core::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme| Style {
            background: theme.neutral.border.into(),
            ..Style::default()
        })
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}
