use crate::element::{Element, IntoElement};
use crate::Size;
use crate::Theme;
use iced_widget::Renderer;

/// A row of icedxdy elements that participates in size propagation.
pub struct Row<'a, Message> {
    children: Vec<Element<'a, Message>>,
    spacing: f32,
}

impl<'a, Message: 'a> Row<'a, Message> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 8.0,
        }
    }

    pub fn with_children(children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        Self {
            children: children.into_iter().collect(),
            spacing: 8.0,
        }
    }

    /// Push our own Element.
    pub fn push(mut self, child: Element<'a, Message>) -> Self {
        self.children.push(child);
        self
    }

    /// Push a raw iced widget (opaque, no size propagation).
    pub fn push_iced(
        mut self,
        child: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.children.push(crate::element::wrap(child));
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
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

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        let mut row = iced_widget::Row::new()
            .spacing(self.spacing)
            .align_y(iced_core::Alignment::Center);

        for child in self.children {
            row = row.push(child.into_iced());
        }

        row.into()
    }
}
