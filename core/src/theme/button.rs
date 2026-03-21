//! Button styling matching shadcn/ui v4 radix variants
//!
//! See plan/shadcn-radix-button-guide.md for full spec.

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

/// Default/primary: bg-primary text-primary-foreground hover:bg-primary/80
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
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(p.primary.base.alpha(0.8).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(p.primary.base.alpha(0.7).into()),
            ..base
        },
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
/// Light: border-border bg-background hover:bg-muted
/// Dark:  border-input  bg-input/30  hover:bg-input/50
pub fn outline(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();
    let dark = is_dark(theme);

    let (bg, border_color, hover_bg, text_color, hover_text) = if dark {
        // dark:bg-input/30 = oklch(1 0 0 / 15%) * 0.3 = ~4.5% white
        // dark:hover:bg-input/50 = oklch(1 0 0 / 15%) * 0.5 = ~7.5% white
        let input_a = p.input.a;
        (
            p.input.alpha(input_a * 0.3),
            p.input,
            p.input.alpha(input_a * 0.5),
            p.muted.foreground,
            p.foreground,
        )
    } else {
        (
            p.background,
            p.border,
            p.muted.base,
            p.foreground,
            p.foreground,
        )
    };

    let base = Style {
        background: Some(bg.into()),
        text_color: text_color.into(),
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
            text_color: hover_text.into(),
            ..base
        },
        Status::Pressed => Style {
            background: Some(hover_bg.alpha(0.9).into()),
            text_color: hover_text.into(),
            ..base
        },
        Status::Disabled => Style {
            background: Some(bg.alpha(0.5).into()),
            text_color: text_color.alpha(0.5).into(),
            border: Border {
                color: border_color.alpha(0.5).into(),
                ..base.border
            },
            ..base
        },
    }
}

/// Ghost:
/// Light: hover:bg-muted
/// Dark:  hover:bg-muted/50
pub fn ghost(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();
    let dark = is_dark(theme);

    let hover_bg = if dark {
        p.muted.base.alpha(0.5) // dark:hover:bg-muted/50
    } else {
        p.muted.base            // hover:bg-muted
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

/// Link: text-primary hover:underline (opacity fallback since iced lacks underline)
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
