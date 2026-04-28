//! Progress component — shadcn/ui faithful port.
//!
//! A pill-shaped progress indicator, 4px tall (`h-1`), with a `bg-muted`
//! track and `bg-primary` fill. Mirrors nova's `Progress` exactly.
//!
//! The widget self-animates: prop value changes ease over 150ms via
//! `cubic-bezier(0.4, 0, 0.2, 1)`, the same easing button uses for hover
//! transitions and the same Tailwind `transition-all` default that
//! shadcn's React `<Progress>` inherits on its `<Indicator>` element.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::progress;
//!
//! progress(value).width(Length::Fill)
//! ```

mod element;
pub mod widget;

pub use element::{progress, Progress};
