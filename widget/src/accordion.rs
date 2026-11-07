//! Accordion component — shadcn/ui faithful port
//!
//! A vertically stacked set of disclosure items where each header reveals
//! a body section. Wraps our vendored `accordion::widget::Accordion`.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::{accordion, text};
//!
//! // Uncontrolled: the accordion keeps its own open set.
//! accordion(vec![
//!     accordion::item("shipping", "What are your shipping options?")
//!         .content(text::p("We offer standard, express, and overnight ...")),
//!     accordion::item("returns", "What is your return policy?")
//!         .content(text::p("Returns accepted within 30 days ...")),
//! ])
//! .default_value(["shipping"])
//!
//! // Controlled: the app owns the open set.
//! accordion(items)
//!     .value(&self.open)
//!     .on_value_change(Message::Faq)
//! ```
//!
//! # State model
//!
//! Radix state: uncontrolled with [`default_value`](Accordion::default_value)
//! (the open set lives in the widget tree), or controlled with
//! [`value`](Accordion::value) plus
//! [`on_value_change`](Accordion::on_value_change). Either way the widget
//! applies the [`Mode`] rules itself and publishes the complete new open
//! set.

pub(crate) mod element;
pub mod widget;

pub use widget::{Catalog, Mode, StyleFn};

use iced_core::Padding;
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme;

use crate::Slot;

/// Builder for one disclosure item. Constructed via [`item`].
pub struct Item<'a, Message> {
    id: String,
    trigger: Slot<'a, Message>,
    chevron_closed: Option<Slot<'a, Message>>,
    chevron_open: Option<Slot<'a, Message>>,
    body: Option<Slot<'a, Message>>,
    disabled: bool,
}

impl<'a, Message: 'a> Item<'a, Message> {
    /// Set the body content. Required for the item to render anything
    /// useful below the trigger; an item with no body still toggles but
    /// has nothing to reveal.
    pub fn content(mut self, content: impl Into<Slot<'a, Message>>) -> Self {
        self.body = Some(content.into());
        self
    }

    /// Override the closed-state chevron icon. Defaults to a small text
    /// glyph (`>`) when unset; browser callers wire this to
    /// `icon::chevron_down()`.
    pub fn chevron_closed(mut self, icon: impl Into<Slot<'a, Message>>) -> Self {
        self.chevron_closed = Some(icon.into());
        self
    }

    /// Override the open-state chevron icon. Defaults to a small text
    /// glyph (`v`) when unset; browser callers wire this to
    /// `icon::chevron_up()`.
    pub fn chevron_open(mut self, icon: impl Into<Slot<'a, Message>>) -> Self {
        self.chevron_open = Some(icon.into());
        self
    }

    /// Mark this item disabled. Disabled triggers don't toggle and the
    /// trigger row + body render at 50% opacity per nova's
    /// `disabled:opacity-50`.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Construct a single accordion item with the given stable id and trigger
/// label content.
///
/// `id` is matched against the open-set passed via [`Accordion::open`] to
/// determine open/closed state. Animation state in the widget tree is
/// keyed by id too — reordering items between renders preserves their
/// in-flight transitions.
pub fn item<'a, Message: 'a>(
    id: impl Into<String>,
    trigger: impl Into<Slot<'a, Message>>,
) -> Item<'a, Message> {
    Item {
        id: id.into(),
        trigger: trigger.into(),
        chevron_closed: None,
        chevron_open: None,
        body: None,
        disabled: false,
    }
}

/// Top-level builder for the accordion widget.
pub struct Accordion<'a, Message> {
    items: Vec<Item<'a, Message>>,
    mode: Mode,
    value: Option<Vec<String>>,
    default_value: Vec<String>,
    on_value_change: Option<Box<dyn Fn(Vec<String>) -> Message + 'a>>,
    style: Option<theme::accordion::StyleFn<'a, Theme>>,
    outer_padding: Padding,
}

/// Construct an accordion from an iterable of items.
pub fn accordion<'a, Message: 'a, I>(items: I) -> Accordion<'a, Message>
where
    I: IntoIterator<Item = Item<'a, Message>>,
{
    Accordion {
        items: items.into_iter().collect(),
        mode: Mode::Single { collapsible: true },
        value: None,
        default_value: Vec::new(),
        on_value_change: None,
        style: None,
        outer_padding: Padding::ZERO,
    }
}

impl<'a, Message: 'a> Accordion<'a, Message> {
    /// Set the accordion mode. Defaults to `Mode::Single { collapsible: true }`
    /// — matching shadcn's `<Accordion type="single" collapsible>`.
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// Control the accordion: it always shows `value` open, and a trigger
    /// click only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.value = Some(value.into_iter().map(Into::into).collect());
        self
    }

    /// The items an uncontrolled accordion starts with open. Later rebuilds
    /// keep the set the user toggled to.
    pub fn default_value(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.default_value = value.into_iter().map(Into::into).collect();
        self
    }

    /// Set the message published with the complete new open set on every
    /// accepted trigger click, after the [`Mode`] rules are applied.
    pub fn on_value_change(mut self, on_change: impl Fn(Vec<String>) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Override the accordion's style. Accepts our OKLCH-native variants
    /// (`Default`, `Card`) or any `Spec`.
    pub fn style(mut self, style: impl Into<theme::accordion::StyleFn<'a, Theme>>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Override outer padding. Defaults to zero — the wrapping container
    /// (e.g. card) is responsible for outer breathing room. Surfaced as
    /// a knob so the `borders` example can supply `px-4`.
    pub fn padding(mut self, padding: impl Into<icedxdy_core::Padding>) -> Self {
        let padding: icedxdy_core::Padding = padding.into();
        self.outer_padding = padding.into();
        self
    }
}

impl<'a, Message: 'a> From<Accordion<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(a: Accordion<'a, Message>) -> Self {
        // Resolve each Item into the widget-side struct, baking default
        // chevrons via short text glyphs when the user didn't supply
        // explicit icons.
        use crate::text;
        let items: Vec<widget::Item<'a, Message>> = a
            .items
            .into_iter()
            .map(|it| widget::Item {
                id: it.id,
                trigger: it.trigger.compose(),
                chevron_closed: it
                    .chevron_closed
                    .unwrap_or_else(|| Slot::from(text::Text::new(">")))
                    .compose(),
                chevron_open: it
                    .chevron_open
                    .unwrap_or_else(|| Slot::from(text::Text::new("v")))
                    .compose(),
                body: it.body.unwrap_or_else(|| Slot::from("")).compose(),
                disabled: it.disabled,
            })
            .collect();

        // Resolve the styling class. Per accordion's element.rs comment,
        // the widget's Style IS core's accordion::Style — no `from_core`
        // bridge needed; the core StyleFn is the widget StyleFn.
        let class: <Theme as widget::Catalog>::Class<'a> = match a.style {
            Some(style_fn) => style_fn,
            None => <Theme as widget::Catalog>::default(),
        };

        let is_controlled = a.value.is_some();
        let widget = widget::Accordion {
            items,
            mode: a.mode,
            open: a.value.unwrap_or(a.default_value),
            is_controlled,
            on_value_change: a.on_value_change,
            class,
            outer_padding: a.outer_padding,
        };
        widget.into()
    }
}

impl<'a, Message: 'a> From<Accordion<'a, Message>> for crate::Slot<'a, Message> {
    fn from(widget: Accordion<'a, Message>) -> Self {
        crate::Slot::new(widget)
    }
}

impl<'a, Message: 'a> crate::compose::Compose<Message> for Accordion<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(open: &[&str]) -> Vec<String> {
        open.iter().map(|id| (*id).to_owned()).collect()
    }

    /// Single-mode click on a closed item opens it AND closes whatever
    /// else was open. Matches shadcn's `<Accordion type="single">`.
    #[test]
    fn single_mode_toggle_opens_one_closes_others() {
        let mode = Mode::Single { collapsible: true };
        let next = mode.toggle(&ids(&["a"]), "b");
        assert_eq!(next, Some(ids(&["b"])));
    }

    /// Single-mode + collapsible: clicking the open item closes it
    /// (resulting in zero open items).
    #[test]
    fn single_mode_collapsible_close_yields_empty_set() {
        let mode = Mode::Single { collapsible: true };
        assert_eq!(mode.toggle(&ids(&["a"]), "a"), Some(Vec::new()));
    }

    /// Single-mode + !collapsible: clicking the currently-open item is
    /// refused. One item must always remain open.
    #[test]
    fn single_mode_non_collapsible_refuses_close_of_open_item() {
        let mode = Mode::Single { collapsible: false };
        assert_eq!(mode.toggle(&ids(&["a"]), "a"), None);
        assert_eq!(mode.toggle(&ids(&["a"]), "b"), Some(ids(&["b"])));
    }

    /// Multiple-mode: opening item B leaves A open.
    #[test]
    fn multiple_mode_open_b_keeps_a_open() {
        let next = Mode::Multiple.toggle(&ids(&["a"]), "b");
        assert_eq!(next, Some(ids(&["a", "b"])));
    }

    /// Multiple-mode: closing one of two open items leaves the other open.
    #[test]
    fn multiple_mode_close_b_keeps_a_open() {
        let next = Mode::Multiple.toggle(&ids(&["a", "b"]), "b");
        assert_eq!(next, Some(ids(&["a"])));
    }

    /// The builder accepts `&str` ids and starts uncontrolled at its
    /// default set; `value` makes it controlled.
    #[test]
    fn default_value_seeds_an_uncontrolled_accordion() {
        let uncontrolled = accordion::<(), _>(Vec::<Item<()>>::new()).default_value(["one"]);
        assert_eq!(uncontrolled.value, None);
        assert_eq!(uncontrolled.default_value, ids(&["one"]));

        let controlled = accordion::<(), _>(Vec::<Item<()>>::new())
            .value(["two"])
            .on_value_change(|_open| ());
        assert_eq!(controlled.value, Some(ids(&["two"])));
    }
}
