//! Theme type with Catalog implementations for iced widgets
//!
//! The Theme wraps a Design and implements the Catalog traits
//! for various iced widgets with shadcn-inspired styling.

pub mod alert;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod combo_box;
pub mod container;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod menu;
pub mod rule;
pub mod scrollable;
pub mod sidebar;
pub mod table;
pub mod text;
pub mod text_input;

use crate::Design;

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
    pub fn palette(&self) -> &crate::Palette {
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

    fn seed(&self) -> Option<iced_core::theme::palette::Seed> {
        Some(self.design.seed)
    }
}

pub fn is_dark_mode() -> bool {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "Dark")
            .unwrap_or(true) // default to dark
    }
    #[cfg(not(target_os = "macos"))]
    {
        true // default to dark on other platforms
    }
}
