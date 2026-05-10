//! Universal widget status for style resolution.
//!
//! Each iced widget Status type maps onto this enum via per-widget
//! converters added when the bridge for that widget lands. Widgets
//! whose iced catalog doesn't expose a Status synthesize `Active`.

use crate::theme::accordion;

/// Universal widget state passed to `StyleFn` inner closures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Hovered,
    Pressed,
    Focused,
    Disabled,
    Invalid,
}

// Variant constructors as named fns so the per-converter match arms can
// shadow the outer `Status` (via a local `use`) without aliased imports.

fn active() -> Status {
    Status::Active
}
fn hovered() -> Status {
    Status::Hovered
}
fn pressed() -> Status {
    Status::Pressed
}
fn focused() -> Status {
    Status::Focused
}
fn disabled() -> Status {
    Status::Disabled
}

pub fn from_button(s: iced_widget::button::Status) -> Status {
    use iced_widget::button::Status;
    match s {
        Status::Active => active(),
        Status::Hovered => hovered(),
        Status::Pressed => pressed(),
        Status::Disabled => disabled(),
    }
}

pub fn from_checkbox(s: iced_widget::checkbox::Status) -> Status {
    use iced_widget::checkbox::Status;
    match s {
        Status::Active { .. } => active(),
        Status::Hovered { .. } => hovered(),
        Status::Disabled { .. } => disabled(),
    }
}

pub fn from_text_input(s: iced_widget::text_input::Status) -> Status {
    use iced_widget::text_input::Status;
    match s {
        Status::Active => active(),
        Status::Hovered => hovered(),
        Status::Focused { .. } => focused(),
        Status::Disabled => disabled(),
    }
}

pub fn from_toggler(s: iced_widget::toggler::Status) -> Status {
    use iced_widget::toggler::Status;
    match s {
        Status::Active { .. } => active(),
        Status::Hovered { .. } => hovered(),
        Status::Disabled { .. } => disabled(),
    }
}

pub fn from_scrollable(s: iced_widget::scrollable::Status) -> Status {
    use iced_widget::scrollable::Status;
    match s {
        Status::Active { .. } => active(),
        Status::Hovered { .. } => hovered(),
        Status::Dragged { .. } => pressed(),
    }
}

pub fn from_accordion(s: accordion::Status) -> Status {
    use accordion::Status;
    match s {
        Status::Active { .. } => active(),
        Status::Hovered { .. } => hovered(),
        Status::Focused { .. } => focused(),
        Status::Disabled { .. } => disabled(),
    }
}
