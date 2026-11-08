//! Theme type with Catalog implementations for iced widgets.
//!
//! Theme wraps a `Design` and exposes per-role palettes via `Deref` —
//! `theme.muted.base` reads through to `Design::muted.base`. The
//! `tinted(Tint)` accessor returns the palette for a given role by
//! value (Palette is 64 bytes Copy).

pub mod accordion;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod combo_box;
pub mod container;
pub mod dialog;
pub mod empty;
pub mod field;
pub mod float;
pub mod input_group;
pub mod item;
pub mod kbd;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod media;
pub mod menu;
pub mod native_select;
pub mod pick_list;
pub mod progress_bar;
pub mod qr_code;
pub mod radio;
pub mod rule;
pub mod scrollable;
pub mod sidebar;
pub mod slider;
#[cfg(feature = "svg")]
pub mod svg;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod text;
pub mod text_input;
pub mod textarea;
pub mod toggle;
pub mod tooltip;
pub mod tweak;

use std::ops::Deref;

use crate::Design;
use crate::palette::Palette;
use crate::style::Tint;

/// Theme for icedxdy components.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    design: Design,
}

impl Theme {
    pub fn new(design: Design) -> Self {
        Self { design }
    }

    pub fn light() -> Self {
        Self::new(Design::light())
    }

    pub fn dark() -> Self {
        Self::new(Design::dark())
    }

    pub fn design(&self) -> &Design {
        &self.design
    }

    /// Resolve a tint to its concrete `Palette`. Returned by value —
    /// `Palette` is `Copy`.
    pub fn tinted(&self, tint: Tint) -> Palette {
        match tint {
            Tint::Neutral => self.design.neutral,
            Tint::Primary => self.design.primary,
            Tint::Secondary => self.design.secondary,
            Tint::Muted => self.design.muted,
            Tint::Accent => self.design.accent,
            Tint::Destructive => self.design.destructive,
            Tint::Info => self.design.info,
            Tint::Success => self.design.success,
            Tint::Warning => self.design.warning,
        }
    }

    pub fn iced_theme(&self) -> &iced_core::Theme {
        &self.design.theme
    }

    /// The global text-selection highlight (nova `selection:bg-primary`,
    /// softened to the same 30% wash text inputs use).
    pub fn selection(&self) -> iced_core::Color {
        self.design.primary.base.alpha(0.3).into()
    }
}

impl Deref for Theme {
    type Target = Design;

    fn deref(&self) -> &Design {
        &self.design
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
        let bg = self.design.neutral.base;
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
            background_color: self.design.neutral.base.into(),
            text_color: self.design.neutral.foreground.into(),
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
            .unwrap_or(true)
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}
