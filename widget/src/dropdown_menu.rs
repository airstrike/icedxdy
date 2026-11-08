//! DropdownMenu: shadcn/ui faithful port (Radix `DropdownMenu`).
//!
//! A [`Popover`] holding a menu: its panel is as wide as its trigger (at
//! least 128px) with `p-1`, and it is modal, so the page ignores the
//! pointer while it is open and a press anywhere outside closes it. Build
//! the content from [`menu`](crate::menu) parts; selecting an item closes
//! the menu.
//!
//! ```ignore
//! use icedxdy::widget::{button, column, dropdown_menu, menu};
//!
//! dropdown_menu(
//!     |m| m.trigger(button("Open").style(button::Outline)),
//!     |_| {
//!         column![
//!             menu::label("My Account"),
//!             menu::item("Profile").shortcut("⇧⌘P"),
//!             menu::separator(),
//!             menu::item("Log out").on_select(Message::LogOut),
//!         ]
//!         .into()
//!     },
//! )
//! .align(dropdown_menu::Align::Start)
//! .width(160)
//! ```

use iced_core::{Element, Pixels};
use iced_widget::Renderer;
use icedxdy_core::Theme;

use crate::compose::{Compose, Inherited, Slot};
use crate::popover::{self, Popover};

pub use crate::popover::{Align, Handle, Side};

/// A menu opened from its trigger.
pub struct DropdownMenu<'a, Message> {
    popover: Popover<'a, Message>,
}

/// Create a closed, uncontrolled [`DropdownMenu`]: `trigger` builds what
/// sits in place, `content` the menu shown while open.
pub fn dropdown_menu<'a, Message>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> DropdownMenu<'a, Message> {
    DropdownMenu {
        // `w-(--radix-dropdown-menu-trigger-width) min-w-32 p-1`.
        popover: popover::popover(trigger, content)
            .trigger_width(128.0)
            .padding(4)
            .modal(true),
    }
}

impl<'a, Message> DropdownMenu<'a, Message> {
    /// Control the menu: it is open exactly when `open` is.
    pub fn open(mut self, open: bool) -> Self {
        self.popover = self.popover.open(open);
        self
    }

    /// Whether an uncontrolled menu starts open.
    pub fn default_open(mut self, open: bool) -> Self {
        self.popover = self.popover.default_open(open);
        self
    }

    /// Set the message published with the new open state.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.popover = self.popover.on_open_change(on_change);
        self
    }

    /// Set the side of the trigger the menu prefers (`Bottom` by default).
    pub fn side(mut self, side: Side) -> Self {
        self.popover = self.popover.side(side);
        self
    }

    /// Set how the menu lines up with its trigger (`Center` by default).
    pub fn align(mut self, align: Align) -> Self {
        self.popover = self.popover.align(align);
        self
    }

    /// Set the gap between the trigger and the menu (4px by default).
    pub fn side_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.popover = self.popover.side_offset(offset);
        self
    }

    /// Set the menu's width instead of the trigger's (`className="w-56"`).
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.popover = self.popover.width(width);
        self
    }
}

impl<'a, Message: 'static> Compose<Message> for DropdownMenu<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        Compose::inherit(&mut self.popover, from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        Box::new(self.popover).compose()
    }
}

impl<'a, Message: 'static> From<DropdownMenu<'a, Message>> for Slot<'a, Message> {
    fn from(menu: DropdownMenu<'a, Message>) -> Self {
        Slot::new(menu)
    }
}

impl<'a, Message: 'static> From<DropdownMenu<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(menu: DropdownMenu<'a, Message>) -> Self {
        menu.popover.into()
    }
}
