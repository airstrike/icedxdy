//! SidebarMenuAction — ghost-style action button for sidebar menu items
//!
//! A small ghost button intended for inline actions on menu items
//! (e.g. a "..." or "+" button next to a menu item label).
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::sidebar;
//!
//! sidebar::menu_action(icon::plus())
//!     .on_press(Message::AddItem)
//! ```

use iced_widget::Renderer;

use crate::Size;
use crate::button::widget;
use crate::component::{Component, IntoElement};
use icedxdy_core::Theme;
use icedxdy_core::theme::button;

/// A ghost-style action button for sidebar menu items.
///
/// Small (24x24), ghost styling, centered content.
pub struct SidebarMenuAction<'a, Message> {
    content: Component<'a, Message>,
    on_press: Option<Message>,
}

/// Create a sidebar menu action button (ghost-style, 24x24).
pub fn menu_action<'a, Message: Clone + 'a>(
    content: impl Into<Component<'a, Message>>,
) -> SidebarMenuAction<'a, Message> {
    SidebarMenuAction {
        content: content.into(),
        on_press: None,
    }
}

impl<'a, Message: Clone + 'a> SidebarMenuAction<'a, Message> {
    /// Set the message to emit on press.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<'a, Message: Clone + 'a> From<SidebarMenuAction<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(mut action: SidebarMenuAction<'a, Message>) -> Self {
        action.content.size(Size::Xs);
        let content_iced = action.content.into_iced();

        // 24x24, our raw button with translate(false) and ghost style
        let style: widget::StyleFn<'a, Theme> = Box::new(|t, s: widget::Status| {
            let status: iced_widget::button::Status = s.into();
            let iced: iced_widget::button::Style = button::ghost(t, status).into();
            iced.into()
        });
        let mut raw_btn = widget::Button::new(iced_widget::center(content_iced))
            .width(24.0)
            .height(24.0)
            .padding(0)
            .translate(false)
            .style(style);

        if let Some(on_press) = action.on_press {
            raw_btn = raw_btn.on_press(on_press);
        }

        raw_btn.into()
    }
}

impl<'a, Message: Clone + 'a> From<SidebarMenuAction<'a, Message>>
    for crate::Component<'a, Message>
{
    fn from(a: SidebarMenuAction<'a, Message>) -> Self {
        crate::Component::new(a)
    }
}

impl<'a, Message: Clone + 'a> IntoElement<'a, Message> for SidebarMenuAction<'a, Message> {
    fn size(&mut self, _size: Size) {
        // Fixed size, ignore parent hints
    }

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        (*self).into()
    }
}
