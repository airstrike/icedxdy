//! Semantic color palette
//!
//! Based on shadcn/ui's color token system with light and dark mode support.

use crate::color::{Color, oklch, oklch_alpha};

/// A semantic color with base and foreground variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticColor {
    /// The background/base color
    pub base: Color,
    /// The foreground/text color for content on this background
    pub foreground: Color,
}

impl SemanticColor {
    /// Create a new semantic color pair
    pub const fn new(base: Color, foreground: Color) -> Self {
        Self { base, foreground }
    }
}

/// Sidebar-specific color palette
///
/// Based on shadcn/ui's sidebar CSS variables.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarPalette {
    pub background: Color,
    pub foreground: Color,
    pub primary: SemanticColor,
    pub accent: SemanticColor,
    pub border: Color,
    pub ring: Color,
}

/// Complete color palette for a theme
///
/// Based on shadcn/ui's CSS variable naming convention.
#[derive(Debug, Clone, PartialEq)]
pub struct Palette {
    /// Main background color
    pub background: Color,
    /// Main foreground/text color
    pub foreground: Color,

    /// Card background and text
    pub card: SemanticColor,
    /// Popover background and text
    pub popover: SemanticColor,

    /// Primary action color (buttons, links)
    pub primary: SemanticColor,
    /// Secondary action color
    pub secondary: SemanticColor,
    /// Muted/subtle elements
    pub muted: SemanticColor,
    /// Accent/highlight color
    pub accent: SemanticColor,
    /// Destructive/error actions
    pub destructive: SemanticColor,
    /// Informational actions / neutral status
    pub info: SemanticColor,
    /// Successful actions / positive status
    pub success: SemanticColor,
    /// Cautionary actions / warning status
    pub warning: SemanticColor,

    /// Border color
    pub border: Color,
    /// Input border color
    pub input: Color,
    /// Focus ring color
    pub ring: Color,

    /// Code-block bg + fg (matches shadcn's --color-code / --color-code-foreground;
    /// distinct from `muted` because shadcn's `--surface` is darker in dark mode
    /// than `--muted`).
    pub code: SemanticColor,

    /// Sidebar colors
    pub sidebar: SidebarPalette,
}

impl Palette {
    /// Create the default light mode palette (Neutral theme)
    pub fn light() -> Self {
        Self {
            background: oklch(1.0, 0.0, 0.0),
            foreground: oklch(0.145, 0.0, 0.0),

            card: SemanticColor::new(oklch(1.0, 0.0, 0.0), oklch(0.145, 0.0, 0.0)),
            popover: SemanticColor::new(oklch(1.0, 0.0, 0.0), oklch(0.145, 0.0, 0.0)),

            primary: SemanticColor::new(oklch(0.205, 0.0, 0.0), oklch(0.985, 0.0, 0.0)),
            secondary: SemanticColor::new(oklch(0.97, 0.0, 0.0), oklch(0.205, 0.0, 0.0)),
            muted: SemanticColor::new(oklch(0.97, 0.0, 0.0), oklch(0.556, 0.0, 0.0)),
            accent: SemanticColor::new(oklch(0.97, 0.0, 0.0), oklch(0.205, 0.0, 0.0)),
            destructive: SemanticColor::new(oklch(0.58, 0.22, 27.0), oklch(0.985, 0.0, 0.0)),
            // Status colors — same lightness/chroma profile as `destructive` so
            // they read as a coherent set against the Neutral background.
            info: SemanticColor::new(oklch(0.55, 0.20, 245.0), oklch(0.985, 0.0, 0.0)),
            success: SemanticColor::new(oklch(0.58, 0.18, 150.0), oklch(0.985, 0.0, 0.0)),
            warning: SemanticColor::new(oklch(0.72, 0.16, 75.0), oklch(0.205, 0.0, 0.0)),

            border: oklch(0.922, 0.0, 0.0),
            input: oklch(0.922, 0.0, 0.0),
            ring: oklch(0.708, 0.0, 0.0),

            // shadcn --color-code (light) = oklch(0.98 0 0), fg = --foreground
            code: SemanticColor::new(oklch(0.98, 0.0, 0.0), oklch(0.145, 0.0, 0.0)),

            sidebar: SidebarPalette {
                background: oklch(0.985, 0.0, 0.0),
                foreground: oklch(0.337, 0.013, 286.0),
                primary: SemanticColor::new(oklch(0.205, 0.0, 0.0), oklch(0.985, 0.0, 0.0)),
                accent: SemanticColor::new(oklch(0.967, 0.001, 286.0), oklch(0.205, 0.015, 286.0)),
                border: oklch(0.930, 0.006, 250.0),
                ring: oklch(0.623, 0.170, 255.0),
            },
        }
    }

    /// Create the default dark mode palette (Neutral theme)
    pub fn dark() -> Self {
        Self {
            background: oklch(0.145, 0.0, 0.0),
            foreground: oklch(0.985, 0.0, 0.0),

            card: SemanticColor::new(oklch(0.145, 0.0, 0.0), oklch(0.985, 0.0, 0.0)),
            popover: SemanticColor::new(oklch(0.145, 0.0, 0.0), oklch(0.985, 0.0, 0.0)),

            primary: SemanticColor::new(oklch(0.922, 0.0, 0.0), oklch(0.205, 0.0, 0.0)),
            secondary: SemanticColor::new(oklch(0.269, 0.0, 0.0), oklch(0.985, 0.0, 0.0)),
            muted: SemanticColor::new(oklch(0.269, 0.0, 0.0), oklch(0.708, 0.0, 0.0)),
            accent: SemanticColor::new(oklch(0.371, 0.0, 0.0), oklch(0.985, 0.0, 0.0)),
            destructive: SemanticColor::new(oklch(0.704, 0.191, 22.216), oklch(0.985, 0.0, 0.0)),
            // Status colors — pastel-bright variants matching `destructive`'s
            // dark-mode lift so the four state colors form a coherent set.
            info: SemanticColor::new(oklch(0.70, 0.16, 245.0), oklch(0.205, 0.0, 0.0)),
            success: SemanticColor::new(oklch(0.72, 0.16, 150.0), oklch(0.205, 0.0, 0.0)),
            warning: SemanticColor::new(oklch(0.80, 0.15, 80.0), oklch(0.205, 0.0, 0.0)),

            border: oklch_alpha(1.0, 0.0, 0.0, 0.1),
            input: oklch_alpha(1.0, 0.0, 0.0, 0.15),
            ring: oklch(0.556, 0.0, 0.0),

            // shadcn --color-code (dark) = oklch(0.2 0 0), fg = oklch(0.708 0 0)
            code: SemanticColor::new(oklch(0.2, 0.0, 0.0), oklch(0.708, 0.0, 0.0)),

            sidebar: SidebarPalette {
                background: oklch(0.205, 0.015, 286.0),
                foreground: oklch(0.967, 0.001, 286.0),
                primary: SemanticColor::new(oklch(0.922, 0.0, 0.0), oklch(0.205, 0.0, 0.0)),
                accent: SemanticColor::new(oklch(0.269, 0.009, 286.0), oklch(0.967, 0.001, 286.0)),
                border: oklch(0.269, 0.009, 286.0),
                ring: oklch(0.623, 0.170, 255.0),
            },
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::light()
    }
}
