//! A layout-only wrapper that constrains its single child to a fixed
//! width-to-height ratio.
//!
//! Mirrors shadcn's `<AspectRatio ratio={16 / 9}>...</AspectRatio>` (a thin
//! Radix wrapper). By default the widget consumes whatever width its parent
//! provides (`Length::Fill`) and reports `width / ratio` as its height —
//! matching Radix's behavior of `padding-bottom: (1 / ratio) * 100%` on the
//! inner wrapper, which grows the box to the desired ratio relative to the
//! caller-supplied width.
//!
//! There's no theme catalog here: aspect-ratio is purely structural. Style
//! the child (e.g. `bg-muted`, `rounded-lg`) at the call site, just as
//! shadcn's examples do.
//!
//! ```ignore
//! use icedxdy::widget::aspect_ratio;
//!
//! aspect_ratio(my_image, 16.0 / 9.0).max_width(384.0)
//! ```
//!
//! # Width control
//!
//! Use `.max_width(N)` to cap the width — this mirrors shadcn's
//! `max-w-sm` / `max-w-[10rem]` parent-width idioms without needing a
//! wrapping `container(...).max_width(...)`. Wrapping aspect_ratio in a
//! `Shrink`-width container is actively wrong: iced's `Limits::width(Shrink)`
//! sets `compression.width = true`, which collapses our `Length::Fill`
//! request to the (zero) intrinsic width — leaving the child invisible.
//! `.max_width()` here is the right knob for the "needs a known width
//! upstream" rule.
//!
//! `.width(Length)` is also available for explicit `Length::Fixed(n)` /
//! `Length::FillPortion(n)` cases.

use iced_core::layout;
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::widget::{Meta, Operation, Tree};
use iced_core::{Element, Event, Layout, Length, Pixels, Rectangle, Shell, Size, Vector, Widget};
use iced_widget::Renderer;

use icedxdy_core::Theme;

use crate::component::Component;

/// A widget that constrains its child to a `width : height` ratio.
///
/// `ratio` follows the shadcn / CSS convention: it's `width / height` (so
/// `16.0 / 9.0` is widescreen, `9.0 / 16.0` is portrait). The widget
/// claims the available width (`Length::Fill`) by default and reports
/// `width / ratio` for its height. Use [`AspectRatio::max_width`] or
/// [`AspectRatio::width`] to control the width directly — wrapping in a
/// `Shrink`-width container collapses the layout to zero (see module docs).
pub struct AspectRatio<'a, Message, Renderer = iced_widget::Renderer>
where
    Renderer: iced_core::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    ratio: f32,
    width: Length,
    max_width: f32,
}

impl<'a, Message, Renderer> AspectRatio<'a, Message, Renderer>
where
    Renderer: iced_core::Renderer,
{
    /// Wrap `content` and constrain it to the given `width / height` ratio.
    pub fn new(content: Element<'a, Message, Theme, Renderer>, ratio: f32) -> Self {
        Self {
            content,
            ratio,
            width: Length::Fill,
            max_width: f32::INFINITY,
        }
    }

    /// Set the width policy for this aspect_ratio box. Defaults to
    /// `Length::Fill`. Use `Length::Fixed(n)` for an explicit pixel width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Cap the resolved width at `max_width` pixels. Mirrors shadcn's
    /// `max-w-sm` / `max-w-[10rem]` parent-width idioms — the equivalent
    /// of wrapping in `container(...).max_width(N)` but without the
    /// width-compression footgun (see module docs).
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }
}

/// Constrain `content` to a `width / height` ratio at layout time.
///
/// Mirrors shadcn's `<AspectRatio ratio={...}>` — pass the ratio as
/// `width / height` (e.g. `16.0 / 9.0` for widescreen).
pub fn aspect_ratio<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
    ratio: f32,
) -> AspectRatio<'a, Message> {
    AspectRatio::new(content.into().into_iced(), ratio)
}

impl<Message, Renderer> Meta for AspectRatio<'_, Message, Renderer> where
    Renderer: iced_core::Renderer
{
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for AspectRatio<'_, Message, Renderer>
where
    Renderer: iced_core::Renderer,
{
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        // Report the configured width (Fill by default); height is derived
        // in `layout` from the resolved width and the ratio. `Shrink` here
        // lets parents that measure us in a height-Shrink context (the
        // common case — Column, card preview) accept whatever height we
        // report.
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        // Compute the width directly from the limits + the configured
        // width / max_width. We deliberately bypass `Limits::resolve` —
        // when the parent is a `Shrink`-width container (the common
        // wrapping idiom), the parent's `Limits::width(Shrink)` flips
        // `compression.width = true`, and a `Length::Fill` request would
        // then collapse to the (zero) intrinsic width. Aspect-ratio's
        // whole job is to *project* a width into a height; an empty
        // intrinsic width breaks that contract. So we read `limits.max`
        // directly and cap at `max_width`.
        let safe_ratio = if self.ratio.is_finite() && self.ratio > 0.0 {
            self.ratio
        } else {
            // Fall back to a 1:1 box when the caller passes a non-positive or
            // non-finite ratio. Avoids divide-by-zero / NaN propagating into
            // the layout tree.
            1.0
        };

        let width = match self.width {
            Length::Fixed(n) => n,
            // Fill / FillPortion / Shrink: use the available max, capped
            // at the user-supplied `max_width` (defaults to INFINITY).
            _ => limits.bounds().width,
        };
        let width = width.min(self.max_width).max(0.0);
        // Be defensive against an unbounded parent (no `max_width` and
        // `limits.max.width = INFINITY`). Without a known width we cannot
        // produce a sensible height; report a zero box and let the empty
        // child render — matches CSS `aspect-ratio` in the same edge case.
        let width = if width.is_finite() { width } else { 0.0 };
        let height = (width / safe_ratio).max(0.0);

        // If the height would exceed the available height, scale down both
        // axes proportionally so the box fits — same behavior CSS
        // aspect-ratio gets in a doubly-constrained flex item. Without
        // this, portrait ratios overflow card-shaped previews.
        let max_height = limits.bounds().height;
        let (width, height) = if max_height.is_finite() && height > max_height {
            let scaled = max_height * safe_ratio;
            (scaled.max(0.0), max_height.max(0.0))
        } else {
            (width, height)
        };

        let child_size = Size::new(width, height);

        // Tight-limit the child to the computed box so it has exactly the
        // ratio-constrained space to lay out in.
        let child_limits = layout::Limits::new(child_size, child_size);
        let child = &mut tree.children[0];
        self.content.layout(child, renderer, &child_limits);
        child.translation = Vector::ZERO;

        tree.size = child_size;
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
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content
            .update(tree, event, layout, cursor, renderer, shell, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (layout, tree) = layout.iter(&tree.children).next().unwrap();

        self.content
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content
            .operate(tree, layout, viewport, renderer, operation);
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
        let (layout, tree) = layout.iter(&tree.children).next().unwrap();

        self.content.draw(
            tree,
            renderer,
            theme,
            renderer_style,
            layout,
            cursor,
            viewport,
        );
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
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content
            .overlay(tree, layout, renderer, viewport, translation, window)
    }
}

impl<'a, Message: 'a> From<AspectRatio<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(widget: AspectRatio<'a, Message, Renderer>) -> Self {
        widget._boxed()
    }
}

impl<'a, Message: 'a> From<AspectRatio<'a, Message, Renderer>> for Component<'a, Message> {
    fn from(widget: AspectRatio<'a, Message, Renderer>) -> Self {
        Component::new(Element::from(widget))
    }
}
