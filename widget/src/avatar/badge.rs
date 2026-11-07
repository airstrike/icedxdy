//! Avatar corner badge — small chip overlaid on the bottom-right of an
//! [`Avatar`](super::Avatar).
//!
//! Mirrors shadcn's `<AvatarBadge>` (`absolute right-0 bottom-0 z-10
//! rounded-full bg-primary ring-2 ring-background`). The chip is the
//! `palette.primary` surface with a 2px page-bg ring; size scales with
//! the parent avatar's variant.

use iced_core::Length;
use iced_core::Length::Fill;
use iced_core::alignment::{Horizontal, Vertical};
use iced_widget::Renderer;
use iced_widget::Widget;
use iced_widget::stack;
use icedxdy_core::Theme;
use icedxdy_core::theme::avatar::badge_style;

use crate::component::Component;
use crate::container::Container;

use super::Avatar;

/// Wrap an [`Avatar`] with a corner badge chip.
///
/// `inner` is the badge's content — typically empty (a presence dot) or a
/// small icon like `icon::plus()`. The chip's diameter and inner-icon
/// size derive from the avatar's geometry; the chip itself ships with a
/// 2px page-bg ring and circular border.
///
/// ```ignore
/// // Presence dot.
/// avatar::badge(avatar(None).fallback("CN"), avatar::dot())
///
/// // Icon inside the badge.
/// avatar::badge(avatar(None).fallback("PP"), icon::plus())
/// ```
pub fn badge<'a, Message: 'a>(
    avatar: Avatar<'a, Message>,
    inner: impl Into<Component<'a, Message>>,
) -> Component<'a, Message> {
    let geometry = avatar.geometry();
    let badge_diameter = geometry.badge_diameter;
    let icon_size = geometry.badge_icon_size;

    // The Stack's first child dictates the layout bounds — pin to the
    // avatar so the corner overlay sits inside the avatar's footprint
    // rather than expanding the surface.
    let avatar_el: iced_core::Element<'a, Message, Theme, Renderer> = avatar.into();

    // Chip body: optional icon centered. On the `Sm` size shadcn hides
    // the icon entirely (`[&>svg]:hidden`) and `badge_icon_size == 0.0`
    // signals that intent — drop the inner content into the void.
    let mut inner_component = inner.into();
    if icon_size > 0.0 {
        inner_component.size(crate::Size::Custom(icon_size));
    }

    let chip_inner: Component<'a, Message> = if icon_size > 0.0 {
        inner_component
    } else {
        // Drop the caller's content but reserve the (shrunk) chip body.
        Component::from("")
    };

    let chip = Container::new(chip_inner)
        .center(Length::from(badge_diameter))
        .clip(true)
        .class(Box::new(|t: &Theme| badge_style(t).into())
            as iced_widget::container::StyleFn<'static, Theme>);
    let chip_element: iced_core::Element<'a, Message, Theme, Renderer> = chip.into();

    // Anchor the chip to the bottom-right corner of the avatar by
    // expanding its container to fill the stack and aligning to (right,
    // bottom). The chip itself shrinks to its fixed diameter so the
    // alignment math sits the chip on the corner pixel.
    let overlay = iced_widget::container(chip_element)
        .width(Fill)
        .height(Fill)
        .align_x(Horizontal::Right)
        .align_y(Vertical::Bottom);

    let composed: iced_core::Element<'a, Message, Theme, Renderer> =
        stack![avatar_el, overlay].boxed();
    Component::from(composed)
}

/// A presence dot for use as the inner content of [`badge`] — renders as
/// nothing, leaving just the chip surface (matches shadcn's empty
/// `<AvatarBadge className="bg-green-600" />`).
pub fn dot<'a, Message: 'a>() -> Component<'a, Message> {
    Component::from("")
}
