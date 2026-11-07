//! SidebarMenuSkeleton: loading placeholder, h-8, rounded-md, muted bg

use crate::component::Component;
use crate::container::Container;
use iced_core::Length::Fill;
use iced_widget::{Widget, space};
use icedxdy_core::radius;
use icedxdy_core::theme::container::Style;
use icedxdy_core::{Border, Theme};

/// Create a sidebar menu skeleton.
///
/// A loading placeholder with h-8 (32px) height, rounded-md corners,
/// and muted background. Used as a shimmer/placeholder while menu
/// items are loading.
pub fn menu_skeleton<'a, Message: 'a>() -> Container<'a, Message> {
    let spacer: iced_core::Element<'a, Message, Theme, iced_widget::Renderer> =
        space::horizontal().boxed();
    Container::new(Component::from(spacer))
        .width(Fill)
        .height(32)
        .style(|theme: &Theme| Style {
            background: Some(theme.muted.base.into()),
            border: Border {
                radius: radius::MD.into(),
                ..Border::default()
            },
            ..Style::default()
        })
}
