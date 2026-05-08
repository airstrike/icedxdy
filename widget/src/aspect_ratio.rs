//! A layout-only wrapper that constrains its single child to a fixed
//! width-to-height ratio.
//!
//! Mirrors shadcn's `<AspectRatio ratio={16 / 9}>...</AspectRatio>` (a thin
//! Radix wrapper). The widget consumes whatever width its parent provides
//! (`Length::Fill`) and reports `width / ratio` as its height — matching
//! Radix's behavior of `padding-bottom: (1 / ratio) * 100%` on the inner
//! wrapper, which grows the box to the desired ratio relative to the
//! caller-supplied width.
//!
//! There's no theme catalog here: aspect-ratio is purely structural. Style
//! the child (e.g. `bg-muted`, `rounded-lg`) at the call site, just as
//! shadcn's examples do.
//!
//! ```ignore
//! use icedxdy::widget::{aspect_ratio, container};
//! use iced::Fill;
//!
//! container(aspect_ratio(my_image, 16.0 / 9.0)).max_width(384.0)
//! ```
//!
//! # Caveats
//!
//! Like CSS `aspect-ratio`, the widget needs a known width upstream to
//! produce a sensible height. Inside an unbounded width context (e.g. a
//! `Length::Shrink` parent with no max), `Length::Fill` resolves to the
//! widget's intrinsic width — which for aspect_ratio is zero. Wrap in a
//! container with a width or max width to give it something to scale from.

use iced_core::layout;
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::widget::Tree;
use iced_core::{Element, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget};
use iced_widget::Renderer;

use icedxdy_core::Theme;

use crate::component::Component;

/// A widget that constrains its child to a `width : height` ratio.
///
/// `ratio` follows the shadcn / CSS convention: it's `width / height` (so
/// `16.0 / 9.0` is widescreen, `9.0 / 16.0` is portrait). The widget
/// claims the available width (`Length::Fill`) and reports `width / ratio`
/// for its height.
pub struct AspectRatio<'a, Message, Renderer = iced_widget::Renderer>
where
    Renderer: iced_core::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    ratio: f32,
}

impl<'a, Message, Renderer> AspectRatio<'a, Message, Renderer>
where
    Renderer: iced_core::Renderer,
{
    /// Wrap `content` and constrain it to the given `width / height` ratio.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>, ratio: f32) -> Self {
        Self {
            content: content.into(),
            ratio,
        }
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

impl<Message, Renderer> Widget<Message, Theme, Renderer> for AspectRatio<'_, Message, Renderer>
where
    Renderer: iced_core::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        // Fill the available width; height is derived in `layout` from the
        // resolved width and the ratio. `Shrink` here lets parents that
        // measure us in a height-Shrink context (the common case — Column,
        // card preview) accept whatever height we report.
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // Resolve the available width first. The parent decides this — we
        // ask for `Fill`, which collapses to the maximum allowed by the
        // limits unless we're inside a Shrink context (in which case the
        // intrinsic width is zero and the widget effectively disappears,
        // matching CSS aspect-ratio's "needs a known width" rule).
        let resolved = limits.resolve(Length::Fill, Length::Shrink, Size::ZERO);
        let width = resolved.width.max(0.0);
        let safe_ratio = if self.ratio.is_finite() && self.ratio > 0.0 {
            self.ratio
        } else {
            // Fall back to a 1:1 box when the caller passes a non-positive or
            // non-finite ratio. Avoids divide-by-zero / NaN propagating into
            // the layout tree.
            1.0
        };
        let height = (width / safe_ratio).max(0.0);

        let child_size = Size::new(width, height);

        // Tight-limit the child to the computed box so it has exactly the
        // ratio-constrained space to lay out in.
        let child_limits = layout::Limits::new(child_size, child_size);
        let child_node =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &child_limits);

        layout::Node::with_children(child_size, vec![child_node])
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
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().expect("aspect_ratio child layout"),
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
            layout.children().next().expect("aspect_ratio child layout"),
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
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            layout.children().next().expect("aspect_ratio child layout"),
            renderer,
            operation,
        );
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
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            renderer_style,
            layout.children().next().expect("aspect_ratio child layout"),
            cursor,
            viewport,
        );
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
            layout.children().next().expect("aspect_ratio child layout"),
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<AspectRatio<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(widget: AspectRatio<'a, Message, Renderer>) -> Self {
        Element::new(widget)
    }
}

impl<'a, Message: 'a> From<AspectRatio<'a, Message, Renderer>> for Component<'a, Message> {
    fn from(widget: AspectRatio<'a, Message, Renderer>) -> Self {
        Component::new(Element::from(widget))
    }
}
