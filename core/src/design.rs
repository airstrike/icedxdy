//! Design configuration — the superset of Theme.
//!
//! Holds the full palette field set (per-role + surfaces + sidebar),
//! the derived iced Theme, fonts, and identity.

use iced_core::Font;
use iced_core::font::Weight;
use iced_core::theme::palette::Seed;

use crate::color::{Color, oklch, oklch_alpha};
use crate::palette::{Palette, SidebarPair, SidebarPalette};

/// Font family with weight variants
#[derive(Debug, Clone)]
pub struct FontFamily {
    pub regular: Font,
    pub medium: Font,
    pub semibold: Font,
    pub bold: Font,
}

impl FontFamily {
    pub fn from_name(name: &'static str) -> Self {
        Self {
            regular: Font::new(name).weight(Weight::Normal),
            medium: Font::new(name).weight(Weight::Medium),
            semibold: Font::new(name).weight(Weight::Semibold),
            bold: Font::new(name).weight(Weight::Bold),
        }
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::from_name("Inter Tight")
    }
}

/// Complete design configuration — per-role palettes, surfaces, sidebar,
/// fonts, and the derived iced Theme.
#[derive(Debug, Clone)]
pub struct Design {
    pub name: String,

    // Intent palettes — selectable via `Tint`.
    pub neutral: Palette,
    pub primary: Palette,
    pub secondary: Palette,
    pub muted: Palette,
    pub accent: Palette,
    pub destructive: Palette,
    pub info: Palette,
    pub success: Palette,
    pub warning: Palette,

    // Surface palettes — not Tint-eligible; widgets read them directly.
    pub card: Palette,
    pub popover: Palette,
    pub code: Palette,

    // Sidebar context — deferred reshape; stays as its own struct.
    pub sidebar: SidebarPalette,

    /// Input-field border color. shadcn distinguishes `--input` from
    /// `--border` in dark mode (15% vs 10% alpha), so input-shaped widgets
    /// (text input, checkbox, switch track, button outline dark fade) read
    /// from this slot instead of `neutral.border`.
    pub input: Color,

    pub seed: Seed,
    pub theme: iced_core::Theme,
    pub font_sans: FontFamily,
    pub font_mono: Font,
}

impl Design {
    /// Light mode design (Neutral theme).
    pub fn light() -> Self {
        // Structural greys — chrome borders + focus ring.
        let neutral_border = oklch(0.922, 0.0, 0.0);
        let neutral_ring = oklch(0.708, 0.0, 0.0);

        let neutral = Palette {
            base: oklch(1.0, 0.0, 0.0),
            foreground: oklch(0.145, 0.0, 0.0),
            border: neutral_border,
            ring: neutral_ring,
            tint_text: oklch(0.145, 0.0, 0.0),
            is_dark: false,
        };

        let primary = tinted_palette(
            oklch(0.205, 0.0, 0.0),
            oklch(0.985, 0.0, 0.0),
            false,
        );
        let secondary = tinted_palette(
            oklch(0.97, 0.0, 0.0),
            oklch(0.205, 0.0, 0.0),
            false,
        );
        let muted = tinted_palette(
            oklch(0.97, 0.0, 0.0),
            oklch(0.556, 0.0, 0.0),
            false,
        );
        let accent = tinted_palette(
            oklch(0.97, 0.0, 0.0),
            oklch(0.205, 0.0, 0.0),
            false,
        );
        let destructive = tinted_palette(
            oklch(0.577, 0.245, 27.325),
            oklch(0.97, 0.01, 17.0),
            false,
        );
        let info = tinted_palette(
            oklch(0.55, 0.20, 245.0),
            oklch(0.985, 0.0, 0.0),
            false,
        );
        let success = tinted_palette(
            oklch(0.58, 0.18, 150.0),
            oklch(0.985, 0.0, 0.0),
            false,
        );
        let warning = tinted_palette(
            oklch(0.72, 0.16, 75.0),
            oklch(0.205, 0.0, 0.0),
            false,
        );

        let card = surface_palette(
            oklch(1.0, 0.0, 0.0),
            oklch(0.145, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            false,
        );
        let popover = surface_palette(
            oklch(1.0, 0.0, 0.0),
            oklch(0.145, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            false,
        );
        let code = surface_palette(
            oklch(0.98, 0.0, 0.0),
            oklch(0.145, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            false,
        );

        let sidebar = SidebarPalette {
            background: oklch(0.985, 0.0, 0.0),
            foreground: oklch(0.145, 0.0, 0.0),
            primary: SidebarPair {
                base: oklch(0.205, 0.0, 0.0),
                foreground: oklch(0.985, 0.0, 0.0),
            },
            accent: SidebarPair {
                base: oklch(0.97, 0.0, 0.0),
                foreground: oklch(0.205, 0.0, 0.0),
            },
            border: oklch(0.922, 0.0, 0.0),
            ring: oklch(0.708, 0.0, 0.0),
        };

        let input = oklch(0.922, 0.0, 0.0);

        build(
            "Light", neutral, primary, secondary, muted, accent, destructive, info, success,
            warning, card, popover, code, sidebar, input,
        )
    }

    /// Dark mode design (Neutral theme).
    pub fn dark() -> Self {
        let neutral_border = oklch_alpha(1.0, 0.0, 0.0, 0.1);
        let neutral_ring = oklch(0.556, 0.0, 0.0);

        let neutral = Palette {
            base: oklch(0.145, 0.0, 0.0),
            foreground: oklch(0.985, 0.0, 0.0),
            border: neutral_border,
            ring: neutral_ring,
            tint_text: oklch(0.985, 0.0, 0.0),
            is_dark: true,
        };

        let primary = tinted_palette(
            oklch(0.922, 0.0, 0.0),
            oklch(0.205, 0.0, 0.0),
            true,
        );
        let secondary = tinted_palette(
            oklch(0.269, 0.0, 0.0),
            oklch(0.985, 0.0, 0.0),
            true,
        );
        let muted = tinted_palette(
            oklch(0.269, 0.0, 0.0),
            oklch(0.708, 0.0, 0.0),
            true,
        );
        let accent = tinted_palette(
            oklch(0.371, 0.0, 0.0),
            oklch(0.985, 0.0, 0.0),
            true,
        );
        let destructive = tinted_palette(
            oklch(0.704, 0.191, 22.216),
            oklch(0.58, 0.22, 27.0),
            true,
        );
        let info = tinted_palette(
            oklch(0.70, 0.16, 245.0),
            oklch(0.205, 0.0, 0.0),
            true,
        );
        let success = tinted_palette(
            oklch(0.72, 0.16, 150.0),
            oklch(0.205, 0.0, 0.0),
            true,
        );
        let warning = tinted_palette(
            oklch(0.80, 0.15, 80.0),
            oklch(0.205, 0.0, 0.0),
            true,
        );

        let card = surface_palette(
            oklch(0.205, 0.0, 0.0),
            oklch(0.985, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            true,
        );
        let popover = surface_palette(
            oklch(0.205, 0.0, 0.0),
            oklch(0.985, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            true,
        );
        let code = surface_palette(
            oklch(0.2, 0.0, 0.0),
            oklch(0.708, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            true,
        );

        let sidebar = SidebarPalette {
            background: oklch(0.205, 0.0, 0.0),
            foreground: oklch(0.985, 0.0, 0.0),
            primary: SidebarPair {
                base: oklch(0.488, 0.243, 264.376),
                foreground: oklch(0.985, 0.0, 0.0),
            },
            accent: SidebarPair {
                base: oklch(0.269, 0.0, 0.0),
                foreground: oklch(0.985, 0.0, 0.0),
            },
            border: oklch_alpha(1.0, 0.0, 0.0, 0.1),
            ring: oklch(0.439, 0.0, 0.0),
        };

        let input = oklch_alpha(1.0, 0.0, 0.0, 0.15);

        build(
            "Dark", neutral, primary, secondary, muted, accent, destructive, info, success, warning,
            card, popover, code, sidebar, input,
        )
    }

    pub fn with_font_sans(mut self, font: FontFamily) -> Self {
        self.font_sans = font;
        self
    }

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

// Tinted-palette border/ring v0 derivation: border = base, ring = base @ 0.5.
// `tint_text = base` matches shadcn `text-{role}` semantics — when a
// transparent-fill widget tints with this hue, text takes the role's
// base color directly (e.g. outlined-destructive renders text in
// destructive's red, not destructive.foreground white).
fn tinted_palette(base: Color, foreground: Color, is_dark: bool) -> Palette {
    Palette {
        base,
        foreground,
        border: base,
        ring: base.alpha(0.5),
        tint_text: base,
        is_dark,
    }
}

// Surface palettes (card, popover, code) inherit neutral's border + ring.
// `tint_text = foreground` — surfaces aren't role tints; the readable
// text color over them is `foreground` (the page text).
fn surface_palette(base: Color, foreground: Color, border: Color, ring: Color, is_dark: bool) -> Palette {
    Palette {
        base,
        foreground,
        border,
        ring,
        tint_text: foreground,
        is_dark,
    }
}

#[allow(clippy::too_many_arguments)]
fn build(
    name: &'static str,
    neutral: Palette,
    primary: Palette,
    secondary: Palette,
    muted: Palette,
    accent: Palette,
    destructive: Palette,
    info: Palette,
    success: Palette,
    warning: Palette,
    card: Palette,
    popover: Palette,
    code: Palette,
    sidebar: SidebarPalette,
    input: Color,
) -> Design {
    let seed = Seed {
        background: neutral.base.into(),
        text: neutral.foreground.into(),
        primary: primary.base.into(),
        success: accent.base.into(),
        danger: destructive.base.into(),
        warning: secondary.base.into(),
    };
    let name = name.to_string();
    let theme = iced_core::Theme::custom(name.clone(), seed);
    Design {
        name,
        neutral,
        primary,
        secondary,
        muted,
        accent,
        destructive,
        info,
        success,
        warning,
        card,
        popover,
        code,
        sidebar,
        input,
        seed,
        theme,
        font_sans: FontFamily::default(),
        font_mono: Font::MONOSPACE,
    }
}
