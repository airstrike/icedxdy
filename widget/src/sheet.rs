//! Sheet — shadcn/ui faithful port (Radix `Dialog`, pinned to an edge).
//!
//! The same component as [`Dialog`]: two closures receive a
//! [`Handle`](crate::dialog::Handle), `s.trigger` opens it and `s.close`
//! dismisses it. The panel spans the window along its [`Side`] and pads its
//! sections itself, through [`header`] and [`footer`]:
//!
//! ```ignore
//! use icedxdy::widget::{button, column, sheet};
//!
//! sheet(
//!     |s| s.trigger(button("Open").style(button::Outline)),
//!     |s| {
//!         column![
//!             sheet::header("Edit profile", "Make changes to your profile here."),
//!             body(),
//!             sheet::footer([
//!                 button("Save changes").into(),
//!                 s.close(button("Close").style(button::Outline)),
//!             ]),
//!         ]
//!         .into()
//!     },
//! )
//! .side(sheet::Side::Left)
//! ```

use iced_core::Element;
use iced_core::Length::Fill;
use iced_core::alignment::Vertical;
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme;

use crate::column::Column;
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::dialog::{Dialog, Handle, Placement};
use crate::{column, text};

pub use crate::dialog::Side;

/// A panel that slides in from a window edge.
pub struct Sheet<'a, Message>(Dialog<'a, Message>);

/// Create a closed, uncontrolled [`Sheet`] on the right: `trigger` builds
/// what sits in place, `content` the panel shown while open.
pub fn sheet<'a, Message>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> Sheet<'a, Message> {
    Sheet(Dialog::new(trigger, content, Placement::Side(Side::Right)))
}

impl<'a, Message> Sheet<'a, Message> {
    /// Set the edge the sheet is pinned to.
    pub fn side(self, side: Side) -> Self {
        Self(self.0.side(side))
    }

    /// Control the sheet: it is open exactly when `open` is.
    pub fn open(self, open: bool) -> Self {
        Self(self.0.open(open))
    }

    /// Whether an uncontrolled sheet starts open.
    pub fn default_open(self, open: bool) -> Self {
        Self(self.0.default_open(open))
    }

    /// Set the message published with the new open state when the sheet is
    /// opened or dismissed.
    pub fn on_open_change(self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        Self(self.0.on_open_change(on_change))
    }

    /// Whether the panel shows its corner close button (on by default).
    pub fn show_close_button(self, show: bool) -> Self {
        Self(self.0.show_close_button(show))
    }
}

/// A `SheetHeader`: a `SheetTitle` over a `SheetDescription`, `p-4 gap-0.5`.
pub fn header<'a, Message: 'a>(title: &'a str, description: &'a str) -> Slot<'a, Message> {
    Container::new(
        column![
            text::Text::new(title)
                .size(text::size::BASE)
                .medium()
                .style(theme::text::Foreground),
            text::muted(description),
        ]
        .spacing(2),
    )
    .width(Fill)
    .padding(16)
    .into()
}

/// A `SheetFooter`: its actions stacked, `p-4 gap-2`, at the bottom of the
/// panel (`mt-auto`).
pub fn footer<'a, Message: 'a>(
    actions: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    Container::new(
        Column::with_children(
            actions
                .into_iter()
                .map(|action| Slot::from(Container::new(action).width(Fill))),
        )
        .spacing(8)
        .width(Fill),
    )
    .width(Fill)
    .height(Fill)
    .padding(16)
    .align_y(Vertical::Bottom)
    .into()
}

impl<'a, Message: Clone + 'static> Compose<Message> for Sheet<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        Compose::inherit(&mut self.0, from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        Box::new(self.0).compose()
    }
}

impl<'a, Message: Clone + 'static> From<Sheet<'a, Message>> for Slot<'a, Message> {
    fn from(sheet: Sheet<'a, Message>) -> Self {
        Slot::new(sheet)
    }
}

impl<'a, Message: Clone + 'static> From<Sheet<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(sheet: Sheet<'a, Message>) -> Self {
        sheet.0.into()
    }
}
