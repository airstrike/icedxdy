//! Markdown rendering, vendored from `iced_widget::markdown` and extended
//! with a tree-sitter syntax highlighter (replacing iced_highlighter's
//! regex-grammar syntect engine) and a parse-once / theme-at-draw
//! pipeline (`Viewer::code_theme`).
//!
//! Built-in language: Rust, using Helix's vendored queries. Downstream
//! apps can register additional languages with [`highlighter::register`].
//!
//! See `widget/src/markdown/widget.rs` for the cp'd iced source — kept
//! verbatim where possible so future re-vendoring stays mechanical.

#[cfg(feature = "highlighter")]
pub mod highlighter;
pub mod widget;

pub use widget::{Bullet, Catalog, HeadingLevel, Item, Settings, Style, Text, Uri, Viewer};

#[cfg(feature = "markdown")]
pub use widget::{
    code_block, heading, item, ordered_list, paragraph, parse, quote, rule, table,
    unordered_list, view, view_with,
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
