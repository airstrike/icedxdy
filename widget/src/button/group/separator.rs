//! [`Separator`] for use inside a [`super::ButtonGroup`].
//!
//! Mirrors shadcn's `<ButtonGroupSeparator>`: a 1px line drawn perpendicular
//! to the group's orientation, in the same color as a button outline border
//! (`palette.input` / `palette.border`). The separator's own orientation is
//! the *opposite* of the group's — a horizontal group gets vertical
//! separators between its buttons; a vertical group gets horizontal ones.
//!
//! The separator is a value type — no builder, no theme catalog of its own.
//! Color is read from the active [`Theme`] at draw time via the same accessor
//! the button outline variant uses, so a theme swap reflows automatically.
//!
//! Constructed via [`super::separator()`]; the parent `ButtonGroup` wraps
//! each instance in `super::Item::Separator`.

use iced_core::Length;
use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::widget::Tree;
use iced_core::{Background, Layout, Rectangle, Size, Widget};

use icedxdy_core::Theme;

use super::Orientation;

/// A 1px divider rendered between adjacent buttons in a [`super::ButtonGroup`].
///
/// The held [`Orientation`] is the *group's* orientation — the separator is
/// drawn perpendicular to it. Constructed by [`super::separator()`]
/// with `Horizontal` as the default; the parent `ButtonGroup` overwrites the
/// field at iteration time once its own orientation is known, so callers
/// never need to spell the axis themselves (matching shadcn's auto-axis
/// derivation from the surrounding `ButtonGroup`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Separator {
    /// The *group's* orientation. The separator draws perpendicular to it.
    pub(super) group_orientation: Orientation,
    /// Pixel size for the cross axis (perpendicular to the group's axis).
    /// Set by the parent group at iteration time to the tallest button's
    /// height (horizontal group) or widest button's width (vertical group),
    /// so the separator shrinks to the cluster's natural size rather than
    /// stretching to whatever Fill resolves to in the surrounding container
    /// — matches shadcn's `flex items-stretch` semantics, not `Length::Fill`.
    /// Zero means "no buttons in the group" → atomic 1×1 placeholder.
    pub(super) cross_axis: f32,
}

impl Separator {
    /// Internal constructor. The public surface is
    /// [`super::separator()`].
    pub(super) fn new() -> Self {
        Self {
            group_orientation: Orientation::Horizontal,
            cross_axis: 0.0,
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Separator
where
    Renderer: iced_core::Renderer,
{
    fn size(&self) -> Size<Length> {
        // Perpendicular axis is sized to the cluster's natural max button
        // height/width (set by the parent group); the along-axis is fixed
        // at 1px. Zero cross_axis falls back to 1.0 so the separator stays
        // visible in degenerate "no buttons" cases.
        let cross = if self.cross_axis > 0.0 {
            self.cross_axis
        } else {
            1.0
        };
        match self.group_orientation {
            Orientation::Horizontal => Size {
                width: Length::Fixed(1.0),
                height: Length::Fixed(cross),
            },
            Orientation::Vertical => Size {
                width: Length::Fixed(cross),
                height: Length::Fixed(1.0),
            },
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = <Self as Widget<Message, Theme, Renderer>>::size(self);
        layout::atomic(limits, size.width, size.height)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        // Color matches the outline button's border source: in dark mode
        // `palette.input` (the same OKLCH translucent white the outline button
        // uses); in light mode `palette.border`. shadcn's separator uses
        // `bg-input` directly — same source.
        let bg: iced_core::Color = theme.neutral.border.into();

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            Background::Color(bg),
        );
    }
}
