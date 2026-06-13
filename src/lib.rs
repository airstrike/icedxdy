//! icedxdy - A shadcn-inspired component library for Iced
//!
//! This library provides a design system and themed components for Iced applications,
//! inspired by the shadcn/ui design system.

pub mod card;
pub mod separator;

// Re-export `card::card` as `icedxdy::card` so `use icedxdy::card;` brings
// both the module (for `card::title(...)`, `card::header(...)`, etc.) and
// the convenience constructor (for `card(my_content)`) into scope. Same
// pattern `widget/src/lib.rs` uses for `field` / `checkbox` / `text` / etc.
pub use card::card;

pub use icedxdy_core as core;
pub use icedxdy_core::theme;
pub use icedxdy_core::{Color, Design, FontFamily, Gap, Palette, Space, Theme};
pub use icedxdy_core::{font_size, height, icon_size, radius, spacing};

// Layout-axis primitive shared across widgets (Field, ButtonGroup, ...).
// Re-exported at the crate root with the bare variants so callers reach
// for `use icedxdy::{Vertical, Horizontal};` and pass them directly into
// any `.orientation()` builder, no FQN dance per-widget.
pub use icedxdy_core::Orientation;
pub use icedxdy_core::Orientation::{Horizontal, Vertical};

#[allow(hidden_glob_reexports)]
pub mod widget {
    pub use icedxdy_widget::*;
}

// Re-export the `rich_text!` macro at our crate root so consumers don't
// need a direct `icedxdy_widget` dep. Type-level `RichText` / `Span` /
// `span` / `rich_text` (the constructor fn) are available via the
// `widget` glob above. `#[doc(inline)]` keeps the macro visible in
// rustdoc; `#[macro_export]` from `icedxdy_widget` makes it available
// at `icedxdy_widget::rich_text` and we surface it here.
#[doc(inline)]
pub use icedxdy_widget::rich_text;

pub use widget::Component;
pub use widget::Size;
pub use widget::Weight;

/// Raw iced widgets with modifications (centering, focus/blur, etc.)
pub mod raw {
    pub use icedxdy_widget::button::widget::Button;
}
