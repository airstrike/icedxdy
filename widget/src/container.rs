mod element;
pub mod widget;

use crate::Size;
use crate::component::{Component, IntoElement};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::widget::Id;
use iced_core::{Length, Padding, Pixels};
use iced_widget::Renderer;
use iced_widget::container::StyleFn;
use icedxdy_core::Theme;
use icedxdy_core::theme::container::Style;

/// Create a [`Container`] wrapping the given content.
///
/// ```ignore
/// container(text::h1("Hello")).padding(16).center_x(Fill)
/// ```
pub struct Container<'a, Message> {
    content: Component<'a, Message>,
    id: Option<Id>,
    width: Length,
    height: Length,
    padding: Padding,
    align_x: Horizontal,
    align_y: Vertical,
    clip: bool,
    style: Option<StyleFn<'static, Theme>>,
}

impl<'a, Message: 'a> Container<'a, Message> {
    pub fn new(content: impl Into<Component<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            id: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::ZERO,
            align_x: Horizontal::Left,
            align_y: Vertical::Top,
            clip: false,
            style: None,
        }
    }

    /// Set this container's widget [`Id`] — required for
    /// `scroll_into_view` and other Id-based operations.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
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

    /// Caps the [`Container`] width. Folded into `width` as a
    /// [`Length::Bounded`] variant — when chained after `.width(Fill)` or
    /// `.width(Shrink)`, the cap propagates through `Limits` cleanly and
    /// overrides any inherited cross-axis compression from a `Shrink`
    /// ancestor (the iced flex pass-2 collapse-to-zero pathology).
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.width = self.width.max(max_width);
        self
    }

    /// Caps the [`Container`] height. See [`max_width`] for the folding semantics.
    ///
    /// [`max_width`]: Self::max_width
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.height = self.height.max(max_height);
        self
    }

    /// Sets a minimum width on the [`Container`]. Folded into `width` as a
    /// [`Length::Bounded`] variant.
    pub fn min_width(mut self, min_width: impl Into<Pixels>) -> Self {
        self.width = self.width.min(min_width);
        self
    }

    /// Sets a minimum height on the [`Container`]. Folded into `height` as a
    /// [`Length::Bounded`] variant.
    pub fn min_height(mut self, min_height: impl Into<Pixels>) -> Self {
        self.height = self.height.min(min_height);
        self
    }

    pub fn padding(mut self, padding: impl Into<icedxdy_core::Padding>) -> Self {
        let padding: icedxdy_core::Padding = padding.into();
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Container`] and centers its contents horizontally.
    pub fn center_x(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(Horizontal::Center)
    }

    /// Sets the height of the [`Container`] and centers its contents vertically.
    pub fn center_y(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(Vertical::Center)
    }

    /// Sets the content alignment for the horizontal axis.
    pub fn align_x(mut self, alignment: impl Into<Horizontal>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Sets the content alignment for the vertical axis.
    pub fn align_y(mut self, alignment: impl Into<Vertical>) -> Self {
        self.align_y = alignment.into();
        self
    }

    /// Set the style of this container via a `Fn(&Theme) -> Style` closure
    /// or fn-pointer, where `Style` is icedxdy's OKLCH-native [`Style`].
    /// Mirrors iced's stock `Container::style` so bare fn pointers like
    /// `card_preview` coerce directly.
    ///
    /// For variant unit structs that impl `Into<StyleFn>`
    /// (e.g. `theme::container::Card`, `theme::container::CodeBlock`,
    /// or a `Spec` produced by the `+` tweak combinators), use
    /// [`Container::class`] instead.
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'static) -> Self {
        self.style = Some(Box::new(move |t: &Theme| (style)(t).into()));
        self
    }

    /// Set the style class of this container — accepts anything that can
    /// be turned into an [`iced_widget::container::StyleFn`], which is how
    /// the variant unit structs in `theme::container` (and `Spec` values
    /// from the tweak combinators) flow through.
    pub fn class(mut self, class: impl Into<StyleFn<'static, Theme>>) -> Self {
        self.style = Some(class.into());
        self
    }

    /// Sets whether the contents of this container should be clipped on
    /// overflow. Mirrors iced's stock `Container::clip`.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Center this container horizontally and vertically inside the
    /// available space. Shortcut for `.center_x(Fill).center_y(Fill)`.
    pub fn center(self, length: impl Into<Length>) -> Self {
        let l = length.into();
        self.center_x(l).center_y(l)
    }
}

impl<'a, Message: 'a> From<Container<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(c: Container<'a, Message>) -> Self {
        Box::new(c).into_element()
    }
}

impl<'a, Message: 'a> From<Container<'a, Message>> for crate::Component<'a, Message> {
    fn from(c: Container<'a, Message>) -> Self {
        crate::Component::new(c)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Container<'a, Message> {
    fn size(&mut self, size: Size) {
        self.content.size(size);
    }

    fn weight(&mut self, weight: crate::Weight) {
        self.content.weight(weight);
    }

    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let mut c = widget::Container::new(self.content.into_iced())
            .width(self.width)
            .height(self.height)
            .padding(self.padding)
            .align_x(self.align_x)
            .align_y(self.align_y)
            .clip(self.clip);

        if let Some(id) = self.id {
            c = c.id(id);
        }
        if let Some(style) = self.style {
            // Bridge: wrapper's `style` is `iced_widget::container::StyleFn`
            // (so existing variants like `theme::container::CardPreview`
            // flow naturally via their existing `From<X> for iced::StyleFn`
            // impls). At construction we wrap that closure to land in the
            // vendored widget's `Style` via the field-identical `From`.
            let bridge: widget::StyleFn<'static, Theme> =
                Box::new(move |t: &Theme| widget::Style::from((style)(t)));
            c = c.class(bridge);
        }

        c.into()
    }
}
