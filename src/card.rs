//! Card components
//!
//! Container component with optional header, content, and footer sections.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::card;
//!
//! card::card(
//!     column![
//!         card::header("Title", "Description"),
//!         card::content(my_form),
//!         card::footer(row![cancel_btn, submit_btn]),
//!     ]
//! )
//! ```

use iced_core::text::IntoFragment;
use iced_widget::container::Container;
use iced_widget::Column;
use iced_widget::Row;

use crate::widget::text;
use crate::theme;
use crate::Theme;

/// Padding values based on shadcn card (24px = 1.5rem)
mod padding {
    /// Standard card section padding (24px)
    pub const SECTION: f32 = 24.0;
}

/// Gap between title and description in header
const HEADER_GAP: f32 = 6.0;

/// Card container - wraps content with card styling
///
/// Applies background, border, and rounded corners.
pub fn card<'a, Message>(
    content: impl Into<iced_core::Element<'a, Message, Theme, iced_widget::Renderer>>,
) -> Container<'a, Message, Theme, iced_widget::Renderer> {
    Container::new(content).style(|t| theme::container::card(t).into())
}

/// Card header with title and description
///
/// Creates a column with title (h3) and muted description text.
/// Padding: 24px all sides, 6px gap between title and description.
pub fn header<'a, Message: 'a>(
    title: impl IntoFragment<'a>,
    description: impl IntoFragment<'a>,
) -> Column<'a, Message, Theme, iced_widget::Renderer> {
    Column::new()
        .push(text::h4(title))
        .push(text::muted(description))
        .spacing(HEADER_GAP)
        .padding(padding::SECTION)
}

/// Card header with title only (no description)
///
/// Padding: 24px all sides.
pub fn header_simple<'a, Message: 'a>(
    title: impl IntoFragment<'a>,
) -> Column<'a, Message, Theme, iced_widget::Renderer> {
    Column::new()
        .push(text::h4(title))
        .padding(padding::SECTION)
}

/// Card content section
///
/// Container with horizontal padding but no top padding (flows from header).
/// Padding: 24px left/right, 0 top, 24px bottom.
pub fn content<'a, Message: 'a>(
    child: impl Into<iced_core::Element<'a, Message, Theme, iced_widget::Renderer>>,
) -> Container<'a, Message, Theme, iced_widget::Renderer> {
    Container::new(child).padding(
        iced_core::Padding::ZERO
            .bottom(padding::SECTION)
            .left(padding::SECTION)
            .right(padding::SECTION),
    )
}

/// Card footer section
///
/// Row container for actions, typically buttons.
/// Padding: 24px left/right, 0 top, 24px bottom.
pub fn footer<'a, Message: 'a>(
    child: impl Into<iced_core::Element<'a, Message, Theme, iced_widget::Renderer>>,
) -> Container<'a, Message, Theme, iced_widget::Renderer> {
    Container::new(child).padding(
        iced_core::Padding::ZERO
            .bottom(padding::SECTION)
            .left(padding::SECTION)
            .right(padding::SECTION),
    )
}

/// Card footer as a row with spacing
///
/// Convenience for footer with multiple children in a row.
pub fn footer_row<'a, Message: 'a>(
    children: impl IntoIterator<Item = iced_core::Element<'a, Message, Theme, iced_widget::Renderer>>,
) -> Container<'a, Message, Theme, iced_widget::Renderer> {
    let row = Row::with_children(children).spacing(8);
    footer(row)
}
