use crate::Size;
use crate::component::{Component, IntoElement};
use iced_core::Length;
use iced_widget::Renderer;
use icedxdy_core::Theme;

/// Create a [`Row`] with the given children.
///
/// ```ignore
/// row!["hello", icon::arrow_up(), text::p("world")]
/// ```
#[macro_export]
macro_rules! row {
    () => {
        $crate::row::Row::new()
    };
    ($($child:expr),+ $(,)?) => {
        $crate::row::Row::with_children([
            $(Into::<$crate::component::Component<'_, _>>::into($child)),+
        ])
    };
}

/// A row of icedxdy elements that participates in size propagation.
pub struct Row<'a, Message> {
    children: Vec<Component<'a, Message>>,
    spacing: Option<f32>,
    width: Length,
    height: Length,
    align_y: iced_core::Alignment,
}

impl<'a, Message: 'a> Row<'a, Message> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: None,
            width: Length::Shrink,
            height: Length::Shrink,
            align_y: iced_core::Alignment::Center,
        }
    }

    pub fn with_children(children: impl IntoIterator<Item = Component<'a, Message>>) -> Self {
        Self {
            children: children.into_iter().collect(),
            spacing: None,
            width: Length::Shrink,
            height: Length::Shrink,
            align_y: iced_core::Alignment::Center,
        }
    }

    /// Push our own Element.
    pub fn push(mut self, child: Component<'a, Message>) -> Self {
        self.children.push(child);
        self
    }

    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        self.spacing = Some(spacing.into().0);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn align_y(mut self, align: impl Into<iced_core::Alignment>) -> Self {
        self.align_y = align.into();
        self
    }
}

// Our Row → iced Element (so it can be used in iced's column![], container, etc.)
impl<'a, Message: 'a> From<Row<'a, Message>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(row: Row<'a, Message>) -> Self {
        Box::new(row).into_element()
    }
}

impl<'a, Message: 'a> From<Row<'a, Message>> for crate::Component<'a, Message> {
    fn from(r: Row<'a, Message>) -> Self {
        crate::Component::new(r)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Row<'a, Message> {
    fn size(&mut self, size: Size) {
        for child in &mut self.children {
            child.size(size);
        }
    }

    fn weight(&mut self, weight: crate::Weight) {
        for child in &mut self.children {
            child.weight(weight);
        }
    }

    fn line_height(&mut self, line_height: f32) {
        for child in &mut self.children {
            child.line_height(line_height);
        }
    }

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        let mut row = iced_widget::Row::new()
            .spacing(self.spacing.unwrap_or(0.0))
            .width(self.width)
            .height(self.height)
            .align_y(self.align_y);

        for child in self.children {
            row = row.push(child.into_iced());
        }

        row.into()
    }
}
