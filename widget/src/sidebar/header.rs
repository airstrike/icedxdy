//! SidebarHeader: flex col, gap-2 (8px), p-2 (8px)

use crate::column::Column;
use crate::compose::Slot;

/// Create a sidebar header.
///
/// A vertical column with 8px spacing and 8px padding, used at the top
/// of a sidebar for branding, search, or navigation controls.
pub fn header<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children).spacing(8).padding(8)
}
