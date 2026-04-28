//! SidebarMenuSkeleton: loading placeholder, h-8, rounded-md, muted bg

use crate::container::Container;
use crate::element::Element;
use iced_core::Length::Fill;
use icedxdy_core::radius;
use icedxdy_core::Theme;

/// Create a sidebar menu skeleton.
///
/// A loading placeholder with h-8 (32px) height, rounded-md corners,
/// and muted background. Used as a shimmer/placeholder while menu
/// items are loading.
pub fn menu_skeleton<'a, Message: 'a>() -> Container<'a, Message> {
    let spacer: iced_core::Element<'a, Message, Theme, iced_widget::Renderer> =
        iced_widget::space::horizontal().into();
    let element = Element::from(spacer);
    Container::new(element)
        .width(Fill)
        .height(32)
        .style(move |theme: &icedxdy_core::Theme| {
            iced_widget::container::Style {
                background: Some(theme.palette().muted.base.into()),
                border: iced_core::Border {
                    radius: radius::MD.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
}
