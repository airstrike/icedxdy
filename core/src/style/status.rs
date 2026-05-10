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

pub fn from_button(s: iced_widget::button::Status) -> Status {
    use iced_widget::button::Status;
    match s {
        Status::Active => super::Status::Active,
        Status::Hovered => super::Status::Hovered,
        Status::Pressed => super::Status::Pressed,
        Status::Disabled => super::Status::Disabled,
    }
}

pub fn from_checkbox(s: iced_widget::checkbox::Status) -> Status {
    use iced_widget::checkbox::Status;
    match s {
        Status::Active { .. } => super::Status::Active,
        Status::Hovered { .. } => super::Status::Hovered,
        Status::Disabled { .. } => super::Status::Disabled,
    }
}

pub fn from_text_input(s: iced_widget::text_input::Status) -> Status {
    use iced_widget::text_input::Status;
    match s {
        Status::Active => super::Status::Active,
        Status::Hovered => super::Status::Hovered,
        Status::Focused { .. } => super::Status::Focused,
        Status::Disabled => super::Status::Disabled,
    }
}

pub fn from_toggler(s: iced_widget::toggler::Status) -> Status {
    use iced_widget::toggler::Status;
    match s {
        Status::Active { .. } => super::Status::Active,
        Status::Hovered { .. } => super::Status::Hovered,
        Status::Disabled { .. } => super::Status::Disabled,
    }
}

pub fn from_scrollable(s: iced_widget::scrollable::Status) -> Status {
    use iced_widget::scrollable::Status;
    match s {
        Status::Active { .. } => super::Status::Active,
        Status::Hovered { .. } => super::Status::Hovered,
        Status::Dragged { .. } => super::Status::Pressed,
    }
}

pub fn from_accordion(s: accordion::Status) -> Status {
    use accordion::Status;
    match s {
        Status::Active { .. } => super::Status::Active,
        Status::Hovered { .. } => super::Status::Hovered,
        Status::Focused { .. } => super::Status::Focused,
        Status::Disabled { .. } => super::Status::Disabled,
    }
}
