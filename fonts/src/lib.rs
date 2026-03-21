//! Font definitions and loading for icedxdy
//!
//! Provides a font management system with lazy initialization.
//! Currently supports Inter Tight as the default UI font.
//!
//! # Usage
//!
//! ## Option 1: Settings-based (synchronous, at startup)
//!
//! ```ignore
//! use icedxdy::fonts;
//!
//! fn main() -> iced::Result {
//!     let settings = iced::Settings {
//!         default_font: fonts::INTER.get(),
//!         fonts: fonts::load_bytes(),
//!         ..Default::default()
//!     };
//!
//!     iced::application(App::new, App::update, App::view)
//!         .settings(settings)
//!         .run()
//! }
//! ```
//!
//! ## Option 2: Task-based (async, in App::new)
//!
//! ```ignore
//! use icedxdy::fonts;
//!
//! impl App {
//!     fn new() -> (Self, Task<Message>) {
//!         (App::default(), fonts::load().map(|_| Message::FontsLoaded))
//!     }
//! }
//! ```

use std::borrow::Cow;
use std::sync::OnceLock;

use iced_core::font::{Error, Stretch, Style, Weight};
use iced_core::Font as IcedFont;
use iced_runtime::font;
use iced_runtime::Task;

/// Inter Tight - the default UI font
pub static INTER: Font = Font::new(Weight::Normal);
/// Inter Tight Medium
pub static INTER_MEDIUM: Font = Font::new(Weight::Medium);
/// Inter Tight Semibold
pub static INTER_SEMIBOLD: Font = Font::new(Weight::Semibold);
/// Inter Tight Bold
pub static INTER_BOLD: Font = Font::new(Weight::Bold);

/// A font definition with lazy initialization
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Font {
    weight: Weight,
    style: Style,
    stretch: Stretch,
    inner: OnceLock<IcedFont>,
}

impl Font {
    /// Create a new font with the given weight
    const fn new(weight: Weight) -> Self {
        Self {
            weight,
            stretch: Stretch::Normal,
            style: Style::Normal,
            inner: OnceLock::new(),
        }
    }

    /// Set the font family name (call once at startup)
    fn set(&self, name: &str) {
        let name = Box::leak(name.to_string().into_boxed_str());

        let _ = self.inner.set(IcedFont {
            weight: self.weight,
            stretch: self.stretch,
            style: self.style,
            ..IcedFont::new(name)
        });
    }

    /// Get the underlying iced Font
    pub fn get(&self) -> IcedFont {
        *self.inner.get().expect("fonts must be initialized with set() on startup")
    }
}

impl From<Font> for IcedFont {
    fn from(value: Font) -> Self {
        value.get()
    }
}

impl From<&Font> for IcedFont {
    fn from(value: &Font) -> Self {
        value.get()
    }
}

/// Initialize font name mappings (call once at startup)
///
/// This sets up the font name -> iced::Font mapping. Call this before
/// using any font statics like `INTER.get()`.
///
/// Note: All Inter Tight variants share the same family name "Inter Tight"
pub fn set() {
    INTER.set("Inter Tight");
    INTER_MEDIUM.set("Inter Tight");
    INTER_SEMIBOLD.set("Inter Tight");
    INTER_BOLD.set("Inter Tight");
}

/// Load all vendored fonts as a Task
///
/// Returns a Task that loads all Inter Tight font variants. Use this in `App::new()`:
///
/// ```ignore
/// fn new() -> (Self, Task<Message>) {
///     (App::default(), fonts::load().map(|_| Message::FontsLoaded))
/// }
/// ```
pub fn load() -> Task<Result<(), Error>> {
    set(); // Initialize font name mappings

    Task::batch([
        font::load(include_bytes!("../assets/InterTight-Regular.ttf").as_slice()),
        font::load(include_bytes!("../assets/InterTight-Medium.ttf").as_slice()),
        font::load(include_bytes!("../assets/InterTight-SemiBold.ttf").as_slice()),
        font::load(include_bytes!("../assets/InterTight-Bold.ttf").as_slice()),
    ])
}

/// Get font bytes for Settings-based loading
///
/// Use this with `iced::Settings`:
///
/// ```ignore
/// let settings = iced::Settings {
///     fonts: fonts::load_bytes(),
///     ..Default::default()
/// };
/// ```
pub fn load_bytes() -> Vec<Cow<'static, [u8]>> {
    set(); // Initialize font name mappings

    vec![
        include_bytes!("../assets/InterTight-Regular.ttf").as_slice().into(),
        include_bytes!("../assets/InterTight-Medium.ttf").as_slice().into(),
        include_bytes!("../assets/InterTight-SemiBold.ttf").as_slice().into(),
        include_bytes!("../assets/InterTight-Bold.ttf").as_slice().into(),
    ]
}

/// List all vendored font families
pub fn list() -> &'static [&'static str] {
    &["Inter Tight"]
}

/// Look up a font by name (case-insensitive)
///
/// Returns the regular weight variant. Use `.get()` to get the iced::Font.
pub fn lookup(name: &str) -> Option<&'static Font> {
    match name.to_lowercase().as_str() {
        "inter" | "inter tight" => Some(&INTER),
        _ => None,
    }
}

/// Look up a font by name with a specific weight
pub fn lookup_weight(name: &str, weight: Weight) -> Option<IcedFont> {
    match name.to_lowercase().as_str() {
        "inter" | "inter tight" => Some(match weight {
            Weight::Normal => INTER.get(),
            Weight::Medium => INTER_MEDIUM.get(),
            Weight::Semibold => INTER_SEMIBOLD.get(),
            Weight::Bold => INTER_BOLD.get(),
            _ => INTER.get(), // fallback to regular for other weights
        }),
        _ => None,
    }
}
