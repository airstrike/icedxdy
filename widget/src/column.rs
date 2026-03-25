// FIXME: make tracking generic at compile time — default to no-tracking,
// opt-in to position tracking via generics. For now, always track when `.id()` is set.

use crate::element::{Element, IntoElement};
use crate::operation::position;
use crate::Size;
use icedxdy_core::Theme;
use iced_core::alignment::{self, Alignment};
use iced_core::layout::{self, flex, Layout};
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::widget::{Operation, Tree, tree};
use iced_core::{
    Event, Length, Padding, Pixels, Rectangle, Shell, Vector, Widget,
};
use iced_widget::Renderer;

/// Create a [`Column`] with the given children.
///
/// ```ignore
/// column!["hello", text::p("world")]
/// ```
#[macro_export]
macro_rules! column {
    () => {
        $crate::column::Column::new()
    };
    ($($child:expr),+ $(,)?) => {
        $crate::column::Column::with_children([
            $(Into::<$crate::element::Element<'_, _>>::into($child)),+
        ])
    };
}

/// A column of icedxdy elements that participates in size propagation
/// and supports position tracking.
pub struct Column<'a, Message> {
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
    max_width: f32,
    id: Option<position::Id>,
    width: Length,
    height: Length,
    padding: Padding,
    align: Alignment,
}

impl<'a, Message: 'a> Column<'a, Message> {
    /// Creates an empty [`Column`].
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: None,
            max_width: f32::INFINITY,
            id: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::ZERO,
            align: Alignment::Start,
        }
    }

    /// Creates a [`Column`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        Self {
            children: children.into_iter().collect(),
            ..Self::new()
        }
    }

    /// Sets the vertical spacing between elements.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into().0);
        self
    }

    /// Sets the maximum width of the [`Column`].
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the [`Id`](position::Id) of the [`Column`] for position tracking.
    ///
    /// When set, the [`Column`] tracks the layout bounds of each child widget.
    /// Use [`position::find_position`] to query a child's position by index.
    pub fn id(mut self, id: position::Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the width of the [`Column`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Column`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`Column`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Adds an element to the [`Column`].
    pub fn push(mut self, child: Element<'a, Message>) -> Self {
        self.children.push(child);
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Column`].
    pub fn align_x(mut self, align: impl Into<alignment::Horizontal>) -> Self {
        self.align = Alignment::from(align.into());
        self
    }
}

impl<'a, Message: 'a> Default for Column<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

// Our Column → iced Element (so it can be used in iced's container, etc.)
impl<'a, Message: 'a> From<Column<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(column: Column<'a, Message>) -> Self {
        Box::new(column).into_element()
    }
}

impl<'a, Message: 'a> From<Column<'a, Message>> for crate::Element<'a, Message> {
    fn from(c: Column<'a, Message>) -> Self {
        crate::Element::new(c)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Column<'a, Message> {
    fn size(&mut self, size: Size) {
        // Set gap from size if user didn't set explicit spacing
        if self.spacing.is_none() {
            self.spacing = Some(size.gap());
        }
        for child in &mut self.children {
            child.size(size);
        }
    }

    fn weight(&mut self, weight: crate::Weight) {
        for child in &mut self.children {
            child.weight(weight);
        }
    }

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        let iced_children: Vec<iced_core::Element<'a, Message, Theme, Renderer>> = self
            .children
            .into_iter()
            .map(|child| child.into_iced())
            .collect();

        let widget = ColumnWidget {
            id: self.id,
            spacing: self.spacing.unwrap_or(8.0),
            max_width: self.max_width,
            width: self.width,
            height: self.height,
            padding: self.padding,
            align: self.align,
            children: iced_children,
        };

        iced_core::Element::new(widget)
    }
}

// ---------------------------------------------------------------------------
// ColumnWidget — the actual Widget impl (private)
// ---------------------------------------------------------------------------

#[derive(Default)]
struct WidgetState {
    positions: position::State,
}

struct ColumnWidget<'a, Message> {
    id: Option<position::Id>,
    spacing: f32,
    max_width: f32,
    width: Length,
    height: Length,
    padding: Padding,
    align: Alignment,
    children: Vec<iced_core::Element<'a, Message, Theme, Renderer>>,
}

impl<Message> Widget<Message, Theme, Renderer> for ColumnWidget<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<WidgetState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(WidgetState::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> iced_core::Size<Length> {
        iced_core::Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.max_width(self.max_width);

        let node = layout::flex::resolve(
            flex::Axis::Vertical,
            renderer,
            &limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            self.align,
            &mut self.children,
            &mut tree.children,
        );

        if self.id.is_some() {
            let state = tree.state.downcast_mut::<WidgetState>();
            state.positions.clear();
            for (i, child) in node.children().iter().enumerate() {
                state.positions.set(i, child.bounds());
            }
        }

        node
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let id = self.id.as_ref().map(|id| &id.0);

        operation.container(id, layout.bounds());
        operation.traverse(&mut |operation| {
            self.children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget_mut()
                        .operate(state, layout, renderer, operation);
                });
        });

        if id.is_some() {
            let state = tree.state.downcast_mut::<WidgetState>();
            operation.custom(id, layout.bounds(), &mut state.positions);
        }
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
        for ((child, state), layout) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                state, event, layout, cursor, renderer, shell, viewport,
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget().mouse_interaction(
                    state, layout, cursor, viewport, renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if let Some(clipped_viewport) = layout.bounds().intersection(viewport) {
            for ((child, state), layout) in self
                .children
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
                .filter(|(_, layout)| layout.bounds().intersects(&clipped_viewport))
            {
                child.as_widget().draw(
                    state, renderer, theme, defaults, layout, cursor, viewport,
                );
            }
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}
