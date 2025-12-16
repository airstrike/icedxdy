//! Design configuration - the superset of Theme
//!
//! Design contains the palette, the derived iced Theme, and font families.

use iced_core::Font;

use crate::palette::Palette;

/// Complete design configuration
///
/// This is the superset of Theme - it contains the color palette,
/// the derived iced Theme, and font configuration.
#[derive(Debug, Clone)]
pub struct Design {
    /// Design name
    pub name: String,
    /// Color palette
    pub palette: Palette,
    /// The iced Theme derived from this design's palette
    pub theme: iced_core::Theme,
    /// Primary/sans font for UI text
    pub font_sans: Font,
    /// Monospace font for code
    pub font_mono: Font,
}

impl Design {
    /// Create a new design with the given name and palette
    pub fn new(name: impl Into<String>, palette: Palette) -> Self {
        let name = name.into();
        let iced_palette = iced_core::theme::Palette {
            background: palette.background.into(),
            text: palette.foreground.into(),
            primary: palette.primary.base.into(),
            success: palette.accent.base.into(), // Using accent for success
            danger: palette.destructive.base.into(),
            warning: palette.secondary.base.into(), // Using secondary for warning
        };
        let theme = iced_core::Theme::custom(name.clone(), iced_palette);

        Self {
            name,
            palette,
            theme,
            font_sans: Font::DEFAULT,
            font_mono: Font::MONOSPACE,
        }
    }

    /// Create a light mode design
    pub fn light() -> Self {
        Self::new("Light", Palette::light())
    }

    /// Create a dark mode design
    pub fn dark() -> Self {
        Self::new("Dark", Palette::dark())
    }

    /// Set the sans (UI) font
    pub fn with_font_sans(mut self, font: Font) -> Self {
        self.font_sans = font;
        self
    }

    /// Set the monospace font
    pub fn with_font_mono(mut self, font: Font) -> Self {
        self.font_mono = font;
        self
    }
}

impl Default for Design {
    fn default() -> Self {
        Self::light()
    }
}
