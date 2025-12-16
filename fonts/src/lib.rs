//! Font definitions and loading for icedxdy
//!
//! Provides a font management system with lazy initialization.
//! Currently supports Inter as the default UI font.

use std::borrow::Cow;
use std::sync::OnceLock;

use iced_core::font::{Stretch, Style, Weight};
use iced_core::Font as IcedFont;

/// Inter - the default UI font
pub static INTER: Font = Font::new(Weight::Normal);
/// Inter Medium
pub static INTER_MEDIUM: Font = Font::new(Weight::Medium);
/// Inter Semibold
pub static INTER_SEMIBOLD: Font = Font::new(Weight::Semibold);
/// Inter Bold
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
            ..IcedFont::with_name(name)
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

/// Initialize all fonts (call once at startup before using fonts)
pub fn set() {
    INTER.set("Inter");
    INTER_MEDIUM.set("Inter");
    INTER_SEMIBOLD.set("Inter");
    INTER_BOLD.set("Inter");
}

/// Load all font data for registration with iced
///
/// Returns font bytes that should be passed to iced's font loading.
pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!("../assets/Inter-Regular.ttf").as_slice().into(),
        include_bytes!("../assets/Inter-Medium.ttf").as_slice().into(),
        include_bytes!("../assets/Inter-SemiBold.ttf").as_slice().into(),
        include_bytes!("../assets/Inter-Bold.ttf").as_slice().into(),
    ]
}
