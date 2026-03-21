// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// 669aea4b3e49e7d9875a2ade5428145bf72aac5d143daf75f4dbee33e1e0c417
use iced::widget::text::{self, Text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("chevron_right", "\u{E06F}"),
    ("loader_circle", "\u{E10A}"),
    ("mail", "\u{E10F}"),
    ("moon", "\u{E11E}"),
    ("sun", "\u{E178}"),
];

pub fn chevron_right<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06F}")
}

pub fn loader_circle<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E10A}")
}

pub fn mail<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E10F}")
}

pub fn moon<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E11E}")
}

pub fn sun<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E178}")
}

/// Render any Lucide icon by its codepoint string.
/// Use this together with [`ALL_ICONS`] to display icons dynamically:
/// ```ignore
/// for (name, cp) in ALL_ICONS {
///     button(render(cp)).on_press(Msg::Pick(name.to_string()))
/// }
/// ```
pub fn render<'a, Theme>(codepoint: &'a str) -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    Text::new(codepoint).font("lucide")
}

fn icon<'a, Theme>(codepoint: &'a str) -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    render(codepoint)
}
