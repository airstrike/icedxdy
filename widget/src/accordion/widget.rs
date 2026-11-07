//! Vendored disclosure widget for the accordion suite.
//!
//! There is no upstream iced or sweeten widget that fits — the closest
//! precedents are `switch::widget::Toggler` (animated state, but
//! interpolates between two known scalars: thumb-x positions) and
//! `progress::widget::ProgressBar` (animated value, but again a single
//! scalar). Accordion animates the widget's *own height* between 0 and a
//! measured natural body height. The "measure-but-don't-render" piece is
//! resolved by always laying out the body (so iced computes its natural
//! size unconditionally), then constraining the parent layout node's
//! reported height to `trigger_height + interpolate(0, body_height, t)`
//! and clipping the body in `draw()` to the same interpolated band.
//!
//! # Animation contract
//!
//! - **Body open/close**: 200 ms over CSS `ease-out` =
//!   `cubic-bezier(0, 0, 0.58, 1)` — sourced verbatim from
//!   `tw-animate-css`'s `--animate-accordion-down/up` tokens
//!   (`accordion-down|up var(--tw-duration, 200ms) var(--tw-ease, ease-out) ...`).
//!   See `crate::animation::ease_out`. This is intentionally *different*
//!   from the project's button-hover / progress easing
//!   (`cubic-bezier(0.4, 0, 0.2, 1)` over 150ms — shadcn's generic
//!   `transition-all` default): shadcn's accordion ships its own
//!   directional ease via the `tw-animate-css` keyframes, and matching
//!   it requires this curve, not the generic one.
//! - **Chevron swap**: the wrapper builds the chevron icon into the
//!   trigger Row at view time based on `is_open` for the item. No
//!   widget-level rotation animation (shadcn nova ships two icons that
//!   swap via `group-aria-expanded:hidden` — instantaneous). The body
//!   height tween supplies the smooth-state-change visual.
//! - **Disabled**: opacity 50% applied to trigger_text, chevron, body_text
//!   via `Style::disabled_alpha`. Disabled triggers don't toggle.
//!
//! # State
//!
//! Open-state is **external** (passed in via `Accordion::open`), mirroring
//! the switch pattern (`is_toggled: bool`). The widget's tree state only
//! tracks per-item animation bookkeeping: `last_is_open`, `Animation<bool>`,
//! and the cached natural body height. When the parent re-renders with a
//! new `open` set, the widget kicks the relevant animations from `update`
//! and the next layout/draw cycle interpolates accordingly.

use std::time::Duration;

use iced_core::Length::{Fill, Shrink};
use iced_core::animation::Easing;
use iced_core::border::Border;
use iced_core::layout;
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::time::Instant;
use iced_core::touch;
use iced_core::widget::tree::{self, Tree};
use iced_core::widget::{Meta, Operation};
use iced_core::window;
use iced_core::{
    Animation, Background, Color, Element, Event, Layout, Length, Padding, Rectangle, Shell, Size,
    Vector, Widget,
};

// Bring the renderer trait methods (`fill_quad`, `with_layer`) into scope
// for the concrete `iced_widget::Renderer`.
use iced_core::Renderer as _;

use icedxdy_core::Theme;
use icedxdy_core::theme::accordion::{self, Status};

/// Animation duration. Matches `tw-animate-css`'s
/// `--animate-accordion-down/up` token, which defaults to
/// `var(--tw-duration, 200ms)` — this is shadcn's accordion at runtime.
/// Intentionally not the project's 150ms button-hover standard: shadcn's
/// accordion uses its own dedicated keyframe pair, and pixel/ms-faithful
/// parity requires the matching duration.
const TRANSITION_DURATION: Duration = Duration::from_millis(200);

/// Vertical padding around each trigger row (`py-2.5` = 10 px).
const TRIGGER_PADDING_Y: f32 = 10.0;

/// Spacing between the trigger label and the chevron icon (8 px).
const TRIGGER_GAP: f32 = 8.0;

/// Bottom padding inside the body wrapper (`pb-2.5` = 10 px). The top
/// padding is `pt-0` per nova spec — the trigger's own bottom padding
/// already separates label from body.
const BODY_PADDING_BOTTOM: f32 = 10.0;

/// One disclosure item — the trigger row, the body, and a per-item id for
/// open-state matching.
pub struct Item<'a, Message> {
    pub(crate) id: String,
    pub(crate) trigger: Element<'a, Message, Theme, iced_widget::Renderer>,
    pub(crate) chevron_closed: Element<'a, Message, Theme, iced_widget::Renderer>,
    pub(crate) chevron_open: Element<'a, Message, Theme, iced_widget::Renderer>,
    pub(crate) body: Element<'a, Message, Theme, iced_widget::Renderer>,
    pub(crate) disabled: bool,
}

/// Whether multiple items can be open simultaneously.
///
/// `Single { collapsible: false }` is the only mode that can refuse a
/// close click — it ensures one item is always open. Shadcn's
/// `<Accordion type="single">` without `collapsible` matches this. All
/// six bundled examples pass `collapsible`, so the wrapper defaults to
/// `Single { collapsible: true }`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// At most one item open. `collapsible = true` allows zero open.
    Single { collapsible: bool },
    /// Any subset open.
    Multiple,
}

/// The disclosure widget proper. Constructed by the public `accordion()`
/// builder in the parent module; users never name this type directly.
pub struct Accordion<'a, Message> {
    pub(crate) items: Vec<Item<'a, Message>>,
    pub(crate) mode: Mode,
    pub(crate) open: Vec<String>,
    pub(crate) on_toggle: Option<Box<dyn Fn(String) -> Message + 'a>>,
    pub(crate) class: <Theme as Catalog>::Class<'a>,
    pub(crate) outer_padding: Padding,
}

impl<'a, Message> Accordion<'a, Message> {
    /// Resolve whether item `id` is "currently open" from the *external*
    /// open set. Used both at update time (to spot transitions) and at
    /// draw time (to pick which chevron to render).
    fn is_externally_open(&self, id: &str) -> bool {
        self.open.iter().any(|s| s == id)
    }
}

/// Per-item runtime state stored in the widget's tree.
struct ItemState {
    /// Stable id for matching across renders.
    id: String,
    /// Last `is_open` value seen by `update` — when this drifts from the
    /// current external value, kick off the animation.
    last_is_open: bool,
    /// `Animation<bool>` tracks the eased transition between open/closed.
    /// `interpolate(0.0, natural_body_height, now)` yields the displayed
    /// body height.
    animation: Animation<bool>,
    /// Most recent natural body height measured during layout. Cached so
    /// `update`'s redraw-request math (and any future external readers)
    /// can rely on a stable reference height even between layouts.
    natural_height: f32,
    /// Whether the cursor is currently inside this item's trigger row —
    /// drives the catalog's `Status::Hovered` branch.
    hovered: bool,
}

impl ItemState {
    fn seeded(id: String, is_open: bool) -> Self {
        Self {
            id,
            last_is_open: is_open,
            animation: Animation::new(is_open)
                .duration(TRANSITION_DURATION)
                .easing(Easing::Custom(crate::animation::ease_out)),
            natural_height: 0.0,
            hovered: false,
        }
    }
}

/// Tree state for the whole accordion: one [`ItemState`] per item, keyed
/// by id so reordering items between renders preserves animation state.
struct State {
    items: Vec<ItemState>,
    /// Most recent frame timestamp from the runtime. `None` until the
    /// first `RedrawRequested` fires.
    now: Option<Instant>,
}

impl<Message> Meta for Accordion<'_, Message> {}

impl<Message> Widget<Message, Theme, iced_widget::Renderer> for Accordion<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let items = self
            .items
            .iter()
            .map(|item| {
                let is_open = self.is_externally_open(&item.id);
                ItemState::seeded(item.id.clone(), is_open)
            })
            .collect();

        tree::State::new(State { items, now: None })
    }

    fn diff(&mut self, tree: &mut Tree) {
        // One geometry-only tree per item, each holding the item's four
        // widget trees in this order so layout() and draw() can index by
        // slot:
        //   0 — trigger
        //   1 — chevron-closed
        //   2 — chevron-open
        //   3 — body
        tree.children.resize_with(self.items.len(), Tree::empty);

        for (item_tree, item) in tree.children.iter_mut().zip(&mut self.items) {
            item_tree.diff_children(&mut [
                &mut item.trigger,
                &mut item.chevron_closed,
                &mut item.chevron_open,
                &mut item.body,
            ]);
        }

        // Reconcile per-item animation state: items added since the last
        // render get fresh `ItemState`s; items dropped get their state
        // dropped; surviving items keep their animations + caches by
        // walking the existing state vector by id.
        let state = tree.state.downcast_mut::<State>();
        let mut next: Vec<ItemState> = Vec::with_capacity(self.items.len());
        for item in &self.items {
            let is_open = self.is_externally_open(&item.id);
            let existing = state.items.iter().position(|s| s.id == item.id);
            match existing {
                Some(i) => next.push(state.items.swap_remove(i)),
                None => next.push(ItemState::seeded(item.id.clone(), is_open)),
            }
        }
        state.items = next;
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Fill,
            height: Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced_widget::Renderer,
        limits: &layout::Limits,
    ) {
        let limits = limits.width(Fill);

        // Outer padding eats into the available width before children lay out.
        let pad_h = self.outer_padding.left + self.outer_padding.right;
        let pad_v = self.outer_padding.top + self.outer_padding.bottom;
        let inner_limits = limits.shrink(Size::new(pad_h, pad_v));

        let inner_max = inner_limits.max;
        let mut cursor_y = 0.0_f32;
        let mut max_width = 0.0_f32;

        let state = tree.state.downcast_mut::<State>();
        let now_ref = state.now;

        for (item, item_tree) in self.items.iter_mut().zip(tree.children.iter_mut()) {
            // Tree layout: 4 child trees per item — trigger, chevron-closed,
            // chevron-open, body.
            let [
                trigger_tree,
                chevron_closed_tree,
                chevron_open_tree,
                body_tree,
            ] = item_tree.children.as_mut_slice()
            else {
                unreachable!("diff plants four trees per accordion item");
            };

            // ----- chevrons (laid out first so we know their size for the
            // trigger row's width budget) ------------------------------
            let chevron_limits = layout::Limits::new(Size::ZERO, inner_max).loose();
            item.chevron_closed
                .layout(chevron_closed_tree, renderer, &chevron_limits);
            item.chevron_open
                .layout(chevron_open_tree, renderer, &chevron_limits);
            let chevron_closed_size = chevron_closed_tree.size;
            let chevron_open_size = chevron_open_tree.size;
            let chevron_w = chevron_closed_size.width.max(chevron_open_size.width);
            let chevron_h = chevron_closed_size.height.max(chevron_open_size.height);

            // ----- trigger label ---------------------------------------
            // Trigger label takes whatever's left after the chevron + gap.
            let trigger_max_w = (inner_max.width - chevron_w - TRIGGER_GAP).max(0.0);
            let trigger_limits =
                layout::Limits::new(Size::ZERO, Size::new(trigger_max_w, inner_max.height)).loose();
            item.trigger.layout(trigger_tree, renderer, &trigger_limits);

            let trigger_label_h = trigger_tree.size.height;
            let trigger_inner_h = trigger_label_h.max(chevron_h);
            let trigger_h = trigger_inner_h + TRIGGER_PADDING_Y * 2.0;
            let trigger_w = inner_max.width;

            // Center label and chevron vertically inside the trigger row.
            let label_y = TRIGGER_PADDING_Y + (trigger_inner_h - trigger_label_h) / 2.0;
            let chevron_closed_y =
                TRIGGER_PADDING_Y + (trigger_inner_h - chevron_closed_size.height) / 2.0;
            let chevron_open_y =
                TRIGGER_PADDING_Y + (trigger_inner_h - chevron_open_size.height) / 2.0;
            // Anchor chevrons to the right edge of the trigger row.
            let chevron_closed_x = trigger_w - chevron_closed_size.width;
            let chevron_open_x = trigger_w - chevron_open_size.width;

            trigger_tree.translation = Vector::new(0.0, label_y);
            chevron_closed_tree.translation = Vector::new(chevron_closed_x, chevron_closed_y);
            chevron_open_tree.translation = Vector::new(chevron_open_x, chevron_open_y);

            // ----- body --------------------------------------------------
            // Always lay out the body to capture its natural height. Width is
            // the same as the trigger row so child reflow stays consistent
            // across open/close transitions.
            let body_limits =
                layout::Limits::new(Size::ZERO, Size::new(trigger_w, f32::INFINITY)).loose();
            item.body.layout(body_tree, renderer, &body_limits);

            let body_natural_h_inner = body_tree.size.height;
            let body_natural_h = body_natural_h_inner + BODY_PADDING_BOTTOM;

            // Cache for `update`'s redraw math.
            let item_state = state
                .items
                .iter_mut()
                .find(|s| s.id == item.id)
                .expect("ItemState reconciled in diff()");
            item_state.natural_height = body_natural_h;

            // Compute the displayed body height — interpolate between 0
            // and `body_natural_h` over the animation progress.
            let displayed_body_h = match now_ref {
                Some(now) => item_state.animation.interpolate(0.0, body_natural_h, now),
                None => {
                    // Pre-first-frame: snap to the steady-state extreme. The
                    // animation hasn't been able to start ticking yet, so
                    // any in-flight transition won't render until the next
                    // RedrawRequested arrives — we fall back to "wherever
                    // the animation thinks it currently is" via `value()`.
                    if item_state.animation.value() {
                        body_natural_h
                    } else {
                        0.0
                    }
                }
            };

            // Body sits below the trigger row. The body's *own* layout
            // node keeps its full natural height (so child paint coords
            // are stable); we tell the parent layout that the item's
            // outer height is `trigger + displayed_body_h` and clip the
            // body in draw to the same band.
            let body_y = trigger_h;
            // Position content with no top padding — `pt-0` per nova spec.
            body_tree.translation = Vector::new(0.0, body_y);

            let item_outer_h = trigger_h + displayed_body_h;
            let item_outer_w = trigger_w;

            item_tree.size = Size::new(item_outer_w, item_outer_h);
            item_tree.translation =
                Vector::new(self.outer_padding.left, self.outer_padding.top + cursor_y);

            cursor_y += item_outer_h;
            max_width = max_width.max(item_outer_w);
        }

        let total_w = max_width + pad_h;
        let total_h = cursor_y + pad_v;
        tree.size = Size::new(total_w, total_h);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &iced_widget::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        for (item, (item_layout, item_tree)) in self
            .items
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            let mut children = item_layout.iter_mut(&mut item_tree.children);
            let (trigger_layout, trigger_tree) = children.next().expect("trigger layout");
            let _chevron_closed = children.next();
            let _chevron_open = children.next();
            let (body_layout, body_tree) = children.next().expect("body layout");

            item.trigger
                .operate(trigger_tree, trigger_layout, viewport, renderer, operation);
            item.body
                .operate(body_tree, body_layout, viewport, renderer, operation);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &iced_widget::Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        // Animation bookkeeping. Order matters: cache `now` BEFORE the
        // is_open-drift detection below so newly-kicked animations see a
        // valid `now` on the same frame they're started.
        //
        // Body height is animated, and the interpolation runs in `layout`
        // (`displayed_body_h = animation.interpolate(0, natural, now)`).
        // Iced caches layout across frames and only re-runs it when a
        // shell explicitly invalidates — so EVERY frame an animation is
        // in progress we must `invalidate_layout`, otherwise the cached
        // `displayed_body_h` from the last layout pass freezes the body
        // mid-tween. Unlike colour-animating widgets (`button`, `switch`,
        // `progress`) which animate purely in `draw`, accordion's tween
        // is a layout-affecting size, so request_redraw alone is not enough.
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = Some(*now);
            for s in &mut state.items {
                if s.animation.is_animating(*now) {
                    shell.invalidate_layout();
                    shell.request_redraw();
                }
            }
        }

        // Detect external is_open drift and kick the animation.
        for item in &self.items {
            let is_open = self.is_externally_open(&item.id);
            let s = state
                .items
                .iter_mut()
                .find(|s| s.id == item.id)
                .expect("ItemState exists by id (reconciled in diff)");
            if is_open != s.last_is_open {
                if let Some(now) = state.now {
                    s.animation.go_mut(is_open, now);
                    s.last_is_open = is_open;
                }
                shell.invalidate_layout();
                shell.request_redraw();
            }
        }

        // Hover + click handling per item.
        for (item, (item_layout, item_tree)) in self
            .items
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            let mut children = item_layout.iter_mut(&mut item_tree.children);
            let (trigger_layout, trigger_tree) = children.next().expect("trigger layout");
            let (chevron_closed_layout, _) = children.next().expect("chevron-closed layout");
            // Skip chevron_open layout slot — events forward to whichever
            // chevron is currently visible (decided in draw); for hit
            // testing the chevron-closed bounds are a sufficient proxy
            // since both chevrons are anchored to the same row band.
            let _ = children.next();
            let (body_layout, body_tree) = children.next().expect("body layout");

            // The trigger row's clickable bounds = trigger label bounds
            // unioned with the chevron column. We don't want a click at
            // the right edge (over the chevron) to miss because the
            // chevron is laid out separately.
            let trigger_row_bounds = item_layout.bounds();
            let trigger_row_height = trigger_layout.bounds().height + TRIGGER_PADDING_Y * 2.0;
            let trigger_hit = Rectangle {
                x: trigger_row_bounds.x,
                y: trigger_row_bounds.y,
                width: trigger_row_bounds.width,
                height: trigger_row_height
                    .max(chevron_closed_layout.bounds().height + TRIGGER_PADDING_Y * 2.0),
            };

            let s = state
                .items
                .iter_mut()
                .find(|s| s.id == item.id)
                .expect("ItemState exists by id");

            let now_hovered = !item.disabled && cursor.is_over(trigger_hit);
            if now_hovered != s.hovered {
                s.hovered = now_hovered;
                shell.request_redraw();
            }

            // Click → on_toggle dispatch (skipped for disabled items).
            if !item.disabled {
                let pressed =
                    matches!(
                        event,
                        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                    ) || matches!(event, Event::Touch(touch::Event::FingerPressed { .. }));
                if pressed
                    && cursor.is_over(trigger_hit)
                    && let Some(on_toggle) = self.on_toggle.as_ref()
                {
                    // Mode-aware refusal: Single { collapsible: false }
                    // refuses a close click on the currently-open item.
                    // Inline is_externally_open to avoid &self while
                    // self.items is borrowed mut by the surrounding loop.
                    let is_open_now = self.open.iter().any(|s| s == &item.id);
                    let allow = match (self.mode, is_open_now) {
                        // The one refusal case: single + non-collapsible
                        // forbids closing the currently-open item
                        // because there must always be one open.
                        (Mode::Single { collapsible: false }, true) => false,
                        // All three remaining (Mode, is_open) shapes
                        // accept the click; the parent state mutation
                        // and re-render will reflect the new open-set.
                        (Mode::Single { collapsible: false }, false)
                        | (Mode::Single { collapsible: true }, _)
                        | (Mode::Multiple, _) => true,
                    };
                    if allow {
                        shell.publish(on_toggle(item.id.clone()));
                        shell.capture_event();
                    }
                }
            }

            // Forward inner events to trigger / body. We forward
            // unconditionally: the body widget's own layout coords are
            // stable, so even a clipped body (animating closed) still
            // gets event flow — most child widgets (text) ignore mouse
            // events anyway. If the body is fully closed we skip the
            // forward to avoid clicks landing on hidden content.
            item.trigger.update(
                trigger_tree,
                event,
                trigger_layout,
                cursor,
                renderer,
                shell,
                viewport,
            );

            // Skip body event forwarding when fully collapsed (avoids
            // clicks bleeding into hidden content).
            let displayed_body_h = item_layout.bounds().height - trigger_row_height;
            if displayed_body_h > 0.5 {
                item.body.update(
                    body_tree,
                    event,
                    body_layout,
                    cursor,
                    renderer,
                    shell,
                    viewport,
                );
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced_widget::Renderer,
    ) -> mouse::Interaction {
        for (item, (item_layout, item_tree)) in self.items.iter().zip(layout.iter(&tree.children)) {
            let mut children = item_layout.iter(&item_tree.children);
            let _trigger = children.next();
            let _chevron_closed = children.next();
            let _chevron_open = children.next();
            let body = children.next();

            let trigger_row_bounds = item_layout.bounds();
            // Best-effort trigger hit zone — re-derive from trigger label
            // bounds as in `update`. For mouse_interaction we don't need
            // a tight bound; the full row width works.
            let trigger_row_height = if let Some(s) = tree
                .state
                .downcast_ref::<State>()
                .items
                .iter()
                .find(|s| s.id == item.id)
            {
                trigger_row_bounds.height
                    - s.animation.interpolate(
                        0.0,
                        s.natural_height,
                        tree.state
                            .downcast_ref::<State>()
                            .now
                            .unwrap_or_else(Instant::now),
                    )
            } else {
                trigger_row_bounds.height
            };

            let trigger_hit = Rectangle {
                x: trigger_row_bounds.x,
                y: trigger_row_bounds.y,
                width: trigger_row_bounds.width,
                height: trigger_row_height,
            };

            if cursor.is_over(trigger_hit) {
                if item.disabled {
                    return mouse::Interaction::NotAllowed;
                }
                return mouse::Interaction::Pointer;
            }

            // Body hover — pass through to body's own interaction.
            if let Some((body_layout, body_tree)) = body {
                let interaction =
                    item.body
                        .mouse_interaction(body_tree, body_layout, cursor, viewport, renderer);
                if interaction != mouse::Interaction::default() {
                    return interaction;
                }
            }
        }
        mouse::Interaction::default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced_widget::Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        let resolve_style = |status: Status| theme.style(&self.class, status);

        // Outer border for the wrapped accordion ("borders" example).
        // Drawn first so item content paints on top.
        let outer_style = resolve_style(Status::Active { is_open: false });
        if outer_style.border.width > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        width: outer_style.border.width,
                        color: outer_style.border.color.into(),
                        radius: outer_style.border.radius.into(),
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
        }

        let last_index = self.items.len().saturating_sub(1);

        for (item_index, (item, (item_layout, item_tree))) in self
            .items
            .iter()
            .zip(layout.iter(&tree.children))
            .enumerate()
        {
            let mut children = item_layout.iter(&item_tree.children);
            let (trigger_layout, trigger_tree) = children.next().expect("trigger layout");
            let (chevron_closed_layout, chevron_closed_tree) =
                children.next().expect("chevron-closed layout");
            let (chevron_open_layout, chevron_open_tree) =
                children.next().expect("chevron-open layout");
            let (body_layout, body_tree) = children.next().expect("body layout");

            let item_state = state
                .items
                .iter()
                .find(|s| s.id == item.id)
                .expect("ItemState by id");

            // Status resolution mirrors button: hover wins over active,
            // disabled overrides everything. We don't track keyboard
            // focus here yet (no Tab traversal in iced for arbitrary
            // widget content), so Focused is currently unreachable from
            // pure widget-side state — left as a seam for the future.
            let is_open_now = self.is_externally_open(&item.id);
            let status = if item.disabled {
                Status::Disabled {
                    is_open: is_open_now,
                }
            } else if item_state.hovered {
                Status::Hovered {
                    is_open: is_open_now,
                }
            } else {
                Status::Active {
                    is_open: is_open_now,
                }
            };
            let style = resolve_style(status);

            let alpha = if item.disabled {
                style.disabled_alpha
            } else {
                1.0
            };

            // ----- divider ---------------------------------------------
            // Paint the bottom border under every item except the last.
            // shadcn `not-last:border-b`.
            if item_index < last_index && style.divider.a > 0.0 {
                let bounds = item_layout.bounds();
                let line_y = bounds.y + bounds.height - 1.0;
                let line = Rectangle {
                    x: bounds.x,
                    y: line_y,
                    width: bounds.width,
                    height: 1.0,
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: line,
                        ..renderer::Quad::default()
                    },
                    Background::Color(style.divider.into()),
                );
            }

            // ----- focus ring (around the trigger row) -----------------
            if let Some(ring) = style.ring {
                let trigger_row_bounds = Rectangle {
                    x: item_layout.bounds().x,
                    y: item_layout.bounds().y,
                    width: item_layout.bounds().width,
                    height: trigger_layout.bounds().height + TRIGGER_PADDING_Y * 2.0,
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: trigger_row_bounds.x - ring.width,
                            y: trigger_row_bounds.y - ring.width,
                            width: trigger_row_bounds.width + ring.width * 2.0,
                            height: trigger_row_bounds.height + ring.width * 2.0,
                        },
                        border: Border {
                            width: ring.width,
                            color: ring.color.into(),
                            radius: style.trigger_radius.into(),
                        },
                        ..renderer::Quad::default()
                    },
                    Background::Color(Color::TRANSPARENT),
                );
            }

            // ----- trigger label ---------------------------------------
            let label_color = style.trigger_text.scale_alpha(alpha);
            item.trigger.draw(
                trigger_tree,
                renderer,
                theme,
                &renderer::Style {
                    text_color: label_color.into(),
                },
                trigger_layout,
                cursor,
                viewport,
            );

            // ----- chevron (one of two; the other is laid out but skipped) -----
            let chevron_color = style.chevron.scale_alpha(alpha);
            let chevron_defaults = renderer::Style {
                text_color: chevron_color.into(),
            };
            let (chevron_layout, chevron_tree, chevron_element) = if is_open_now {
                (chevron_open_layout, chevron_open_tree, &item.chevron_open)
            } else {
                (
                    chevron_closed_layout,
                    chevron_closed_tree,
                    &item.chevron_closed,
                )
            };
            chevron_element.draw(
                chevron_tree,
                renderer,
                theme,
                &chevron_defaults,
                chevron_layout,
                cursor,
                viewport,
            );

            // ----- body (clipped to the animated height band) ----------
            let item_bounds = item_layout.bounds();
            let trigger_row_height = trigger_layout.bounds().height + TRIGGER_PADDING_Y * 2.0;
            let displayed_body_h = (item_bounds.height - trigger_row_height).max(0.0);
            if displayed_body_h > 0.0 {
                let body_clip = Rectangle {
                    x: item_bounds.x,
                    y: item_bounds.y + trigger_row_height,
                    width: item_bounds.width,
                    height: displayed_body_h,
                };

                let intersected = body_clip.intersection(viewport).unwrap_or(body_clip);
                let body_text_color = style.body_text.scale_alpha(alpha);
                let body_defaults = renderer::Style {
                    text_color: body_text_color.into(),
                };

                renderer.with_layer(intersected, |renderer| {
                    item.body.draw(
                        body_tree,
                        renderer,
                        theme,
                        &body_defaults,
                        body_layout,
                        cursor,
                        viewport,
                    );
                });
            }

            // Suppress unused-variable warnings when `defaults` is the
            // best fallback for the trigger but no current branch uses it
            // directly.
            let _ = defaults;
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &iced_widget::Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, iced_widget::Renderer>> {
        // Collect overlays from every item (trigger + body).
        let mut overlays: Vec<overlay::Element<'b, Message, Theme, iced_widget::Renderer>> =
            Vec::new();

        for (item, (item_layout, item_tree)) in self
            .items
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            let mut children = item_layout.iter_mut(&mut item_tree.children);
            let (trigger_layout, trigger_tree) = children.next().expect("trigger layout");
            let _chevron_closed = children.next();
            let _chevron_open = children.next();
            let (body_layout, body_tree) = children.next().expect("body layout");

            overlays.extend(item.trigger.overlay(
                trigger_tree,
                trigger_layout,
                renderer,
                viewport,
                translation,
                window,
            ));
            overlays.extend(item.body.overlay(
                body_tree,
                body_layout,
                renderer,
                viewport,
                translation,
                window,
            ));
        }

        overlays
    }
}

impl<'a, Message: 'a> From<Accordion<'a, Message>>
    for Element<'a, Message, Theme, iced_widget::Renderer>
{
    fn from(a: Accordion<'a, Message>) -> Self {
        a._boxed()
    }
}

// --- Catalog -----------------------------------------------------------------

/// Theme catalog for the accordion widget.
///
/// Mirrors the shape of `switch::Catalog` / `button::Catalog`: a class
/// type per `'a` and a `style(class, status)` resolver.
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`accordion::Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> accordion::Style;
}

/// Convenience type-alias for the styling closure type.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> accordion::Style + 'a>;
