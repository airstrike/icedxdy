//! Markdown styling

use iced_core::text::Highlight;
use iced_core::text::highlighter::{self, Highlighter};
use iced_core::theme::Base;
use iced_core::{Code, border, font, padding};
use iced_widget::container;
use iced_widget::markdown::{Catalog, InlineCode};

use super::Theme;
use super::container::Style;

impl Catalog for Theme {
    fn id(&self) -> &str {
        Base::name(self)
    }

    fn link_color(&self) -> iced_core::Color {
        self.primary.base.into()
    }

    /// shadcn's inline `<code>` chip: `bg-muted` with body-colored text.
    fn code(&self) -> InlineCode {
        InlineCode {
            padding: padding::horizontal(2),
            highlight: Highlight {
                background: iced_core::Background::Color(self.muted.base.into()),
                border: border::rounded(4),
            },
            color: self.neutral.foreground.into(),
        }
    }

    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a> {
        // theme::container::code_block returns our OKLCH-native Style;
        // wrap to convert to iced::container::Style for the Catalog Class.
        Box::new(|t| super::container::code_block(t).into())
    }

    fn quote<'a>() -> <Self as container::Catalog>::Class<'a> {
        Box::new(|t| {
            Style {
                text_color: Some(t.muted.foreground),
                background: Some(t.muted.base.into()),
                border: crate::Border {
                    radius: crate::radius::MD.into(),
                    ..crate::Border::default()
                },
                ..Style::default()
            }
            .into()
        })
    }

    fn highlighter(&self) -> &dyn Highlighter<Code, Self> {
        &highlight
    }
}

/// Highlights Markdown code regions with the [`Theme`]'s semantic palettes.
fn highlight(code: Code, theme: &Theme) -> highlighter::Style {
    let color = match code {
        Code::Keyword | Code::Support => Some(theme.primary.base),
        Code::Type | Code::Path | Code::Function => Some(theme.warning.base),
        Code::Variable | Code::Constant | Code::Invalid => Some(theme.destructive.base),
        Code::String => Some(theme.success.base),
        Code::Punctuation | Code::Comment => Some(theme.muted.foreground),
        Code::Other => None,
    };

    highlighter::Style {
        color: color.map(Into::into),
        style: (code == Code::Comment).then_some(font::Style::Italic),
    }
}
