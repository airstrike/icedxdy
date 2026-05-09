mod element;
pub mod widget;

pub use widget::{
    AbsoluteOffset, Anchor, AutoScroll, Direction, Rail, RelativeOffset, Scrollbar, Scroller,
    Status, Style, StyleFn, Viewport,
};

use crate::component::{Component, IntoElement};
use iced_core::widget::Id;
use iced_core::Length;
use iced_widget::Renderer;
use icedxdy_core::Theme;

/// The project-wide thin scrollbar shape: 1px-wide rail, 2px scroller.
/// Used as the default for both `Scrollable::new` and `.horizontal()`.
/// Pair with the catalog default in `core::theme::scrollable` for the
/// faint-hairline effect that fades up under hover/drag.
pub fn thin_scrollbar() -> Scrollbar {
    Scrollbar::default().width(1).scroller_width(2)
}

/// Create a [`Scrollable`] wrapping the given content. Mirrors
/// [`iced_widget::scrollable`] but flows through our [`Component`] system
/// so size/weight propagation and the project-wide thin invisible-rail
/// default are picked up automatically.
///
/// Default scrollbar style: 1px rail (transparent), 2px scroller drawn at
/// `palette.muted.foreground` alpha 0.4 (rises to 0.7 on hover, 0.9 when
/// dragged). See the `default` fn in icedxdy_core's `theme/scrollable.rs`
/// for the exact spec.
pub fn scrollable<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Scrollable<'a, Message> {
    Scrollable::new(content)
}

/// User-facing scrollable builder. Mirrors [`iced_widget::scrollable`]'s
/// surface (id / width / height / direction / on_scroll / spacing /
/// auto_scroll / anchor_x / anchor_y) but stores style as
/// `iced_widget::scrollable::StyleFn` so `core::theme::scrollable::*`
/// variants flow through unchanged. Bridge laundering to the vendored
/// widget happens at [`IntoElement::into_element`] time.
pub struct Scrollable<'a, Message> {
    content: Component<'a, Message>,
    id: Option<Id>,
    width: Length,
    height: Length,
    direction: Direction,
    auto_scroll: bool,
    on_scroll: Option<Box<dyn Fn(Viewport) -> Message + 'a>>,
    style: Option<iced_widget::scrollable::StyleFn<'static, Theme>>,
}

impl<'a, Message: 'a> Scrollable<'a, Message> {
    /// Create a new scrollable wrapping the given content. Default
    /// direction is vertical with a 1px-wide rail and 2px scroller —
    /// the project's thin invisible-rail spec, replacing iced's stock
    /// 10px default. Pair with the default theme style and the result
    /// is a faint hairline that fades up under hover/drag.
    pub fn new(content: impl Into<Component<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            id: None,
            width: Length::Shrink,
            height: Length::Shrink,
            direction: Direction::Vertical(thin_scrollbar()),
            auto_scroll: false,
            on_scroll: None,
            style: None,
        }
    }

    /// Set this scrollable's widget [`Id`] — required for
    /// `scroll_into_view` and other Id-based operations.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the layout width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the layout height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Configure the scroll direction (Vertical, Horizontal, or Both).
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = direction.into();
        self
    }

    /// Shorthand for switching to a horizontal direction with the
    /// project-wide thin scrollbar shape (1px rail, 2px scroller).
    pub fn horizontal(self) -> Self {
        self.direction(Direction::Horizontal(thin_scrollbar()))
    }

    /// Set a callback fired whenever the scrollable scrolls; receives the
    /// new [`Viewport`].
    pub fn on_scroll(mut self, f: impl Fn(Viewport) -> Message + 'a) -> Self
    where
        Message: 'a,
    {
        self.on_scroll = Some(Box::new(f));
        self
    }

    /// Set a spacing between the scrollbar and the contents (embeds the
    /// scrollbar so it takes layout space rather than floating on top).
    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        match &mut self.direction {
            Direction::Horizontal(scrollbar) | Direction::Vertical(scrollbar) => {
                *scrollbar = scrollbar.spacing(spacing);
            }
            Direction::Both { .. } => {}
        }
        self
    }

    /// Whether middle-click should trigger auto-scrolling. Off by default.
    pub fn auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.auto_scroll = auto_scroll;
        self
    }

    /// Anchor the vertical direction's scroll position.
    pub fn anchor_y(mut self, anchor: Anchor) -> Self {
        match &mut self.direction {
            Direction::Vertical(scrollbar) | Direction::Both { vertical: scrollbar, .. } => {
                *scrollbar = scrollbar.anchor(anchor);
            }
            Direction::Horizontal(_) => {}
        }
        self
    }

    /// Anchor the horizontal direction's scroll position.
    pub fn anchor_x(mut self, anchor: Anchor) -> Self {
        match &mut self.direction {
            Direction::Horizontal(scrollbar) | Direction::Both { horizontal: scrollbar, .. } => {
                *scrollbar = scrollbar.anchor(anchor);
            }
            Direction::Vertical(_) => {}
        }
        self
    }

    /// Set the style of this scrollable via a closure or fn-pointer that
    /// returns an [`iced_widget::scrollable::Style`]. Mirrors iced's stock
    /// `Scrollable::style` so existing callsites (and the variant unit
    /// structs in `theme::scrollable` whose `From<X>` impls produce
    /// `iced_widget::scrollable::StyleFn`) flow through naturally.
    pub fn style(
        mut self,
        style: impl Fn(&Theme, iced_widget::scrollable::Status) -> iced_widget::scrollable::Style
        + 'static,
    ) -> Self {
        self.style = Some(Box::new(style));
        self
    }

    /// Set the style class — accepts anything convertible into an
    /// [`iced_widget::scrollable::StyleFn`], which is how variant unit
    /// structs in `theme::scrollable` flow through.
    pub fn class(
        mut self,
        class: impl Into<iced_widget::scrollable::StyleFn<'static, Theme>>,
    ) -> Self {
        self.style = Some(class.into());
        self
    }
}

impl<'a, Message: 'a> From<Scrollable<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(s: Scrollable<'a, Message>) -> Self {
        Box::new(s).into_element()
    }
}

impl<'a, Message: 'a> From<Scrollable<'a, Message>> for crate::Component<'a, Message> {
    fn from(s: Scrollable<'a, Message>) -> Self {
        crate::Component::new(s)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Scrollable<'a, Message> {
    fn size(&mut self, size: crate::Size) {
        self.content.size(size);
    }

    fn weight(&mut self, weight: crate::Weight) {
        self.content.weight(weight);
    }

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        let mut s = widget::Scrollable::with_direction(self.content.into_iced(), self.direction)
            .width(self.width)
            .height(self.height)
            .auto_scroll(self.auto_scroll);

        if let Some(id) = self.id {
            s = s.id(id);
        }
        if let Some(on_scroll) = self.on_scroll {
            s = s.on_scroll(move |v| on_scroll(v));
        }
        if let Some(style) = self.style {
            // Bridge: wrapper's `style` is `iced_widget::scrollable::StyleFn`
            // (so existing `From<Variant> for iced::scrollable::StyleFn` impls
            // in `core::theme::scrollable` flow naturally). At construction we
            // wrap it so the vendored widget receives a `widget::StyleFn`
            // returning our local `Style` via the field-by-field `From` impl.
            s = s.style(move |t: &Theme, status: widget::Status| {
                let iced_status: iced_widget::scrollable::Status = status.into();
                widget::Style::from((style)(t, iced_status))
            });
        }

        s.into()
    }
}
