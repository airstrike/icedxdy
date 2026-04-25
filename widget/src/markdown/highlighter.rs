//! Tree-sitter syntax highlighter for markdown code blocks.
//!
//! Tokenisation is grammar-driven via tree-sitter; capture names map to a
//! [`Theme`] at draw time. Bundled queries come from the Helix editor
//! (MIT-licensed, vendored under `queries/<lang>/`) — they're the
//! highest-quality public tree-sitter queries available and use a
//! comprehensive capture taxonomy (`@namespace`, `@function.macro`,
//! `@variable.other.member`, …).
//!
//! Downstream apps can [`register`] additional languages with their own
//! `.scm` query files — e.g. `tree-sitter-python` + Helix's python
//! highlights.

use std::collections::HashMap;
use std::ops::Range;
use std::sync::{LazyLock, RwLock};

use iced_core::{Color, Font, font};

use tree_sitter::QueryError;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent};

/// Capture names recognised by the highlighter. Order matters only for
/// presentation; lookups use longest-prefix match (e.g. `@function.macro`
/// matches `function.macro` if present, else falls back to `function`).
///
/// Set covers Helix's tree-sitter conventions plus the legacy
/// `tree-sitter-highlight` standard names. A [`Theme`] returns a
/// [`Style`] for each index in this slice.
pub const CAPTURE_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "function",
    "function.builtin",
    "function.macro",
    "function.method",
    "keyword",
    "label",
    "module",
    "namespace",
    "number",
    "operator",
    "property",
    "property.builtin",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "special",
    "string",
    "string.escape",
    "string.regexp",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "type.enum",
    "variable",
    "variable.builtin",
    "variable.mutable",
    "variable.other.member",
    "variable.parameter",
];

/// Per-token output of the highlighter.
#[derive(Debug, Clone)]
pub struct Span {
    pub text: String,
    pub capture: Option<u32>,
}

/// One line of highlighted code: a sequence of spans.
pub type Line = Vec<Span>;

/// Visual style for a captured token. Both fields optional — `None` means
/// "use the renderer's default".
#[derive(Debug, Default, Clone, Copy)]
pub struct Style {
    pub color: Option<Color>,
    pub font: Option<Font>,
}

/// Maps highlighter capture indices to visual styles. Implementations like
/// [`Light`] / [`Dark`] live in this module; downstream crates can
/// implement their own (e.g., a shadcn-faithful GitHub theme).
pub trait Theme {
    fn style(&self, capture: u32) -> Style;
}

// --- Language registry ----------------------------------------------------------

/// Build a configured `HighlightConfiguration` from raw query strings.
fn build_config(
    name: &str,
    grammar: tree_sitter::Language,
    highlights: &str,
    injections: &str,
    locals: &str,
) -> Result<HighlightConfiguration, QueryError> {
    let mut config = HighlightConfiguration::new(grammar, name, highlights, injections, locals)?;
    config.configure(CAPTURE_NAMES);
    Ok(config)
}

/// Built-in Rust support using Helix's vendored queries.
fn rust_config() -> HighlightConfiguration {
    build_config(
        "rust",
        tree_sitter_rust::LANGUAGE.into(),
        include_str!("queries/rust/highlights.scm"),
        include_str!("queries/rust/injections.scm"),
        include_str!("queries/rust/locals.scm"),
    )
    .expect("vendored rust queries are valid")
}

/// Global language registry. Initialised lazily with the built-in Rust
/// config; callers can [`register`] additional languages at app start.
///
/// Configurations are stored as `&'static` references via [`Box::leak`]
/// — registering a language is a one-shot, app-lifetime decision (the
/// memory cost is on the order of tens of KB per language).
static REGISTRY: LazyLock<RwLock<HashMap<String, &'static HighlightConfiguration>>> =
    LazyLock::new(|| {
        let leaked: &'static HighlightConfiguration = Box::leak(Box::new(rust_config()));
        let mut map = HashMap::new();
        map.insert("rust".to_owned(), leaked);
        map.insert("rs".to_owned(), leaked);
        RwLock::new(map)
    });

/// Register a tree-sitter language for syntax highlighting. The
/// `highlights`, `injections`, and `locals` arguments are the raw
/// contents of `.scm` query files (typically loaded with
/// `include_str!`). Pass `""` for an empty query.
///
/// `tokens` are the language identifiers used in markdown code-block
/// fences (e.g. `["python", "py"]` so both ` ```python ` and ` ```py `
/// resolve to this language).
///
/// # Example
///
/// ```ignore
/// use icedxdy::widget::markdown::highlighter;
///
/// highlighter::register(
///     ["python", "py"],
///     tree_sitter_python::LANGUAGE.into(),
///     include_str!("queries/python/highlights.scm"),
///     include_str!("queries/python/injections.scm"),
///     "",
/// ).expect("python highlight config");
/// ```
pub fn register<I, S>(
    tokens: I,
    grammar: tree_sitter::Language,
    highlights: &str,
    injections: &str,
    locals: &str,
) -> Result<(), QueryError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let config = build_config("custom", grammar, highlights, injections, locals)?;
    let leaked: &'static HighlightConfiguration = Box::leak(Box::new(config));
    let mut map = REGISTRY.write().expect("highlighter registry poisoned");
    for token in tokens {
        map.insert(token.into(), leaked);
    }
    Ok(())
}

fn config_for(language: &str) -> Option<&'static HighlightConfiguration> {
    REGISTRY
        .read()
        .expect("highlighter registry poisoned")
        .get(language)
        .copied()
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

    /// Highlight `code` written in `language`, returning one [`Line`] per
    /// source line. Unrecognised languages yield unstyled spans (one per
    /// line, capture = `None`).
    pub fn highlight(&mut self, code: &str, language: &str) -> Vec<Line> {
        let Some(config) = config_for(language) else {
            return code.split('\n').map(unstyled_line).collect();
        };

        let events = match self
            .inner
            .highlight(config, code.as_bytes(), None, |_| None)
        {
            Ok(events) => events,
            Err(_) => return code.split('\n').map(unstyled_line).collect(),
        };

        let mut stack: Vec<u32> = Vec::new();
        let mut flat: Vec<(Range<usize>, Option<u32>)> = Vec::new();

        for event in events {
            match event {
                Ok(HighlightEvent::HighlightStart(highlight)) => {
                    stack.push(highlight.0 as u32);
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
        let mut lines: Vec<Line> = Vec::new();
        let mut current: Line = Vec::new();
        for (range, capture) in flat {
            let chunk = &code[range];
            for (i, segment) in chunk.split('\n').enumerate() {
                if i > 0 {
                    lines.push(std::mem::take(&mut current));
                }
                if !segment.is_empty() {
                    current.push(Span {
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

fn unstyled_line(line: &str) -> Line {
    vec![Span {
        text: line.to_owned(),
        capture: None,
    }]
}

// --- Default themes -------------------------------------------------------------
//
// The colour tables below come straight from Shiki's `github-light-default` and
// `github-dark` themes — the same palettes shadcn's nova style ships
// (`apps/v4/source.config.ts`). No nova-specific CSS overrides exist; the styling
// is 100% delegated to these Shiki themes.
//
// Source:
//   https://raw.githubusercontent.com/shikijs/textmate-grammars-themes/main/
//     packages/tm-themes/themes/github-{light-default,dark}.json
//
// Mapping from TextMate scope → tree-sitter capture follows the pattern in
// `tree-sitter-highlight`'s docs (capture names match the highlights query).

const fn rgb(hex: u32) -> Color {
    Color {
        r: ((hex >> 16) & 0xff) as u8 as f32 / 255.0,
        g: ((hex >> 8) & 0xff) as u8 as f32 / 255.0,
        b: (hex & 0xff) as u8 as f32 / 255.0,
        a: 1.0,
    }
}

const fn bold() -> Font {
    Font {
        weight: font::Weight::Bold,
        ..Font::MONOSPACE
    }
}

const fn italic() -> Font {
    Font {
        style: font::Style::Italic,
        ..Font::MONOSPACE
    }
}

fn fg(hex: u32) -> Style {
    Style {
        color: Some(rgb(hex)),
        font: None,
    }
}

fn fg_bold(hex: u32) -> Style {
    Style {
        color: Some(rgb(hex)),
        font: Some(bold()),
    }
}

fn fg_italic(hex: u32) -> Style {
    Style {
        color: Some(rgb(hex)),
        font: Some(italic()),
    }
}

/// GitHub Light Default — the light theme shadcn's nova style ships for code
/// blocks (via Shiki).
pub struct Light;

impl Theme for Light {
    fn style(&self, capture: u32) -> Style {
        let name = CAPTURE_NAMES.get(capture as usize).copied().unwrap_or("");
        match name {
            "comment" => fg(0x6e7781),
            "string" | "string.regexp" | "string.special" => fg(0x0a3069),
            "string.escape" => fg_bold(0x116329),
            "constant"
            | "constant.builtin"
            | "number"
            | "function.builtin"
            | "type.builtin"
            | "variable.builtin"
            | "attribute"
            | "property"
            | "property.builtin"
            | "variable.other.member"
            | "module"
            | "namespace" => fg(0x0550ae),
            "keyword" | "operator" | "embedded" | "special" => fg(0xcf222e),
            "function" | "function.macro" | "function.method" | "constructor" => fg(0x8250df),
            "type" | "type.enum" | "tag" => fg(0x116329),
            "variable" | "variable.mutable" | "label" => fg(0x953800),
            "punctuation.bracket" => fg(0x57606a),
            // `punctuation`, `punctuation.delimiter`, `punctuation.special`,
            // `variable.parameter` — no explicit Shiki rule; inherit foreground.
            _ => Style::default(),
        }
    }
}

/// GitHub Dark — the dark theme shadcn's nova style ships for code blocks
/// (via Shiki).
pub struct Dark;

impl Theme for Dark {
    fn style(&self, capture: u32) -> Style {
        let name = CAPTURE_NAMES.get(capture as usize).copied().unwrap_or("");
        match name {
            "comment" => fg(0x6a737d),
            "string" | "string.special" => fg(0x9ecbff),
            "string.regexp" => fg(0xdbedff),
            "string.escape" => fg_bold(0x85e89d),
            "constant"
            | "constant.builtin"
            | "number"
            | "function.builtin"
            | "type.builtin"
            | "variable.builtin"
            | "attribute"
            | "property"
            | "property.builtin"
            | "variable.other.member"
            | "module"
            | "namespace" => fg(0x79b8ff),
            "keyword" | "operator" | "embedded" | "special" => fg(0xf97583),
            "function" | "function.macro" | "function.method" | "constructor" => fg(0xb392f0),
            "type" | "type.enum" | "tag" => fg(0x85e89d),
            "variable" | "variable.mutable" | "label" => fg(0xffab70),
            "punctuation.bracket" => fg(0xd1d5da),
            _ => Style::default(),
        }
    }
}

// `fg_italic` is currently unused (GitHub themes only italicise markup.italic
// which doesn't appear in code blocks). Kept around because the Shiki theme
// JSON does declare italic on a couple of scopes — when we add markup support
// to the highlighter it'll wire up here.
#[allow(dead_code)]
fn _keep_fg_italic_referenced() -> Style {
    fg_italic(0x000000)
}
