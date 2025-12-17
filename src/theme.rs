//! Theme type with Catalog implementations for iced widgets
//!
//! The Theme wraps a Design and implements the Catalog traits
//! for various iced widgets with shadcn-inspired styling.

pub mod button;
pub mod checkbox;
pub mod container;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod rule;
pub mod scrollable;
pub mod table;
pub mod text;

use icedxdy_core::Design;

/// Theme for icedxdy components
///
/// Wraps a Design and implements Catalog traits for iced widgets.
#[derive(Debug, Clone)]
pub struct Theme {
    design: Design,
}

impl Theme {
    /// Create a new theme from a design
    pub fn new(design: Design) -> Self {
        Self { design }
    }

    /// Create a light theme
    pub fn light() -> Self {
        Self::new(Design::light())
    }

    /// Create a dark theme
    pub fn dark() -> Self {
        Self::new(Design::dark())
    }

    /// Get the underlying design
    pub fn design(&self) -> &Design {
        &self.design
    }

    /// Get the palette
    pub fn palette(&self) -> &icedxdy_core::Palette {
        &self.design.palette
    }

    /// Get the iced theme
    pub fn iced_theme(&self) -> &iced_core::Theme {
        &self.design.theme
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl iced_core::theme::Base for Theme {
    fn default(preference: iced_core::theme::Mode) -> Self {
        match preference {
            iced_core::theme::Mode::Light | iced_core::theme::Mode::None => Theme::light(),
            iced_core::theme::Mode::Dark => Theme::dark(),
        }
    }

    fn mode(&self) -> iced_core::theme::Mode {
        // Determine mode based on background lightness
        let bg = self.design.palette.background;
        if bg.l < 0.5 {
            iced_core::theme::Mode::Dark
        } else {
            iced_core::theme::Mode::Light
        }
    }

    fn name(&self) -> &str {
        &self.design.name
    }

    fn base(&self) -> iced_core::theme::Style {
        iced_core::theme::Style {
            background_color: self.design.palette.background.into(),
            text_color: self.design.palette.foreground.into(),
        }
    }

    fn palette(&self) -> Option<iced_core::theme::Palette> {
        Some(self.design.theme.palette())
    }
}
