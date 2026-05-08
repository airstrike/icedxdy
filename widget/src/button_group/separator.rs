//! [`Separator`] for use inside a [`crate::button_group::ButtonGroup`].
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

/// A 1px divider rendered between adjacent buttons in a [`crate::button_group::ButtonGroup`].
///
/// The held [`Orientation`] is the *group's* orientation — the separator is
/// drawn perpendicular to it. Constructed by [`crate::button_group::separator()`]
/// with `Horizontal` as the default; the parent `ButtonGroup` overwrites the
/// field at iteration time once its own orientation is known, so callers
/// never need to spell the axis themselves (matching shadcn's auto-axis
/// derivation from the surrounding `ButtonGroup`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Separator {
    /// The *group's* orientation. The separator draws perpendicular to it.
    pub(super) group_orientation: Orientation,
}

impl Separator {
    /// Internal constructor. The public surface is
    /// [`crate::button_group::separator()`].
    pub(super) fn new() -> Self {
        Self {
            group_orientation: Orientation::Horizontal,
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Separator
where
    Renderer: iced_core::Renderer,
{
    fn size(&self) -> Size<Length> {
        // Perpendicular axis stretches to the cross-axis of the group; the
        // along-axis is fixed at 1px.
        match self.group_orientation {
            Orientation::Horizontal => Size {
                width: Length::Fixed(1.0),
                height: Length::Fill,
            },
            Orientation::Vertical => Size {
                width: Length::Fill,
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
        let bg: iced_core::Color = theme.palette().input.into();

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            Background::Color(bg),
        );
    }
}
