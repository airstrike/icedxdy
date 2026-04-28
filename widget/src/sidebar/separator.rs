//! SidebarSeparator: horizontal rule with mx-2 (8px horizontal margin)

use crate::container::Container;
use crate::element::Element;
use iced_core::Length::Fill;
use iced_core::Padding;
use iced_widget::rule;
use icedxdy_core::Theme;

/// Create a sidebar separator.
///
/// A horizontal rule with 8px horizontal margin (mx-2), using the
/// theme's border color.
pub fn separator<'a, Message: 'a>() -> Container<'a, Message> {
    let rule: iced_core::Element<'a, Message, Theme, iced_widget::Renderer> =
        rule::horizontal(1).into();
    let element = Element::from(rule);
    Container::new(element)
        .width(Fill)
        .padding(Padding {
            top: 0.0,
            bottom: 0.0,
            left: 8.0,
            right: 8.0,
        })
}
