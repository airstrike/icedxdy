//! Markdown rendering, vendored from `iced_widget::markdown` and extended
//! with configurable syntax-highlighter theme.
//!
//! Upstream is hardcoded to `iced_highlighter::Theme::Base16Ocean`; our
//! [`parse_with_theme`] lets callers pick any theme (e.g.
//! `InspiredGitHub` for shadcn-faithful light-mode rendering).
//!
//! See `widget/src/markdown/widget.rs` for the cp'd source — kept verbatim
//! where possible so future re-vendoring stays mechanical.

#[cfg(feature = "highlighter")]
pub mod highlighter;
pub mod widget;

pub use widget::{Bullet, Catalog, HeadingLevel, Item, Settings, Style, Text, Uri, Viewer};

#[cfg(feature = "markdown")]
pub use widget::{
    code_block, heading, item, ordered_list, paragraph, parse, quote, rule, table,
    unordered_list, view, view_with,
};

#[cfg(feature = "highlighter")]
pub use highlighter::{
    CAPTURE_NAMES, CodeStyle, CodeTheme, DefaultDark, DefaultLight, HighlightedLine,
    HighlightedSpan,
};

// Catalog impl for our own Theme — mirrors iced's default impl on
// `iced_core::Theme` (in widget.rs:1649-1653) so callers using
// `icedxdy_core::Theme` can render markdown.
#[cfg(feature = "markdown")]
mod theme_impl {
    use icedxdy_core::Theme;
    use icedxdy_core::theme::container::code_block;

    impl super::widget::Catalog for Theme {
        fn code_block<'a>() -> <Self as iced_widget::container::Catalog>::Class<'a> {
            Box::new(code_block)
        }
    }
}
