// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// e980ec6d53f5f92dc55667a3086bca7a4cffcbc3ce111b96e7d3246f9359827f
use iced::widget::text::{self, Text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("archive", "\u{E041}"),
    ("arrow_left", "\u{E048}"),
    ("arrow_right", "\u{E049}"),
    ("check", "\u{E06C}"),
    ("chevron_down", "\u{E06D}"),
    ("chevron_right", "\u{E06F}"),
    ("chevron_up", "\u{E070}"),
    ("circle", "\u{E076}"),
    ("circle_help", "\u{E082}"),
    ("copy", "\u{E09E}"),
    ("ellipsis", "\u{E0B6}"),
    ("github", "\u{E0E6}"),
    ("menu", "\u{E115}"),
    ("minus", "\u{E11C}"),
    ("moon", "\u{E11E}"),
    ("paintbrush", "\u{E2E7}"),
    ("plus", "\u{E13D}"),
    ("radius", "\u{E52D}"),
    ("refresh_cw", "\u{E145}"),
    ("search", "\u{E151}"),
    ("settings", "\u{E154}"),
    ("square", "\u{E167}"),
    ("sun", "\u{E178}"),
    ("trash_2", "\u{E18E}"),
    ("upload", "\u{E19E}"),
];

pub fn archive<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E041}")
}

pub fn arrow_left<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E048}")
}

pub fn arrow_right<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E049}")
}

pub fn check<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06C}")
}

pub fn chevron_down<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06D}")
}

pub fn chevron_right<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06F}")
}

pub fn chevron_up<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E070}")
}

pub fn circle<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E076}")
}

pub fn circle_help<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E082}")
}

pub fn copy<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E09E}")
}

pub fn ellipsis<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0B6}")
}

pub fn github<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0E6}")
}

pub fn menu<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E115}")
}

pub fn minus<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E11C}")
}

pub fn moon<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E11E}")
}

pub fn paintbrush<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E2E7}")
}

pub fn plus<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E13D}")
}

pub fn radius<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E52D}")
}

pub fn refresh_cw<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E145}")
}

pub fn search<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E151}")
}

pub fn settings<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E154}")
}

pub fn square<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E167}")
}

pub fn sun<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E178}")
}

pub fn trash_2<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E18E}")
}

pub fn upload<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E19E}")
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
