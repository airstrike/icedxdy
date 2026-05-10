//! Universal widget status for style resolution.
//!
//! Each iced widget Status type maps onto this enum via per-widget
//! converters added when the bridge for that widget lands. Widgets
//! whose iced catalog doesn't expose a Status synthesize `Active`.

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
