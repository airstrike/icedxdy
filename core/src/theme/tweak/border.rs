//! Border atoms: `border::Color` and `border::Width`.
//!
//! Nested under `border::` rather than promoted to composite names
//! (`BorderColor`, `BorderWidth`) per the no-composite-names rule. Iced
//! groups these fields under `iced::Border { color, width, radius }` so
//! the call-site shape `tweak::border::Color(Border)` reads as the same
//! concept.

pub mod color;
pub mod width;

pub use color::Color;
pub use width::Width;
