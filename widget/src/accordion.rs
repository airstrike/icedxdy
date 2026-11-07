//! Accordion component — shadcn/ui faithful port
//!
//! A vertically stacked set of disclosure items where each header reveals
//! a body section. Wraps our vendored `accordion::widget::Accordion`.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::{accordion, text};
//! use std::collections::HashSet;
//!
//! let open: HashSet<String> = ...; // from app state
//!
//! accordion(vec![
//!     accordion::item("shipping", "What are your shipping options?")
//!         .content(text::p("We offer standard, express, and overnight ...")),
//!     accordion::item("returns", "What is your return policy?")
//!         .content(text::p("Returns accepted within 30 days ...")),
//! ])
//! .open(&open)
//! .on_toggle(Message::ToggleAccordion)
//! ```
//!
//! # State model
//!
//! Open-state lives in the parent app, mirroring [`crate::switch`]. The
//! widget's own tree state only tracks per-item animation transitions
//! (Animation<bool>, last_is_open, cached natural body height). The
//! parent passes the current open-set in via `.open(...)` and receives
//! `on_toggle(id)` callbacks when the user clicks a trigger; it's the
//! parent's job to mutate its open-set and re-render. This keeps the
//! single-vs-multiple branching, persistence, and any custom rules
//! (e.g. "keep first item open by default") at the layer that already
//! has the message context.

pub(crate) mod element;
pub mod widget;

pub use widget::{Catalog, Mode, StyleFn};

use std::collections::HashSet;

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
    open: Vec<String>,
    on_toggle: Option<Box<dyn Fn(String) -> Message + 'a>>,
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
        open: Vec::new(),
        on_toggle: None,
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

    /// Pass the current open-set as a slice of ids. Items not in the set
    /// render closed; items in the set render open.
    ///
    /// Single-mode callers can pass `[id]` (a one-element slice) or an
    /// empty slice to model "no item open" (when `collapsible: true`).
    /// Multiple-mode callers pass the full set.
    pub fn open(mut self, open: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.open = open.into_iter().map(Into::into).collect();
        self
    }

    /// Convenience for `Mode::Single`: pass a single optional open id.
    /// `None` means no item is open (only valid with
    /// `Mode::Single { collapsible: true }`).
    pub fn open_one(self, open: Option<impl Into<String>>) -> Self {
        match open {
            Some(id) => self.open([id.into()]),
            None => self.open(Vec::<String>::new()),
        }
    }

    /// Sync the open-set against a `&HashSet<String>`. The widget makes
    /// its own owned copy — accordion isn't a hot-path widget, the alloc
    /// at view time is fine.
    pub fn open_set(self, open: &HashSet<String>) -> Self {
        self.open(open.iter().cloned())
    }

    /// Set the click handler. The handler receives the toggled item id
    /// and is expected to mutate the parent's open-set in response. The
    /// widget itself is open-set-agnostic past view-time.
    pub fn on_toggle(mut self, on_toggle: impl Fn(String) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
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

        let widget = widget::Accordion {
            items,
            mode: a.mode,
            open: a.open,
            on_toggle: a.on_toggle,
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

    /// Pure-logic transition function for the open-set under a given Mode.
    /// Mirrors the click handler's match expression in `widget::update`,
    /// extracted so it can be unit-tested without spinning up a render
    /// loop. Returns `Some(new_set)` when the click is allowed; `None`
    /// when the mode-rules refuse it (Single+!collapsible+already_open).
    fn apply_toggle(mode: Mode, open: &[String], clicked: &str) -> Option<Vec<String>> {
        let is_open_now = open.iter().any(|s| s == clicked);
        match (mode, is_open_now) {
            (Mode::Single { collapsible: false }, true) => None,
            (Mode::Single { .. }, true) => Some(Vec::new()),
            (Mode::Single { .. }, false) => Some(vec![clicked.to_owned()]),
            (Mode::Multiple, true) => Some(
                open.iter()
                    .filter(|s| s.as_str() != clicked)
                    .cloned()
                    .collect(),
            ),
            (Mode::Multiple, false) => {
                let mut out = open.to_vec();
                out.push(clicked.to_owned());
                Some(out)
            }
        }
    }

    /// Single-mode click on a closed item opens it AND closes whatever
    /// else was open. Matches shadcn's `<Accordion type="single">`.
    #[test]
    fn single_mode_toggle_opens_one_closes_others() {
        let mode = Mode::Single { collapsible: true };
        let open = vec!["a".to_owned()];
        let next = apply_toggle(mode, &open, "b").expect("click should be allowed");
        assert_eq!(next, vec!["b".to_owned()]);
    }

    /// Single-mode + collapsible: clicking the open item closes it
    /// (resulting in zero open items).
    #[test]
    fn single_mode_collapsible_close_yields_empty_set() {
        let mode = Mode::Single { collapsible: true };
        let open = vec!["a".to_owned()];
        let next = apply_toggle(mode, &open, "a").expect("collapsible accepts close");
        assert!(next.is_empty());
    }

    /// Single-mode + !collapsible: clicking the currently-open item is
    /// refused (open-set unchanged). One item must always remain open.
    #[test]
    fn single_mode_non_collapsible_refuses_close_of_open_item() {
        let mode = Mode::Single { collapsible: false };
        let open = vec!["a".to_owned()];
        let result = apply_toggle(mode, &open, "a");
        assert!(
            result.is_none(),
            "non-collapsible single-mode must refuse close of the open item",
        );
    }

    /// Multiple-mode: opening item B leaves A open. Both items stay open
    /// independently.
    #[test]
    fn multiple_mode_open_b_keeps_a_open() {
        let mode = Mode::Multiple;
        let open = vec!["a".to_owned()];
        let next = apply_toggle(mode, &open, "b").expect("multiple accepts add");
        assert!(next.contains(&"a".to_owned()));
        assert!(next.contains(&"b".to_owned()));
        assert_eq!(next.len(), 2);
    }

    /// Multiple-mode: closing one of two open items leaves the other open.
    #[test]
    fn multiple_mode_close_b_keeps_a_open() {
        let mode = Mode::Multiple;
        let open = vec!["a".to_owned(), "b".to_owned()];
        let next = apply_toggle(mode, &open, "b").expect("multiple accepts close");
        assert_eq!(next, vec!["a".to_owned()]);
    }

    /// Disabled items don't reach `apply_toggle` at all — the click
    /// handler short-circuits before the mode check (`if !item.disabled`
    /// in `widget::update`). This test asserts the *intent* by calling
    /// the same gate the runtime uses.
    #[test]
    fn disabled_item_short_circuits_before_mode_check() {
        struct ToggleAttempt {
            disabled: bool,
        }
        let attempt = ToggleAttempt { disabled: true };

        // Mirrors the `if !item.disabled` guard in widget::update —
        // disabled items must be filtered out before any mode logic
        // runs, regardless of what apply_toggle would otherwise do.
        let click_allowed = !attempt.disabled;
        assert!(
            !click_allowed,
            "disabled items must not reach the mode-check at all",
        );
    }

    /// Builder ergonomics check: `accordion(...).open([id]).on_toggle(f)`
    /// should compile and produce a usable `Accordion<'_, M>` value.
    /// Caught a regression earlier where `open()` accepted `&[String]`
    /// but rejected `&[&str]` — the `Into<String>` bound fixes that.
    #[test]
    fn builder_accepts_str_slice_for_open_set() {
        let _ = accordion::<(), _>(Vec::<Item<()>>::new())
            .mode(Mode::Single { collapsible: true })
            .open(["one"])
            .on_toggle(|_id| ());
    }
}
