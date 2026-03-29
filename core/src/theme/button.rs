//! Button styling — nova style
//!
//! See plan/nova-style-guide.md for full spec.

pub use iced_widget::button::{Catalog, Status, Style, StyleFn};

use iced_core::Border;

use super::Theme;
use crate::radius;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn is_dark(theme: &Theme) -> bool {
    theme.palette().background.l < 0.5
}

/// Default/primary: bg-primary text-primary-foreground
/// Nova: NO hover fill change on buttons ([a]:hover only applies inside <a>)
pub fn default(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    let base = Style {
        background: Some(p.primary.base.into()),
        text_color: p.primary.foreground.into(),
        border: Border {
            radius: radius::MD.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        Status::Active | Status::Hovered | Status::Pressed => base,
        Status::Disabled => Style {
            background: Some(p.primary.base.alpha(0.5).into()),
            text_color: p.primary.foreground.alpha(0.5).into(),
            ..base
        },
    }
}

/// Secondary: bg-secondary text-secondary-foreground hover:bg-secondary/80
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    let base = Style {
        background: Some(p.secondary.base.into()),
        text_color: p.secondary.foreground.into(),
        border: Border {
            radius: radius::MD.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(p.secondary.base.alpha(0.8).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(p.secondary.base.alpha(0.7).into()),
            ..base
        },
        Status::Disabled => Style {
            background: Some(p.secondary.base.alpha(0.5).into()),
            text_color: p.secondary.foreground.alpha(0.5).into(),
            ..base
        },
    }
}

/// Outline:
/// Light: border-border bg-background hover:bg-muted hover:text-foreground
/// Dark:  border-input  bg-input/30  hover:bg-input/50
/// Nova: no shadow
pub fn outline(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();
    let dark = is_dark(theme);

    // Dark: input is oklch(1 0 0 / 15%). Tailwind /30 and /50 MULTIPLY the alpha.
    let input_a = p.input.a;
    let (bg, border_color, hover_bg) = if dark {
        (
            p.input.alpha(input_a * 0.3),  // ~4.5% white
            p.input,                       // oklch(1 0 0 / 15%)
            p.input.alpha(input_a * 0.5),  // ~7.5% white
        )
    } else {
        (
            p.background,                  // white
            p.border,                      // oklch(0.922 0 0)
            p.muted.base,                  // oklch(0.97 0 0)
        )
    };

    let base = Style {
        background: Some(bg.into()),
        text_color: p.foreground.into(),
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: border_color.into(),
        },
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(hover_bg.into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(if dark {
                p.input.alpha(input_a * 0.6)
            } else {
                p.muted.base.alpha(0.9)
            }.into()),
            ..base
        },
        Status::Disabled => Style {
            background: Some(bg.alpha(0.5).into()),
            text_color: p.foreground.alpha(0.5).into(),
            border: Border {
                color: border_color.alpha(0.5).into(),
                ..base.border
            },
            ..base
        },
    }
}

/// Ghost:
/// hover:bg-muted hover:text-foreground
/// dark:hover:bg-muted/50
pub fn ghost(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();
    let dark = is_dark(theme);

    let hover_bg = if dark {
        p.muted.base.alpha(0.5)
    } else {
        p.muted.base
    };

    let base = Style {
        background: None,
        text_color: p.foreground.into(),
        border: Border {
            radius: radius::MD.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(hover_bg.into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(hover_bg.alpha(0.9).into()),
            ..base
        },
        Status::Disabled => Style {
            text_color: p.foreground.alpha(0.5).into(),
            ..base
        },
    }
}

/// Destructive:
/// Light: bg-destructive/10 hover:bg-destructive/20 text-destructive
/// Dark:  bg-destructive/20 hover:bg-destructive/30 text-destructive
pub fn destructive(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();
    let dark = is_dark(theme);

    let (rest_alpha, hover_alpha) = if dark {
        (0.2, 0.3)
    } else {
        (0.1, 0.2)
    };

    let base = Style {
        background: Some(p.destructive.base.alpha(rest_alpha).into()),
        text_color: p.destructive.base.into(),
        border: Border {
            radius: radius::MD.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(p.destructive.base.alpha(hover_alpha).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(p.destructive.base.alpha(hover_alpha + 0.05).into()),
            ..base
        },
        Status::Disabled => Style {
            background: Some(p.destructive.base.alpha(rest_alpha * 0.5).into()),
            text_color: p.destructive.base.alpha(0.5).into(),
            ..base
        },
    }
}

/// Sidebar: inactive sidebar item
/// No bg at rest, bg-accent on hover, text muted→accent-foreground
pub fn sidebar(theme: &Theme, status: Status) -> Style {
    let sb = &theme.palette().sidebar;

    let base = Style {
        background: None,
        text_color: sb.foreground.into(),
        border: Border {
            radius: radius::SM.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(sb.accent.base.into()),
            text_color: sb.accent.foreground.into(),
            ..base
        },
        Status::Pressed => Style {
            background: Some(sb.accent.base.into()),
            text_color: sb.accent.foreground.into(),
            ..base
        },
        Status::Disabled => Style {
            text_color: sb.foreground.alpha(0.5).into(),
            ..base
        },
    }
}

/// Sidebar active: currently selected sidebar item
/// Uses main accent (brighter) not sidebar-accent, matching shadcn docs-sidebar
pub fn sidebar_active(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    let base = Style {
        background: Some(p.accent.base.into()),
        text_color: p.accent.foreground.into(),
        border: Border {
            radius: radius::SM.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        Status::Active | Status::Hovered | Status::Pressed => base,
        Status::Disabled => Style {
            background: Some(p.accent.base.alpha(0.5).into()),
            text_color: p.accent.foreground.alpha(0.5).into(),
            ..base
        },
    }
}

/// Link: text-primary underline-offset-4 hover:underline
/// No bg, no border. Opacity fallback since iced lacks underline.
pub fn link(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    let base = Style {
        background: None,
        text_color: p.primary.base.into(),
        border: Border::default(),
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            text_color: p.primary.base.alpha(0.8).into(),
            ..base
        },
        Status::Pressed => Style {
            text_color: p.primary.base.alpha(0.7).into(),
            ..base
        },
        Status::Disabled => Style {
            text_color: p.primary.base.alpha(0.5).into(),
            ..base
        },
    }
}
