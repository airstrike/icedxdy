//! SidebarRail — thin click-target rail to toggle the sidebar.
//!
//! Per shadcn nova `sidebar.tsx::SidebarRail`: a 16px-wide hit target
//! (`w-4`), full-height (`inset-y-0`), with a 2px hover line down the
//! middle (`after:w-[2px] hover:after:bg-sidebar-border`) and a horizontal
//! resize cursor. Click toggles the sidebar.
//!
//! In iced we render the rail as a thin button: `widget::Button` with
//! `width(16)`, `height(Fill)`, no press-down (`translate(false)`), and
//! a horizontal resize cursor. The 2px hover indicator is approximated
//! by setting the button's hover background to `palette.sidebar.border`
//! (the same token shadcn's `::after` pseudo-element targets).
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::sidebar;
//!
//! sidebar::rail(Message::ToggleSidebar)
//! ```

use iced_core::Length::Fill;
use iced_core::mouse::Interaction;
use iced_widget::Renderer;
use iced_widget::button::Status;
use iced_widget::space;

use crate::button::widget;
use crate::compose::Compose;
use icedxdy_core::Theme;
use icedxdy_core::theme::button;

/// Width of the rail hit target (matches shadcn's `w-4` = 16px).
pub const RAIL_WIDTH: f32 = 16.0;

/// A thin click-target rail that toggles the sidebar.
pub struct SidebarRail<Message> {
    on_press: Message,
}

/// Create a sidebar rail.
///
/// Emits `on_press` when clicked. Renders as a 16px-wide button spanning
/// the full sidebar height with a horizontal-resize cursor.
pub fn rail<Message: Clone>(on_press: Message) -> SidebarRail<Message> {
    SidebarRail { on_press }
}

impl<Message: Clone> SidebarRail<Message> {}

// -- From impls ---------------------------------------------------------------

impl<'a, Message: Clone + 'a> From<SidebarRail<Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(rail: SidebarRail<Message>) -> Self {
        // Hover lights up the rail with `palette.sidebar.border` — same
        // token shadcn's `hover:after:bg-sidebar-border` pseudo-element
        // targets. Resting state is fully transparent (ghost) so the rail
        // vanishes until pointed at. The chained `.into()` lands core::Style
        // → iced::Style → widget::Style.
        let style: widget::StyleFn<'a, Theme> = Box::new(|t, s: widget::Status| {
            let status: Status = s.into();
            let mut core = button::ghost(t, status);
            if matches!(status, Status::Hovered | Status::Pressed) {
                core.background = Some(t.sidebar.border.into());
            }
            let iced: iced_widget::button::Style = core.into();
            iced.into()
        });

        let raw_btn = widget::Button::new(space::horizontal())
            .width(RAIL_WIDTH)
            .height(Fill)
            .padding(0)
            .translate(false)
            .interaction(Interaction::ResizingHorizontally)
            .style(style)
            .on_press(rail.on_press);

        raw_btn.into()
    }
}

impl<'a, Message: Clone + 'a> From<SidebarRail<Message>> for crate::Slot<'a, Message> {
    fn from(widget: SidebarRail<Message>) -> Self {
        crate::Slot::new(widget)
    }
}

impl<Message: Clone> Compose<Message> for SidebarRail<Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
