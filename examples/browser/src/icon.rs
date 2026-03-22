// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// 371a4414b14e4d6bf83c440625752030955e172a13bc754345d43c2d9bffc85e
use iced::widget::text::{self, Text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("arrow_up", "\u{E04A}"),
    ("arrow_up_right", "\u{E04D}"),
    ("chevron_right", "\u{E06F}"),
    ("git_branch", "\u{E0E2}"),
    ("moon", "\u{E11E}"),
    ("sun", "\u{E178}"),
];

pub fn arrow_up<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E04A}")
}

pub fn arrow_up_right<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E04D}")
}

pub fn chevron_right<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06F}")
}

pub fn git_branch<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0E2}")
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
