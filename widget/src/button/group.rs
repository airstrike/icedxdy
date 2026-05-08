//! Button Group — shadcn-faithful sibling-button cluster
//!
//! Visually fuses a set of [`Button`]s along a single axis (horizontal or
//! vertical), collapsing the touching corners so the cluster reads as one
//! pill-shaped group. Mirrors shadcn's `<ButtonGroup>` component.
//!
//! # Children are typed
//!
//! Children arrive as [`Item`] values, NOT as `iced::Element`. The group
//! needs to mutate each button's resolved style (corner-radius collapse) AT
//! iteration time — once a child is converted to an `iced::Element`, its
//! style is locked behind the catalog boundary and cannot be touched.
//!
//! Three input shapes flow into the items vec:
//!
//! - Any [`Button`] — converts to [`Item::Button`] via [`From`].
//! - [`separator()`] — yields an [`Item::Separator`] that the group renders
//!   as a 1px divider perpendicular to its orientation.
//! - The [`button::group!`] macro accepts mixed children and calls `.into()`
//!   on each (matching `row!` / `column!` ergonomics).
//!
//! [`button::group!`]: crate::button_group
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::button;
//!
//! button::group![
//!     button::outline("Copy"),
//!     button::group::separator(),
//!     button::outline("Paste"),
//! ]
//! ```
//!
//! # Edge-radius collapse
//!
//! shadcn does this via Tailwind's child-selector cascade
//! (`[&>*:not(:first-child)]:rounded-l-none`). We reproduce it by walking
//! the items vec, computing each button's [`Position`] (`First`,
//! `Middle`, `Last`, `Only`), and calling [`Button::map_style`] to zero out
//! the appropriate corners on the resolved [`iced_widget::button::Style`].
//! Position counts every child slot (buttons AND separators) — a button
//! immediately after a separator is *not* a "first" button by shadcn's
//! cascade rules.
//!
//! # Orientation
//!
//! Defaults to [`Horizontal`]. Override with [`ButtonGroup::orientation`].
//! The orientation is also passed down to every [`Item::Separator`] so the
//! divider draws perpendicular to the group axis without the caller spelling
//! it.

use iced_core::Length;
use iced_widget::Renderer;
use icedxdy_core::Theme;

use crate::Button;
use crate::Component;
use crate::component::IntoElement;

pub mod separator;

pub use separator::Separator;

/// Layout orientation for a [`ButtonGroup`].
///
/// Mirrors shadcn's `<ButtonGroup orientation="horizontal" | "vertical">`
/// prop. Horizontal is the default per the live docs page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    /// Buttons in a row (default).
    #[default]
    Horizontal,
    /// Buttons in a column.
    Vertical,
}

// Variants reachable at module scope: `button::group::Horizontal` rather
// than `button::group::Orientation::Horizontal`. Mirrors `field`'s pattern.
pub use Orientation::*;

/// Position of a [`Button`] within the group's child sequence — drives
/// which corners get collapsed at radius-stripping time.
///
/// Counts every child slot (buttons AND separators). A button that sits
/// immediately after a separator is NOT [`Position::First`] — the separator
/// occupies that slot. Matches shadcn's `[&>*:not(:first-child)]` semantics:
/// the selector counts every direct DOM child, not just the buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    /// The only child in the group — keeps every corner.
    Only,
    /// First child — keeps "leading" corners (top-left + bottom-left for
    /// horizontal; top-left + top-right for vertical), collapses the
    /// trailing pair.
    First,
    /// Internal child — collapses every corner.
    Middle,
    /// Last child — keeps "trailing" corners, collapses the leading pair.
    Last,
}

/// One slot in a [`ButtonGroup`]'s child sequence.
///
/// Constructed via the [`button::group!`] macro (which calls `.into()` on
/// each child) or directly via [`From`] from a [`Button`] / [`Separator`].
/// Domain-enum shape per the minimap M2 critique: re-styling children means
/// calling [`Button::map_style`] BEFORE the conversion to `iced::Element`,
/// which only the typed enum permits.
///
/// [`button::group!`]: crate::button_group
pub enum Item<'a, Message> {
    /// A button that participates in the cluster — its corners get
    /// collapsed based on neighbors at iteration time.
    Button(Button<'a, Message>),
    /// A vertical-or-horizontal divider drawn between buttons. The
    /// separator's own axis is set to be perpendicular to the group's
    /// orientation when [`ButtonGroup::into_element`] runs.
    Separator(Separator),
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Item<'a, Message> {
    fn from(b: Button<'a, Message>) -> Self {
        Item::Button(b)
    }
}

impl<'a, Message> From<Separator> for Item<'a, Message> {
    fn from(s: Separator) -> Self {
        Item::Separator(s)
    }
}

/// Construct a [`Separator`] item. The group's orientation is patched into
/// the separator at iteration time, so the caller doesn't pass an axis.
pub fn separator() -> Separator {
    Separator::new()
}

/// A horizontally- or vertically-fused cluster of [`Button`]s with optional
/// [`Separator`] dividers.
///
/// Built via [`group()`] or the [`button::group!`] macro. Configures via
/// [`ButtonGroup::orientation`] and [`ButtonGroup::width`].
///
/// [`button::group!`]: crate::button_group
pub struct ButtonGroup<'a, Message> {
    items: Vec<Item<'a, Message>>,
    orientation: Orientation,
    width: Option<Length>,
}

/// Build a [`ButtonGroup`] from an ordered sequence of items.
///
/// Prefer the [`button::group!`] macro — it calls `.into()` on each child
/// so callers don't spell `.into()` per item. This function exists for the
/// macro to delegate to, and for callers building the items vec dynamically.
///
/// [`button::group!`]: crate::button_group
///
/// ```ignore
/// use icedxdy::widget::button;
///
/// button::group![
///     button::outline("Archive"),
///     button::outline("Report"),
///     button::outline("Snooze"),
/// ]
/// ```
pub fn group<'a, Message: Clone + 'a>(
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> ButtonGroup<'a, Message> {
    ButtonGroup {
        items: items.into_iter().collect(),
        orientation: Orientation::default(),
        width: None,
    }
}

impl<'a, Message: Clone + 'a> ButtonGroup<'a, Message> {
    /// Set the layout axis. Defaults to [`Horizontal`].
    ///
    /// Horizontal: buttons sit in a row; corners collapse on the inside
    /// (right edge of the first button, both sides of middle buttons, left
    /// edge of the last button).
    ///
    /// Vertical: buttons stack in a column; corners collapse on the inside
    /// (bottom of the first button, top + bottom of middle buttons, top of
    /// the last button).
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }

    /// Set the outer width of the rendered group. Defaults to shrink-to-content.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }
}

impl<'a, Message: Clone + 'a> From<ButtonGroup<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(g: ButtonGroup<'a, Message>) -> Self {
        Box::new(g).into_element()
    }
}

impl<'a, Message: Clone + 'a> From<ButtonGroup<'a, Message>> for Component<'a, Message> {
    fn from(g: ButtonGroup<'a, Message>) -> Self {
        Component::new(g)
    }
}

impl<'a, Message: Clone + 'a> IntoElement<'a, Message> for ButtonGroup<'a, Message> {
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        let ButtonGroup {
            items,
            orientation,
            width,
        } = *self;

        // Pre-compute the Position label for each Item::Button slot. The
        // walk counts ALL child slots (buttons + separators) — shadcn's
        // `:first-child` / `:last-child` cascade keys off direct DOM
        // children, not just buttons.
        let total = items.len();
        let positions: Vec<Option<Position>> = items
            .iter()
            .enumerate()
            .map(|(i, item)| match item {
                Item::Button(_) => Some(position_at(i, total)),
                Item::Separator(_) => None,
            })
            .collect();

        // Compute the max button height in this group so separators can
        // size their perpendicular axis to a concrete pixel value rather
        // than `Length::Fill`. With Fill, the separator inherits the
        // surrounding container's available cross-axis size and stretches
        // far past the buttons (288px in the browser preview card). With
        // a fixed pixel value matching the tallest button, the separator
        // shrinks to the cluster's natural height — like shadcn's
        // `flex items-stretch` resolves to the tallest sibling.
        let cross_axis_size = items
            .iter()
            .filter_map(|item| match item {
                Item::Button(b) => Some(b.size_height()),
                Item::Separator(_) => None,
            })
            .fold(0.0_f32, f32::max);

        // Convert each item to an iced::Element. Buttons get their style
        // wrapped via `map_style` so the resolved Style has the appropriate
        // corners zeroed; separators get their group_orientation patched.
        let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> = items
            .into_iter()
            .zip(positions.into_iter())
            .map(|(item, pos)| -> iced_core::Element<'a, Message, Theme, Renderer> {
                match item {
                    Item::Button(btn) => {
                        // pos is always Some(_) for Item::Button per the
                        // mapping above; expect makes the invariant explicit.
                        let pos = pos.expect("Item::Button must carry a Position");
                        let btn = btn.map_style(move |style| collapse_corners(style, pos, orientation));
                        btn.into()
                    }
                    Item::Separator(mut sep) => {
                        sep.group_orientation = orientation;
                        sep.cross_axis = cross_axis_size;
                        iced_core::Element::new(sep)
                    }
                }
            })
            .collect();

        // Lay them out along the chosen axis with NO inter-child spacing —
        // the buttons share the seam (matches shadcn's `flex items-stretch`
        // with no `gap`). Cross-axis stretches so vertical groups have
        // consistent button widths and horizontal groups present a
        // consistent height for separator fill.
        let element: iced_core::Element<'a, Message, Theme, Renderer> = match orientation {
            Orientation::Horizontal => iced_widget::Row::with_children(elements)
                .spacing(0)
                .align_y(iced_core::Alignment::Center)
                .into(),
            Orientation::Vertical => iced_widget::Column::with_children(elements)
                .spacing(0)
                .align_x(iced_core::Alignment::Start)
                .into(),
        };

        match width {
            Some(w) => iced_widget::container(element).width(w).into(),
            None => element,
        }
    }
}

/// Compute the [`Position`] of a child at flat index `i` given a total
/// child count `total`. Edge slots collapse to [`Position::Only`] when
/// `total == 1`.
fn position_at(i: usize, total: usize) -> Position {
    if total <= 1 {
        Position::Only
    } else if i == 0 {
        Position::First
    } else if i + 1 == total {
        Position::Last
    } else {
        Position::Middle
    }
}

/// Zero the corners of `style.border.radius` that should collapse for a
/// button at the given position in a group with the given orientation.
///
/// Mirrors shadcn's `[&>*:not(:first-child)]:rounded-l-none` /
/// `:not(:last-child)]:rounded-r-none` cascade for horizontal groups, and
/// the `rounded-t-none` / `rounded-b-none` analogs for vertical.
///
/// Also collapses the inward border so adjacent buttons don't double-stroke
/// the shared seam (`[&>*:not(:first-child)]:border-l-0` in shadcn).
fn collapse_corners(
    mut style: iced_widget::button::Style,
    pos: Position,
    orientation: Orientation,
) -> iced_widget::button::Style {
    let r = &mut style.border.radius;
    match (orientation, pos) {
        (_, Position::Only) => {}
        (Orientation::Horizontal, Position::First) => {
            r.top_right = 0.0;
            r.bottom_right = 0.0;
        }
        (Orientation::Horizontal, Position::Middle) => {
            r.top_left = 0.0;
            r.top_right = 0.0;
            r.bottom_left = 0.0;
            r.bottom_right = 0.0;
            // Avoid the double-stroke seam — drop the leading border edge.
            // The first button still paints its full border, so the seam
            // shows the first button's right edge once (not twice).
            style.border.width = style.border.width.max(0.0);
        }
        (Orientation::Horizontal, Position::Last) => {
            r.top_left = 0.0;
            r.bottom_left = 0.0;
        }
        (Orientation::Vertical, Position::First) => {
            r.bottom_left = 0.0;
            r.bottom_right = 0.0;
        }
        (Orientation::Vertical, Position::Middle) => {
            r.top_left = 0.0;
            r.top_right = 0.0;
            r.bottom_left = 0.0;
            r.bottom_right = 0.0;
        }
        (Orientation::Vertical, Position::Last) => {
            r.top_left = 0.0;
            r.top_right = 0.0;
        }
    }
    style
}

/// Default-variant button-group macro. Children are passed as a comma-list
/// and each child is `Into::<Item<'_, _>>::into`'d before being collected
/// into the items vec. Mirrors `row!` / `column!` ergonomics so callers
/// don't spell `.into()` per child.
///
/// ```ignore
/// use icedxdy::widget::button;
///
/// button::group![
///     button::outline("Archive"),
///     button::outline("Report"),
///     button::outline("Snooze"),
/// ]
///
/// // With a separator in the middle — the separator value is `Into<Item>`.
/// button::group![
///     button::secondary("Copy"),
///     button::group::separator(),
///     button::secondary("Paste"),
/// ]
/// ```
///
/// `#[macro_export]` is crate-root-scoped (Rust limitation on macros), so
/// the macro name at the consuming crate is `button_group!` — re-aliased
/// to `button::group!` via `pub use crate::button_group as group;` in
/// `widget/src/button.rs` (file-based exemption in the pre-commit hook,
/// matching the same trick `button::secondary!` / `button::outline!` use).
#[macro_export]
macro_rules! button_group {
    () => (
        $crate::button::group::group(::core::iter::empty())
    );
    ($($child:expr),+ $(,)?) => (
        $crate::button::group::group([
            $(Into::<$crate::button::group::Item<'_, _>>::into($child)),+
        ])
    );
}

#[cfg(test)]
mod tests {
    //! Position + corner-collapse coverage. The group's `into_element`
    //! pipeline can't easily be inspected after the fact (style closures
    //! are erased into iced's catalog), so the reachable surface here is
    //! the pure helpers that compute child position and zero corner radii.

    use super::*;
    use iced_core::Background;
    use iced_widget::button::Style;

    fn dummy_style() -> Style {
        Style {
            background: Some(Background::Color(iced_core::Color::WHITE)),
            text_color: iced_core::Color::BLACK,
            border: iced_core::Border {
                color: iced_core::Color::BLACK,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: iced_core::Shadow::default(),
            snap: false,
        }
    }

    #[test]
    fn position_at_solo_is_only() {
        assert_eq!(position_at(0, 1), Position::Only);
    }

    #[test]
    fn position_at_pair_is_first_then_last() {
        assert_eq!(position_at(0, 2), Position::First);
        assert_eq!(position_at(1, 2), Position::Last);
    }

    #[test]
    fn position_at_triple_has_middle() {
        assert_eq!(position_at(0, 3), Position::First);
        assert_eq!(position_at(1, 3), Position::Middle);
        assert_eq!(position_at(2, 3), Position::Last);
    }

    #[test]
    fn horizontal_first_keeps_left_zeroes_right() {
        let s = collapse_corners(dummy_style(), Position::First, Orientation::Horizontal);
        assert_eq!(s.border.radius.top_left, 8.0);
        assert_eq!(s.border.radius.bottom_left, 8.0);
        assert_eq!(s.border.radius.top_right, 0.0);
        assert_eq!(s.border.radius.bottom_right, 0.0);
    }

    #[test]
    fn horizontal_last_zeroes_left_keeps_right() {
        let s = collapse_corners(dummy_style(), Position::Last, Orientation::Horizontal);
        assert_eq!(s.border.radius.top_left, 0.0);
        assert_eq!(s.border.radius.bottom_left, 0.0);
        assert_eq!(s.border.radius.top_right, 8.0);
        assert_eq!(s.border.radius.bottom_right, 8.0);
    }

    #[test]
    fn horizontal_middle_zeroes_all() {
        let s = collapse_corners(dummy_style(), Position::Middle, Orientation::Horizontal);
        assert_eq!(s.border.radius.top_left, 0.0);
        assert_eq!(s.border.radius.bottom_left, 0.0);
        assert_eq!(s.border.radius.top_right, 0.0);
        assert_eq!(s.border.radius.bottom_right, 0.0);
    }

    #[test]
    fn only_keeps_every_corner() {
        let s = collapse_corners(dummy_style(), Position::Only, Orientation::Horizontal);
        assert_eq!(s.border.radius.top_left, 8.0);
        assert_eq!(s.border.radius.top_right, 8.0);
        assert_eq!(s.border.radius.bottom_left, 8.0);
        assert_eq!(s.border.radius.bottom_right, 8.0);
    }

    #[test]
    fn vertical_first_keeps_top_zeroes_bottom() {
        let s = collapse_corners(dummy_style(), Position::First, Orientation::Vertical);
        assert_eq!(s.border.radius.top_left, 8.0);
        assert_eq!(s.border.radius.top_right, 8.0);
        assert_eq!(s.border.radius.bottom_left, 0.0);
        assert_eq!(s.border.radius.bottom_right, 0.0);
    }

    #[test]
    fn vertical_last_zeroes_top_keeps_bottom() {
        let s = collapse_corners(dummy_style(), Position::Last, Orientation::Vertical);
        assert_eq!(s.border.radius.top_left, 0.0);
        assert_eq!(s.border.radius.top_right, 0.0);
        assert_eq!(s.border.radius.bottom_left, 8.0);
        assert_eq!(s.border.radius.bottom_right, 8.0);
    }
}
