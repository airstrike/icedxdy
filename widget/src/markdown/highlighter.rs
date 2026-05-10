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
use serde::Deserialize;

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
    "keyword.control.import",
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

/// Built-in Rust support using Helix's vendored queries plus a small
/// `highlights-macro-extra.scm` patch that catches paths/calls inside
/// `token_tree` macro bodies (Helix doesn't, by design).
fn rust_config() -> HighlightConfiguration {
    let highlights = concat!(
        include_str!("queries/rust/highlights.scm"),
        "\n",
        include_str!("queries/rust/highlights-macro-extra.scm"),
    );
    build_config(
        "rust",
        tree_sitter_rust::LANGUAGE.into(),
        highlights,
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

// --- Themes (data) --------------------------------------------------------------
//
// Themes are *data*: a [`Map`] from capture index → [`Style`]. The two
// built-in maps ([`github_light`], [`github_dark`]) hold the palettes
// shadcn's nova style ships via Shiki's `github-light-default` and
// `github-dark` themes (verified from
// shikijs/textmate-grammars-themes — no nova-specific overrides).
//
// Custom themes:
//
// ```ignore
// use icedxdy::widget::markdown::highlighter::{fg, fg_bold, Map};
//
// let monokai = Map::from_entries([
//     ("comment", fg(0x75715e)),
//     ("keyword", fg(0xf92672)),
//     ("string",  fg(0xe6db74)),
//     // …
// ]);
// ```

const fn rgb(hex: u32) -> Color {
    Color {
        r: ((hex >> 16) & 0xff) as u8 as f32 / 255.0,
        g: ((hex >> 8) & 0xff) as u8 as f32 / 255.0,
        b: (hex & 0xff) as u8 as f32 / 255.0,
        a: 1.0,
    }
}

const fn bold_font() -> Font {
    Font {
        weight: font::Weight::Bold,
        ..Font::MONOSPACE
    }
}

const fn italic_font() -> Font {
    Font {
        style: font::Style::Italic,
        ..Font::MONOSPACE
    }
}

/// Foreground-only [`Style`] from a 24-bit RGB hex.
pub fn fg(hex: u32) -> Style {
    Style {
        color: Some(rgb(hex)),
        font: None,
    }
}

/// Foreground + bold [`Style`].
pub fn fg_bold(hex: u32) -> Style {
    Style {
        color: Some(rgb(hex)),
        font: Some(bold_font()),
    }
}

/// Foreground + italic [`Style`].
pub fn fg_italic(hex: u32) -> Style {
    Style {
        color: Some(rgb(hex)),
        font: Some(italic_font()),
    }
}

/// Complete code-syntax theme as data: one [`Style`] per capture in
/// [`CAPTURE_NAMES`]. Captures not explicitly set inherit the renderer's
/// default text colour.
///
/// Build via [`Map::from_entries`] or one of the named constructors
/// ([`github_light`], [`github_dark`]).
#[derive(Debug, Clone)]
pub struct Map {
    styles: Vec<Style>,
    is_dark: bool,
}

impl Map {
    /// Build a [`Map`] from `(capture_name, style)` pairs. Capture names
    /// not in [`CAPTURE_NAMES`] are silently ignored. The `is_dark` flag
    /// is informational — used by hosts that want to mirror the code
    /// theme's light/dark mode in their surrounding UI.
    pub fn from_entries<I, S>(entries: I) -> Self
    where
        I: IntoIterator<Item = (S, Style)>,
        S: AsRef<str>,
    {
        let mut styles = vec![Style::default(); CAPTURE_NAMES.len()];
        for (name, style) in entries {
            if let Some(idx) = CAPTURE_NAMES.iter().position(|n| *n == name.as_ref()) {
                styles[idx] = style;
            }
        }
        Self {
            styles,
            is_dark: false,
        }
    }

    /// Set whether this map represents a dark theme. Returned by
    /// [`Map::from_vscode_theme`] from the JSON's `type` field; the
    /// [`From<&icedxdy_core::Theme>`] impl sets it from the source palette.
    pub fn with_dark(mut self, is_dark: bool) -> Self {
        self.is_dark = is_dark;
        self
    }

    /// Whether this map is intended for a dark UI background.
    pub fn is_dark(&self) -> bool {
        self.is_dark
    }
}

impl Theme for Map {
    fn style(&self, capture: u32) -> Style {
        self.styles
            .get(capture as usize)
            .copied()
            .unwrap_or_default()
    }
}

impl Map {
    /// How many capture slots in this map have non-default styling. Useful
    /// for quickly auditing how much of [`CAPTURE_NAMES`] a parsed theme
    /// actually covers (sparse themes may leave many captures unstyled).
    pub fn populated_count(&self) -> usize {
        self.styles
            .iter()
            .filter(|s| s.color.is_some() || s.font.is_some())
            .count()
    }

    /// Look up the [`Style`] for a capture by name. Returns the default
    /// (no colour, no font) for unknown names — handy for layering one
    /// map's entries onto another (`dst.set_capture(n, src.get_capture(n))`).
    pub fn get_capture(&self, name: &str) -> Style {
        CAPTURE_NAMES
            .iter()
            .position(|n| *n == name)
            .map(|idx| self.styles[idx])
            .unwrap_or_default()
    }

    /// Override the [`Style`] for a single capture by name. Capture names
    /// not in [`CAPTURE_NAMES`] are silently ignored.
    pub fn set_capture(&mut self, name: &str, style: Style) {
        if let Some(idx) = CAPTURE_NAMES.iter().position(|n| *n == name) {
            self.styles[idx] = style;
        }
    }
}

// --- VS Code / TextMate theme JSON parsing --------------------------------------

/// Capture-name → list-of-TextMate-scopes-to-try mapping. Used by
/// [`Map::from_vscode_theme`] to look up colours from a VS Code theme
/// JSON given a tree-sitter capture name.
///
/// Each tree-sitter capture lists a fallback chain of TM scopes (most
/// specific first). The first scope that resolves to a token-color rule
/// in the JSON wins.
const CAPTURE_TO_TM_SCOPES: &[(&str, &[&str])] = &[
    ("comment", &["comment"]),
    ("string", &["string", "string.quoted"]),
    (
        "string.regexp",
        &["string.regexp", "source.regexp", "string"],
    ),
    ("string.special", &["string.special", "string"]),
    (
        "string.escape",
        &[
            "constant.character.escape",
            "string.regexp constant.character.escape",
        ],
    ),
    ("number", &["constant.numeric"]),
    (
        "constant",
        &["constant", "variable.other.constant", "variable.language"],
    ),
    (
        "constant.builtin",
        &[
            "support.constant",
            "constant.language",
            "variable.language",
            "constant",
        ],
    ),
    ("keyword", &["keyword", "storage", "storage.type"]),
    ("operator", &["keyword.operator", "keyword"]),
    ("special", &["keyword.operator", "keyword"]),
    ("function", &["entity.name.function"]),
    (
        "function.builtin",
        &["support.function", "entity.name.function"],
    ),
    ("function.macro", &["entity.name.function"]),
    ("function.method", &["entity.name.function"]),
    ("constructor", &["entity.name.function", "entity.name.tag"]),
    (
        "type",
        &["entity.name.type", "support.class", "entity.name.class"],
    ),
    ("type.builtin", &["support.type", "storage.type"]),
    (
        "type.enum",
        &["variable.other.enummember", "entity.name.type"],
    ),
    ("tag", &["entity.name.tag"]),
    ("variable", &["variable", "variable.other"]),
    ("variable.mutable", &["variable", "variable.other"]),
    ("variable.builtin", &["variable.language"]),
    ("variable.parameter", &["variable.parameter"]),
    (
        "variable.other.member",
        &[
            "variable.other.property",
            "meta.property-name",
            "support.type.property-name",
        ],
    ),
    (
        "attribute",
        &["entity.other.attribute-name", "meta.attribute"],
    ),
    ("module", &["entity.name.namespace", "entity.name.module"]),
    (
        "namespace",
        &["entity.name.namespace", "entity.name.module"],
    ),
    ("punctuation.bracket", &["punctuation"]),
    ("label", &["entity.name.label"]),
    (
        "property",
        &["meta.property-name", "variable.other.property"],
    ),
    (
        "property.builtin",
        &["support.type.property-name", "meta.property-name"],
    ),
];

/// VS Code theme JSON shape (just the bits we care about).
#[derive(Deserialize)]
struct VsCodeTheme {
    #[serde(default)]
    r#type: Option<String>,
    #[serde(rename = "tokenColors", default)]
    token_colors: Vec<TokenRule>,
}

#[derive(Deserialize)]
struct TokenRule {
    scope: Option<TokenScope>,
    settings: TokenSettings,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TokenScope {
    Single(String),
    Multi(Vec<String>),
}

impl TokenScope {
    fn iter(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        match self {
            TokenScope::Single(s) => Box::new(std::iter::once(s.as_str())),
            TokenScope::Multi(v) => Box::new(v.iter().map(String::as_str)),
        }
    }
}

#[derive(Deserialize, Default)]
struct TokenSettings {
    #[serde(default)]
    foreground: Option<String>,
    #[serde(default, rename = "fontStyle")]
    font_style: Option<String>,
}

/// Match-specificity for VS Code TM scopes. Rule scope `comment` matches
/// target `comment.line.documentation` with specificity 1 (one segment);
/// rule `comment.line` matches with specificity 2; exact rule
/// `comment.line.documentation` with 3. Returns 0 for no match.
fn match_specificity(rule_scope: &str, target: &str) -> usize {
    if rule_scope == target {
        return rule_scope.split('.').count();
    }
    if target.starts_with(rule_scope) && target.as_bytes().get(rule_scope.len()) == Some(&b'.') {
        return rule_scope.split('.').count();
    }
    0
}

fn parse_hex(hex: &str) -> Option<Color> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let bytes = u32::from_str_radix(hex, 16).ok()?;
    match hex.len() {
        6 => Some(rgb(bytes)),
        8 => Some(Color {
            r: ((bytes >> 24) & 0xff) as u8 as f32 / 255.0,
            g: ((bytes >> 16) & 0xff) as u8 as f32 / 255.0,
            b: ((bytes >> 8) & 0xff) as u8 as f32 / 255.0,
            a: (bytes & 0xff) as u8 as f32 / 255.0,
        }),
        _ => None,
    }
}

fn parse_font_style(s: &str) -> Option<Font> {
    let bold = s.contains("bold");
    let italic = s.contains("italic");
    match (bold, italic) {
        (true, true) => Some(Font {
            weight: font::Weight::Bold,
            style: font::Style::Italic,
            ..Font::MONOSPACE
        }),
        (true, false) => Some(bold_font()),
        (false, true) => Some(italic_font()),
        (false, false) => None,
    }
}

fn find_style(theme: &VsCodeTheme, target: &str) -> Option<Style> {
    let mut best: Option<&TokenRule> = None;
    let mut best_specificity = 0usize;

    for rule in &theme.token_colors {
        let Some(scope) = &rule.scope else {
            continue;
        };
        for s in scope.iter() {
            let specificity = match_specificity(s, target);
            if specificity > best_specificity && rule.settings.foreground.is_some() {
                best = Some(rule);
                best_specificity = specificity;
            }
        }
    }

    let rule = best?;
    let color = rule.settings.foreground.as_deref().and_then(parse_hex);
    let font = rule
        .settings
        .font_style
        .as_deref()
        .and_then(parse_font_style);
    color.map(|c| Style {
        color: Some(c),
        font,
    })
}

impl Map {
    /// Build a [`Map`] from a VS Code (TextMate) theme JSON document. Only
    /// the `tokenColors` section is read; `colors`, `semanticTokenColors`,
    /// etc. are ignored.
    ///
    /// For each tree-sitter capture in [`CAPTURE_NAMES`], we look up the
    /// best-matching TM scope rule using [`CAPTURE_TO_TM_SCOPES`]'s
    /// fallback chain, then use that rule's foreground + fontStyle.
    pub fn from_vscode_theme(json: &str) -> Result<Self, serde_json::Error> {
        let theme: VsCodeTheme = serde_json::from_str(json)?;
        let mut entries = Vec::with_capacity(CAPTURE_TO_TM_SCOPES.len());
        for &(capture, scopes) in CAPTURE_TO_TM_SCOPES {
            for &scope in scopes {
                if let Some(style) = find_style(&theme, scope) {
                    entries.push((capture.to_string(), style));
                    break;
                }
            }
        }
        let is_dark = theme.r#type.as_deref() == Some("dark");
        Ok(Map::from_entries(entries).with_dark(is_dark))
    }
}

// --- Default Map derived from the icedxdy palette -------------------------------

/// Build a default [`Map`] from an [`icedxdy_core::Theme`] palette. Used
/// as the fallback when the application hasn't supplied a custom code
/// theme — colours come from the same semantic tokens the rest of the
/// UI uses (primary, accent, destructive, muted, …) so the code block
/// looks coherent with the surrounding app chrome regardless of which
/// shadcn-style theme is active.
///
/// Apps that want a specific aesthetic (GitHub Light/Dark, Monokai,
/// etc.) should ship their own VS Code theme JSON and parse it via
/// [`Map::from_vscode_theme`].
impl From<&icedxdy_core::Theme> for Map {
    fn from(theme: &icedxdy_core::Theme) -> Self {
        let p = theme;
        let is_dark = p.neutral.base.l < 0.5;

        // Hybrid mapping — TN's blue-family nuance for the categories
        // where it matters (paths read distinctly from punctuation reads
        // distinctly from `use`-style keywords) layered with shadcn's
        // colour personality (purple functions, green types, red
        // keywords, blue strings, orange attributes) for the rest.
        //
        // Three TN-style blue tones derived from `info.base`:
        //   teal    (info −25° hue)                — paths/namespaces
        //   cyan    (info −20°, +0.10 L, light desat) — struct field access
        //   subtle  (info −15°, +0.20 L, half C)   — punctuation, `use`/`mod`
        //
        // Plus four shadcn accents:
        //   red     destructive                   — keywords, `self`, escapes
        //   blue    info                          — strings, numbers, constants
        //   green   success                       — types, constructors, tags
        //   amber   warning                       — attributes, properties
        //   purple  info.blend(destructive, 0.30) — function names
        let teal = p.info.base.shift_hue(-25.0).into();
        let cyan = p
            .info
            .base
            .shift_hue(-20.0)
            .lighten(0.10)
            .desaturate(0.20)
            .into();
        let subtle = p
            .info
            .base
            .shift_hue(-15.0)
            .lighten(0.20)
            .desaturate(0.50)
            .into();
        let purple = p.info.base.blend(p.destructive.base, 0.30).into();
        let red = p.destructive.base.into();
        let blue = p.info.base.into();
        let green = p.success.base.into();
        let amber = p.warning.base.into();
        let muted = p.muted.foreground.into();
        let fg = p.neutral.foreground.into();

        let semantic = |c: iced_core::Color| Style {
            color: Some(c),
            font: None,
        };

        Map::from_entries([
            ("comment", semantic(muted)),
            ("string", semantic(blue)),
            ("string.regexp", semantic(blue)),
            ("string.special", semantic(blue)),
            ("string.escape", semantic(red)),
            ("number", semantic(blue)),
            ("constant", semantic(blue)),
            ("constant.builtin", semantic(blue)),
            ("keyword", semantic(red)),
            // `use` / `mod` / `as` get TN's light-cyan slot so they read
            // distinctly from both regular keywords (red) and the
            // teal-family path components that follow them.
            ("keyword.control.import", semantic(subtle)),
            ("operator", semantic(subtle)),
            ("embedded", semantic(red)),
            ("special", semantic(red)),
            ("function", semantic(purple)),
            ("function.builtin", semantic(purple)),
            ("function.macro", semantic(purple)),
            ("function.method", semantic(purple)),
            ("constructor", semantic(green)),
            ("type", semantic(green)),
            ("type.builtin", semantic(green)),
            ("type.enum", semantic(green)),
            ("tag", semantic(green)),
            ("attribute", semantic(amber)),
            ("property", semantic(amber)),
            ("property.builtin", semantic(amber)),
            ("variable.other.member", semantic(cyan)),
            ("module", semantic(teal)),
            ("namespace", semantic(teal)),
            ("variable", semantic(fg)),
            ("variable.mutable", semantic(fg)),
            ("variable.builtin", semantic(red)),
            ("variable.parameter", semantic(amber)),
            ("label", semantic(fg)),
            ("punctuation", semantic(subtle)),
            ("punctuation.bracket", semantic(subtle)),
            ("punctuation.delimiter", semantic(subtle)),
            ("punctuation.special", semantic(subtle)),
        ])
        .with_dark(is_dark)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run the highlighter and produce a flat `(text, capture_name)` stream
    /// across the whole source. Whitespace-only spans are kept (capture =
    /// `None`); newlines appear as their own entries so multi-line inputs
    /// stay readable in failure output.
    fn capture_stream(src: &str, lang: &str) -> Vec<(String, Option<&'static str>)> {
        let lines = Highlighter::new().highlight(src, lang);
        let mut out = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            for span in line {
                let name = span
                    .capture
                    .and_then(|idx| CAPTURE_NAMES.get(idx as usize).copied());
                out.push((span.text.clone(), name));
            }
            if i + 1 < lines.len() {
                out.push(("\n".to_string(), None));
            }
        }
        out
    }

    /// Assert the highlighter emits `expected` for `src`. Whitespace-only
    /// entries are filtered on both sides so tests focus on tokens. On
    /// mismatch, prints a side-by-side expected-vs-actual diff.
    fn assert_captures(src: &str, lang: &str, expected: &[(&str, Option<&str>)]) {
        let actual_owned = capture_stream(src, lang);
        let actual: Vec<(&str, Option<&str>)> = actual_owned
            .iter()
            .filter(|(t, _)| !t.chars().all(char::is_whitespace))
            .map(|(t, c)| (t.as_str(), *c))
            .collect();
        let expected: Vec<(&str, Option<&str>)> = expected
            .iter()
            .filter(|(t, _)| !t.chars().all(char::is_whitespace))
            .copied()
            .collect();

        if actual == expected {
            return;
        }

        let mut buf = String::from("\nsource:\n");
        for line in src.lines() {
            buf.push_str("  ");
            buf.push_str(line);
            buf.push('\n');
        }
        buf.push_str("\nexpected vs actual:\n");
        let max = actual.len().max(expected.len());
        for i in 0..max {
            let e = expected.get(i);
            let a = actual.get(i);
            let mark = if e == a { ' ' } else { '✗' };
            buf.push_str(&format!("  {mark} #{i:>3}: {e:?} | {a:?}\n"));
        }
        panic!("{buf}");
    }

    /// Macro bodies are opaque `token_tree` nodes in tree-sitter-rust, so
    /// `path::call(arg)` patterns inside them are invisible to Helix's
    /// upstream queries. This test pins the behaviour of our
    /// `highlights-macro-extra.scm` patch: the path component must
    /// resolve to `@namespace`, the call name to `@function`.
    #[test]
    fn macro_body_paths_and_calls() {
        let src = r#"row![button::outline("X")]"#;
        assert_captures(
            src,
            "rust",
            &[
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("button", Some("namespace")),
                ("::", Some("punctuation.delimiter")),
                ("outline", Some("function")),
                ("(", Some("punctuation.bracket")),
                ("\"X\"", Some("string")),
                (")", Some("punctuation.bracket")),
                ("]", Some("punctuation.bracket")),
            ],
        );
    }

    /// Outside macros, `button::outline(...)` parses as a real
    /// `(call_expression (scoped_identifier))` and Helix's queries handle
    /// it directly. Verify both halves resolve identically to the macro
    /// case so the visual rendering doesn't shift between contexts.
    #[test]
    fn non_macro_paths_and_calls_match_macro_body() {
        let src = r#"button::outline("X");"#;
        assert_captures(
            src,
            "rust",
            &[
                ("button", Some("namespace")),
                ("::", Some("punctuation.delimiter")),
                ("outline", Some("function")),
                ("(", Some("punctuation.bracket")),
                ("\"X\"", Some("string")),
                (")", Some("punctuation.bracket")),
                (";", Some("punctuation.delimiter")),
            ],
        );
    }

    /// Nested macro invocations — `row![ row![...] ]` — only the outer
    /// is parsed as `macro_invocation` by tree-sitter-rust; the inner
    /// is a flat `identifier`, `!`, `token_tree` triple inside the
    /// outer's `token_tree`. Our `highlights-macro-extra.scm` patches
    /// the inner case so both render identically.
    #[test]
    fn nested_macro_invocations() {
        let src = r#"row![row![]]"#;
        assert_captures(
            src,
            "rust",
            &[
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("]", Some("punctuation.bracket")),
                ("]", Some("punctuation.bracket")),
            ],
        );
    }

    /// Multiline nested macros — the same `(token_tree (identifier)
    /// @function.macro . "!" . (token_tree))` pattern that handles
    /// `row![row![]]` (single-line) must still fire when there's
    /// whitespace and a body between the brackets. Pins the exact
    /// shape used in the components-browser examples.
    #[test]
    fn nested_macro_multiline() {
        let src = "row![\n    row![\n        x,\n    ],\n]";
        assert_captures(
            src,
            "rust",
            &[
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("x", Some("variable")),
                (",", Some("punctuation.delimiter")),
                ("]", Some("punctuation.bracket")),
                (",", Some("punctuation.delimiter")),
                ("]", Some("punctuation.bracket")),
            ],
        );
    }

    /// Image #21 reproduction using the EXACT contents of the
    /// `button-size.md` snippet that the browser feeds the highlighter
    /// (everything between the ``` fences). If any inner `row!` reads
    /// as fg/white in the rendered browser but `function.macro` here,
    /// the bug is downstream of the highlighter (markdown extraction,
    /// viewer wiring, etc.).
    #[test]
    fn image_21_button_size_snippet_literal() {
        let src = "\
use icedxdy::widget::{button, row};
use icedxdy::Size::*;

row![
    row![
        button::outline(\"Extra Small\").size(Xs),
        button::icon(icon::arrow_up_right()).size(Xs),
    ],
    row![
        button::outline(\"Small\").size(Sm),
        button::icon(icon::arrow_up_right()).size(Sm),
    ],
    row![
        button::outline(\"Default\"),
        button::icon(icon::arrow_up_right()),
    ],
    row![
        button::outline(\"Large\").size(Lg),
        button::icon(icon::arrow_up_right()).size(Lg),
    ],
]";
        let stream = capture_stream(src, "rust");
        let row_caps: Vec<_> = stream
            .iter()
            .filter(|(t, _)| t == "row")
            .map(|(_, c)| *c)
            .collect();
        assert_eq!(
            row_caps,
            vec![
                Some("variable"),       // `row` in `use {button, row}` — use_list override
                Some("function.macro"), // outer  row![…]
                Some("function.macro"), // inner #1
                Some("function.macro"), // inner #2
                Some("function.macro"), // inner #3
                Some("function.macro"), // inner #4
            ],
            "row capture sequence wrong",
        );
    }

    /// Image #21 reproduction: outer `row!` containing multiple
    /// sibling `row!` blocks, each with a body of `path::call(arg)`
    /// chains. Pin every capture so we can tell at a glance whether
    /// any nested-macro identifier silently falls through to fg.
    #[test]
    fn image_21_nested_button_calls() {
        let src = "row![\n    row![\n        button::outline(\"X\").size(Xs),\n    ],\n]";
        assert_captures(
            src,
            "rust",
            &[
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("button", Some("namespace")),
                ("::", Some("punctuation.delimiter")),
                ("outline", Some("function")),
                ("(", Some("punctuation.bracket")),
                ("\"X\"", Some("string")),
                (")", Some("punctuation.bracket")),
                (".", Some("punctuation.delimiter")),
                ("size", Some("function")),
                ("(", Some("punctuation.bracket")),
                ("Xs", Some("type")),
                (")", Some("punctuation.bracket")),
                (",", Some("punctuation.delimiter")),
                ("]", Some("punctuation.bracket")),
                (",", Some("punctuation.delimiter")),
                ("]", Some("punctuation.bracket")),
            ],
        );
    }

    /// Multiple SIBLING nested macros inside one outer macro — matches
    /// the structure of example bodies like `row![ row![…], row![…],
    /// row![…] ]`. Each sibling `row!` must independently fire the
    /// macro pattern. Regression check for image #21 in the
    /// conversation history.
    #[test]
    fn sibling_nested_macros() {
        let src = "row![row![a],row![b]]";
        assert_captures(
            src,
            "rust",
            &[
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("a", Some("variable")),
                ("]", Some("punctuation.bracket")),
                (",", Some("punctuation.delimiter")),
                ("row", Some("function.macro")),
                ("!", Some("function.macro")),
                ("[", Some("punctuation.bracket")),
                ("b", Some("variable")),
                ("]", Some("punctuation.bracket")),
                ("]", Some("punctuation.bracket")),
            ],
        );
    }

    /// Use clauses: `use` is a control keyword, every path component is a
    /// namespace, the brace-grouped imports are also namespaces. This
    /// matches what Tokyo Night colours in VS Code (path components in
    /// cyan, `use` keyword in purple, braces/`::` in light cyan).
    #[test]
    fn use_clause_namespace_chain() {
        let src = "use icedxdy::widget::{button, row};";
        assert_captures(
            src,
            "rust",
            &[
                ("use", Some("keyword.control.import")),
                ("icedxdy", Some("namespace")),
                ("::", Some("punctuation.delimiter")),
                ("widget", Some("namespace")),
                ("::", Some("punctuation.delimiter")),
                ("{", Some("punctuation.bracket")),
                ("button", Some("variable")),
                (",", Some("punctuation.delimiter")),
                ("row", Some("variable")),
                ("}", Some("punctuation.bracket")),
                (";", Some("punctuation.delimiter")),
            ],
        );
    }
}
