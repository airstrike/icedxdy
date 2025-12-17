//! Button styling matching shadcn/ui variants
//!
//! Variants:
//! - default: bg-primary text-primary-foreground, hover:bg-primary/90
//! - secondary: bg-secondary text-secondary-foreground, hover:bg-secondary/80
//! - outline: border bg-background, hover:bg-accent text-accent-foreground
//! - ghost: transparent, hover:bg-accent text-accent-foreground
//! - destructive: bg-destructive text-white, hover:bg-destructive/90
//! - link: text-primary, hover:underline

pub use iced_widget::button::{Catalog, Status, Style, StyleFn};

use iced_core::Border;

use super::Theme;
use icedxdy_core::radius;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// Default/primary button: bg-primary text-primary-foreground
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
            background: Some(p.primary.base.alpha(0.9).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(p.primary.base.alpha(0.85).into()),
            ..base
        },
        Status::Disabled => Style {
            background: Some(p.primary.base.alpha(0.5).into()),
            text_color: p.primary.foreground.alpha(0.5).into(),
            ..base
        },
    }
}

/// Secondary button: bg-secondary text-secondary-foreground
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

/// Outline button: border bg-background, hover:bg-accent
pub fn outline(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    let base = Style {
        background: Some(p.background.into()),
        text_color: p.foreground.into(),
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: p.border.into(),
        },
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(p.accent.base.into()),
            text_color: p.accent.foreground.into(),
            ..base
        },
        Status::Pressed => Style {
            background: Some(p.accent.base.alpha(0.9).into()),
            text_color: p.accent.foreground.into(),
            ..base
        },
        Status::Disabled => Style {
            background: Some(p.background.alpha(0.5).into()),
            text_color: p.foreground.alpha(0.5).into(),
            border: Border {
                color: p.border.alpha(0.5).into(),
                ..base.border
            },
            ..base
        },
    }
}

/// Ghost button: transparent, hover:bg-accent
pub fn ghost(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

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
            background: Some(p.accent.base.into()),
            text_color: p.accent.foreground.into(),
            ..base
        },
        Status::Pressed => Style {
            background: Some(p.accent.base.alpha(0.9).into()),
            text_color: p.accent.foreground.into(),
            ..base
        },
        Status::Disabled => Style {
            text_color: p.foreground.alpha(0.5).into(),
            ..base
        },
    }
}

/// Destructive button: bg-destructive text-white
pub fn destructive(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    let base = Style {
        background: Some(p.destructive.base.into()),
        text_color: icedxdy_core::Color::new(1.0, 0.0, 0.0).into(), // white
        border: Border {
            radius: radius::MD.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(p.destructive.base.alpha(0.9).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(p.destructive.base.alpha(0.85).into()),
            ..base
        },
        Status::Disabled => Style {
            background: Some(p.destructive.base.alpha(0.5).into()),
            text_color: icedxdy_core::Color::new(1.0, 0.0, 0.0).alpha(0.5).into(),
            ..base
        },
    }
}

/// Link button: text-primary (underline on hover not supported, using opacity instead)
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
