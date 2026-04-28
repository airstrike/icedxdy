//! Text input — sweetened iced text_input vendored here so we can render
//! focus rings and emit `on_focus` / `on_blur` events.

pub mod cursor;
mod editor;
mod value;
pub mod widget;

pub use cursor::Cursor;
pub use value::Value;
pub use widget::TextInput;
