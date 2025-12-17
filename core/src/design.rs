//! Design configuration - the superset of Theme
//!
//! Design contains the palette, the derived iced Theme, and font families.

use iced_core::font::Weight;
use iced_core::Font;

use crate::palette::Palette;

/// Font family with weight variants
#[derive(Debug, Clone)]
pub struct FontFamily {
    /// Regular weight
    pub regular: Font,
    /// Medium weight
    pub medium: Font,
    /// Semibold weight
    pub semibold: Font,
    /// Bold weight
    pub bold: Font,
}

impl FontFamily {
    /// Create a font family from a font name with all weight variants
    pub fn from_name(name: &'static str) -> Self {
        Self {
            regular: Font {
                weight: Weight::Normal,
                ..Font::with_name(name)
            },
            medium: Font {
                weight: Weight::Medium,
                ..Font::with_name(name)
            },
            semibold: Font {
                weight: Weight::Semibold,
                ..Font::with_name(name)
            },
            bold: Font {
                weight: Weight::Bold,
                ..Font::with_name(name)
            },
        }
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        // Inter Tight is the default UI font for icedxdy
        Self::from_name("Inter Tight")
    }
}

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
    /// Sans-serif font family for UI text
    pub font_sans: FontFamily,
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
            font_sans: FontFamily::default(),
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

    /// Set the sans (UI) font family
    pub fn with_font_sans(mut self, font: FontFamily) -> Self {
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
