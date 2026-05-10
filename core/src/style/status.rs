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

/// Map iced's button status to ours.
pub fn from_button(s: iced_widget::button::Status) -> Status {
    use iced_widget::button::Status;
    match s {
        Status::Active => super::Status::Active,
        Status::Hovered => super::Status::Hovered,
        Status::Pressed => super::Status::Pressed,
        Status::Disabled => super::Status::Disabled,
    }
}

/// Map iced's checkbox status to ours. Drops `is_checked` — that's widget
/// state, not visual state.
pub fn from_checkbox(s: iced_widget::checkbox::Status) -> Status {
    use iced_widget::checkbox::Status;
    match s {
        Status::Active { .. } => super::Status::Active,
        Status::Hovered { .. } => super::Status::Hovered,
        Status::Disabled { .. } => super::Status::Disabled,
    }
}

/// Map iced's text-input status to ours. `Focused { is_hovered }` collapses
/// to `Focused` — hover-while-focused isn't a distinct style in shadcn nova.
pub fn from_text_input(s: iced_widget::text_input::Status) -> Status {
    use iced_widget::text_input::Status;
    match s {
        Status::Active => super::Status::Active,
        Status::Hovered => super::Status::Hovered,
        Status::Focused { .. } => super::Status::Focused,
        Status::Disabled => super::Status::Disabled,
    }
}

/// Map iced's toggler status to ours. Drops `is_toggled` — widget state.
pub fn from_toggler(s: iced_widget::toggler::Status) -> Status {
    use iced_widget::toggler::Status;
    match s {
        Status::Active { .. } => super::Status::Active,
        Status::Hovered { .. } => super::Status::Hovered,
        Status::Disabled { .. } => super::Status::Disabled,
    }
}

/// Map iced's scrollable status to ours. `Dragged` becomes `Pressed` —
/// they're both "user is actively manipulating this".
pub fn from_scrollable(s: iced_widget::scrollable::Status) -> Status {
    use iced_widget::scrollable::Status;
    match s {
        Status::Active { .. } => super::Status::Active,
        Status::Hovered { .. } => super::Status::Hovered,
        Status::Dragged { .. } => super::Status::Pressed,
    }
}

/// Map our accordion's status to the universal one.
pub fn from_accordion(s: accordion::Status) -> Status {
    use accordion::Status;
    match s {
        Status::Active { .. } => super::Status::Active,
        Status::Hovered { .. } => super::Status::Hovered,
        Status::Focused { .. } => super::Status::Focused,
        Status::Disabled { .. } => super::Status::Disabled,
    }
}
