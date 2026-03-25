use crate::element::{Element, IntoElement};
use crate::Size;
use icedxdy_core::Theme;
use iced_widget::Renderer;

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
            $(Into::<$crate::element::Element<'_, _>>::into($child)),+
        ])
    };
}

/// A row of icedxdy elements that participates in size propagation.
pub struct Row<'a, Message> {
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
}

impl<'a, Message: 'a> Row<'a, Message> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: None,
        }
    }

    pub fn with_children(children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        Self {
            children: children.into_iter().collect(),
            spacing: None,
        }
    }

    /// Push our own Element.
    pub fn push(mut self, child: Element<'a, Message>) -> Self {
        self.children.push(child);
        self
    }

    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        self.spacing = Some(spacing.into().0);
        self
    }
}

// Our Row → iced Element (so it can be used in iced's column![], container, etc.)
impl<'a, Message: 'a> From<Row<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(row: Row<'a, Message>) -> Self {
        Box::new(row).into_element()
    }
}

impl<'a, Message: 'a> From<Row<'a, Message>> for crate::Element<'a, Message> {
    fn from(r: Row<'a, Message>) -> Self {
        crate::Element::new(r)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Row<'a, Message> {
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
        let mut row = iced_widget::Row::new()
            .spacing(self.spacing.unwrap_or(8.0))
            .align_y(iced_core::Alignment::Center);

        for child in self.children {
            row = row.push(child.into_iced());
        }

        row.into()
    }
}
