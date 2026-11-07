//! Avatar group — N overlapping avatars with an optional trailing count
//! chip.
//!
//! Mirrors shadcn's `<AvatarGroup>` (`flex -space-x-2
//! *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background`).
//! Composition is a horizontal `Row` with negative spacing — each avatar
//! claims its own circular footprint and the row's `-8px` gap pulls the
//! next avatar's left edge under the previous avatar's right edge by 8
//! logical pixels. Each avatar in the group ships with a 2px outer ring
//! colored to the page background so overlapping avatars read as
//! distinct discs.
//!
//! Z-order matches render order: later children draw on top, so the
//! count chip (when present) sits on top of the rightmost avatar — same
//! as shadcn (the `<AvatarGroupCount>` is the last child of the group
//! and naturally lands at the top of the stacking context).

use iced_core::Length;
use iced_widget::Widget;
use iced_widget::{Renderer, row};
use icedxdy_core::Theme;
use icedxdy_core::theme::avatar::group_count_style;

use crate::component::Component;
use crate::container::Container;

use super::{Avatar, Geometry, Size};

/// Negative inter-avatar spacing — shadcn `-space-x-2` = `margin-left:
/// -0.5rem` = `-8px` on every child after the first.
const OVERLAP_PX: f32 = -8.0;

/// Builder for an [`Avatar`] group.
///
/// Compose with [`group`] then optionally append a count chip via
/// [`Group::count`]. Each avatar in the group renders with a 2px outer
/// ring colored to the page background.
pub struct Group<'a, Message> {
    avatars: Vec<Avatar<'a, Message>>,
    count: Option<Component<'a, Message>>,
}

/// Compose N avatars into a horizontal group with negative-space overlap.
///
/// ```ignore
/// avatar::group([
///     avatar(None).fallback("CN"),
///     avatar(None).fallback("LR"),
///     avatar(None).fallback("ER"),
/// ])
/// ```
///
/// Append a trailing count chip with [`Group::count`].
pub fn group<'a, Message: 'a>(
    avatars: impl IntoIterator<Item = Avatar<'a, Message>>,
) -> Group<'a, Message> {
    Group {
        avatars: avatars.into_iter().collect(),
        count: None,
    }
}

impl<'a, Message: 'a> Group<'a, Message> {
    /// Append a trailing count chip after the avatars.
    ///
    /// Accepts any `Component` — pass a string like `"+3"` for the
    /// numeric form, or `icon::plus()` for the icon form.
    /// Mirrors shadcn's `<AvatarGroupCount>` slot at the tail of the
    /// group.
    pub fn count(mut self, content: impl Into<Component<'a, Message>>) -> Self {
        self.count = Some(content.into());
        self
    }
}

impl<'a, Message: 'a> From<Group<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(group: Group<'a, Message>) -> Self {
        let Group { avatars, count } = group;

        // Geometry for the trailing count chip mirrors the FIRST avatar's
        // diameter — shadcn's `<AvatarGroupCount>` adapts to the group's
        // `data-size` via `group-has-data-[size=…]/avatar-group:size-…`,
        // and we bind that to the first avatar's variant since the group
        // doesn't carry a size of its own.
        let count_geometry: Geometry = avatars
            .first()
            .map(Avatar::geometry)
            .unwrap_or_else(|| Size::default().geometry());

        // Push `ring(true)` onto every avatar so the 2px page-bg outline
        // is present in the group context. Outside the group an avatar's
        // bare surface has no ring (per shadcn — the ring is a sibling-
        // selector only).
        let avatar_elements = avatars.into_iter().map(|a| {
            let with_ring: iced_core::Element<'a, Message, Theme, Renderer> = a.ring(true).into();
            with_ring
        });

        let count_element: Option<iced_core::Element<'a, Message, Theme, Renderer>> =
            count.map(|inner| {
                let mut sized = inner;
                sized.size(crate::Size::Custom(count_geometry.font_size));
                let chip = Container::new(sized)
                    .center(Length::from(count_geometry.diameter))
                    .clip(true)
                    .class(Box::new(|t: &Theme| group_count_style(t).into())
                        as iced_widget::container::StyleFn<'static, Theme>);
                chip.into()
            });

        let mut children: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
            avatar_elements.collect();
        if let Some(c) = count_element {
            children.push(c);
        }

        row(children).spacing(OVERLAP_PX).boxed()
    }
}

impl<'a, Message: 'a> From<Group<'a, Message>> for Component<'a, Message> {
    fn from(group: Group<'a, Message>) -> Self {
        Component::from(iced_core::Element::from(group))
    }
}
