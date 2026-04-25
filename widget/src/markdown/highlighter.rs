//! Tree-sitter syntax highlighter for markdown code blocks.
//!
//! Replaces `iced_highlighter` (regex-based syntect) with `tree-sitter`.
//! Tokenization is grammar-driven and accurate; capture names map to a
//! [`CodeTheme`] at draw time (chunk B will move this from parse-time to
//! Viewer-time per the design discussion).

use std::ops::Range;
use std::sync::LazyLock;

use iced_core::{Color, Font, font};

use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent};

/// Standard capture names (from `tree-sitter-highlight` conventions).
/// The same set must be `configure`d on each `HighlightConfiguration`.
/// A [`CodeTheme`] returns a [`CodeStyle`] for each index in this slice.
pub const CAPTURE_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "function",
    "function.builtin",
    "keyword",
    "label",
    "module",
    "number",
    "operator",
    "property",
    "property.builtin",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regexp",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

/// Per-token output of the highlighter.
#[derive(Debug, Clone)]
pub struct HighlightedSpan {
    pub text: String,
    pub capture: Option<u32>,
}

/// One line of highlighted code: a sequence of spans.
pub type HighlightedLine = Vec<HighlightedSpan>;

/// Visual style for a captured token. Both fields optional — `None` means
/// "use the renderer's default".
#[derive(Debug, Default, Clone, Copy)]
pub struct CodeStyle {
    pub color: Option<Color>,
    pub font: Option<Font>,
}

/// Maps highlighter capture indices to visual styles. Implementations like
/// [`DefaultDark`] / [`DefaultLight`] live in this module; downstream
/// crates can implement their own (e.g., a shadcn-faithful GitHub theme).
pub trait CodeTheme {
    fn style(&self, capture: u32) -> CodeStyle;
}

// --- Per-language configuration -------------------------------------------------

/// Pre-configured `tree-sitter-highlight::HighlightConfiguration` for Rust.
/// More languages will get their own `LazyLock` entry as we ship pages that
/// need them (TOML, JSON, Bash…).
static RUST_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter_rust::LANGUAGE.into(),
        "rust",
        tree_sitter_rust::HIGHLIGHTS_QUERY,
        tree_sitter_rust::INJECTIONS_QUERY,
        "",
    )
    .expect("rust highlight config");
    config.configure(CAPTURE_NAMES);
    config
});

fn config_for(language: &str) -> Option<&'static HighlightConfiguration> {
    match language {
        "rust" | "rs" => Some(&RUST_CONFIG),
        _ => None,
    }
}

// --- Highlighter wrapper --------------------------------------------------------

/// Streaming-style wrapper around `tree-sitter-highlight`.
/// Stateless across snippets; cheap to construct.
pub struct Highlighter {
    inner: tree_sitter_highlight::Highlighter,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self {
            inner: tree_sitter_highlight::Highlighter::new(),
        }
    }
}

impl Highlighter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Highlight `code` written in `language`, returning one
    /// [`HighlightedLine`] per source line. Unrecognised languages yield
    /// unstyled spans (one per line, capture = `None`).
    pub fn highlight(&mut self, code: &str, language: &str) -> Vec<HighlightedLine> {
        let Some(config) = config_for(language) else {
            return code
                .split('\n')
                .map(|line| {
                    vec![HighlightedSpan {
                        text: line.to_owned(),
                        capture: None,
                    }]
                })
                .collect();
        };

        let events = match self
            .inner
            .highlight(config, code.as_bytes(), None, |_| None)
        {
            Ok(events) => events,
            Err(_) => {
                return code
                    .split('\n')
                    .map(|line| {
                        vec![HighlightedSpan {
                            text: line.to_owned(),
                            capture: None,
                        }]
                    })
                    .collect();
            }
        };

        let mut stack: Vec<u32> = Vec::new();
        let mut flat: Vec<(Range<usize>, Option<u32>)> = Vec::new();

        for event in events {
            match event {
                Ok(HighlightEvent::HighlightStart(Highlight(idx))) => {
                    stack.push(idx as u32);
                }
                Ok(HighlightEvent::HighlightEnd) => {
                    stack.pop();
                }
                Ok(HighlightEvent::Source { start, end }) => {
                    flat.push((start..end, stack.last().copied()));
                }
                Err(_) => break,
            }
        }

        // Split flat spans on '\n' into lines.
        let mut lines: Vec<HighlightedLine> = Vec::new();
        let mut current: HighlightedLine = Vec::new();
        for (range, capture) in flat {
            let chunk = &code[range];
            for (i, segment) in chunk.split('\n').enumerate() {
                if i > 0 {
                    lines.push(std::mem::take(&mut current));
                }
                if !segment.is_empty() {
                    current.push(HighlightedSpan {
                        text: segment.to_owned(),
                        capture,
                    });
                }
            }
        }
        if !current.is_empty() || lines.is_empty() {
            lines.push(current);
        }
        lines
    }
}

// --- Default themes (placeholders until shadcn-faithful ones land) --------------

const fn rgba(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

const fn italic() -> Font {
    Font {
        style: font::Style::Italic,
        ..Font::MONOSPACE
    }
}

/// Minimal default-light theme. Approximates GitHub Light. Shadcn-faithful
/// theme will replace this once we hand-translate the Shiki palette.
pub struct DefaultLight;

impl CodeTheme for DefaultLight {
    fn style(&self, capture: u32) -> CodeStyle {
        let name = CAPTURE_NAMES.get(capture as usize).copied().unwrap_or("");
        let color = match name {
            "comment" => Some(rgba(0x6E, 0x73, 0x80)),
            "keyword" => Some(rgba(0xCF, 0x22, 0x2E)),
            "string" | "string.escape" | "string.special" | "string.regexp" => {
                Some(rgba(0x0A, 0x30, 0x69))
            }
            "number" | "constant" | "constant.builtin" => Some(rgba(0x09, 0x50, 0xA1)),
            "function" | "function.builtin" => Some(rgba(0x82, 0x50, 0x09)),
            "type" | "type.builtin" | "constructor" => Some(rgba(0x95, 0x3A, 0x14)),
            "attribute" | "label" => Some(rgba(0x11, 0x6A, 0x4E)),
            "punctuation"
            | "punctuation.bracket"
            | "punctuation.delimiter"
            | "punctuation.special"
            | "operator" => None,
            "variable.builtin" | "variable.parameter" => Some(rgba(0x95, 0x3A, 0x14)),
            _ => None,
        };
        let font = matches!(name, "comment").then(italic);
        CodeStyle { color, font }
    }
}

/// Minimal default-dark theme.
pub struct DefaultDark;

impl CodeTheme for DefaultDark {
    fn style(&self, capture: u32) -> CodeStyle {
        let name = CAPTURE_NAMES.get(capture as usize).copied().unwrap_or("");
        let color = match name {
            "comment" => Some(rgba(0x8B, 0x94, 0x9E)),
            "keyword" => Some(rgba(0xFF, 0x7B, 0x72)),
            "string" | "string.escape" | "string.special" | "string.regexp" => {
                Some(rgba(0xA5, 0xD6, 0xFF))
            }
            "number" | "constant" | "constant.builtin" => Some(rgba(0x79, 0xC0, 0xFF)),
            "function" | "function.builtin" => Some(rgba(0xD2, 0xA8, 0xFF)),
            "type" | "type.builtin" | "constructor" => Some(rgba(0xFF, 0xA6, 0x57)),
            "attribute" | "label" => Some(rgba(0x7E, 0xE7, 0x87)),
            "punctuation"
            | "punctuation.bracket"
            | "punctuation.delimiter"
            | "punctuation.special"
            | "operator" => None,
            "variable.builtin" | "variable.parameter" => Some(rgba(0xFF, 0xA6, 0x57)),
            _ => None,
        };
        let font = matches!(name, "comment").then(italic);
        CodeStyle { color, font }
    }
}
