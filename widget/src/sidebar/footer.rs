//! SidebarFooter: flex col, gap-2 (8px), p-2 (8px)

use crate::column::Column;
use crate::element::Element;

/// Create a sidebar footer.
///
/// A vertical column with 8px spacing and 8px padding, used at the bottom
/// of a sidebar for user info, settings, or actions.
pub fn footer<'a, Message: 'a>(
    children: impl IntoIterator<Item = Element<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children).spacing(8).padding(8)
}
