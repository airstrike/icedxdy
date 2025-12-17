//! Separator components
//!
//! Visual dividers for separating content.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::separator;
//!
//! column![
//!     content_a,
//!     separator::horizontal(),
//!     content_b,
//! ]
//!
//! row![
//!     left_panel,
//!     separator::vertical(),
//!     right_panel,
//! ]
//! ```

use iced_widget::rule::{self, Rule};

use crate::Theme;

/// Horizontal separator - divides content vertically
///
/// 1px thick line spanning full width
pub fn horizontal<'a>() -> Rule<'a, Theme> {
    rule::horizontal(1)
}

/// Vertical separator - divides content horizontally
///
/// 1px thick line spanning full height
pub fn vertical<'a>() -> Rule<'a, Theme> {
    rule::vertical(1)
}
