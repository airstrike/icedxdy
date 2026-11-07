//! Scroll-into-view utility.
//!
//! Uses [`selector::find`] to query widget bounds by [`Id`], then scrolls
//! the target scrollable to bring the child into view.
//!
//! # Example
//!
//! ```no_run
//! use iced_core::widget::Id;
//! use icedxdy_widget::operation::scroll;
//!
//! # #[derive(Debug, Clone)]
//! # enum Message {}
//! # fn update(scrollable_id: &Id, child_id: &Id) -> iced_runtime::Task<Message> {
//! scroll::scroll_into_view(scrollable_id.clone(), child_id.clone())
//! # }
//! ```

use iced_core::widget::Id;
use iced_runtime::Task;
use iced_runtime::widget::operation::Animation;
use iced_runtime::widget::operation::scrollable;
use iced_runtime::widget::selector;

/// Scroll a scrollable to bring a child widget into view.
///
/// Finds the bounds of both the scrollable and the child via
/// [`selector::find`], computes the vertical offset, then issues
/// a [`scroll_to`](scrollable::scroll_to) command.
pub fn scroll_into_view<T: Send + 'static>(
    scrollable: impl Into<Id>,
    child: impl Into<Id>,
) -> Task<T> {
    let scrollable_id = scrollable.into();
    let child_id = child.into();

    selector::find(scrollable_id.clone()).and_then(move |scrollable_target| {
        let scrollable_id = scrollable_id.clone();
        selector::find(child_id.clone()).and_then(move |child_target| {
            let offset_y = child_target.bounds().y - scrollable_target.bounds().y;
            scrollable::scroll_to(
                scrollable_id.clone(),
                scrollable::AbsoluteOffset {
                    x: None,
                    y: Some(offset_y),
                },
                Animation::Instant,
            )
        })
    })
}
