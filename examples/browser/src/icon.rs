// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// 2fb3b3d9bbe7ca07089df6d4960b3470adf4288830c35cb7f12fde16805556be
use iced::widget::text::{self, Text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("arrow_left", "\u{E048}"),
    ("arrow_right", "\u{E049}"),
    ("arrow_up", "\u{E04A}"),
    ("arrow_up_right", "\u{E04D}"),
    ("badge_check", "\u{E241}"),
    ("bookmark", "\u{E060}"),
    ("check", "\u{E06C}"),
    ("chevron_down", "\u{E06D}"),
    ("chevron_right", "\u{E06F}"),
    ("chevron_up", "\u{E070}"),
    ("chevrons_up_down", "\u{E211}"),
    ("circle_alert", "\u{E077}"),
    ("circle_check", "\u{E226}"),
    ("circle_check_big", "\u{E07C}"),
    ("copy", "\u{E09E}"),
    ("ellipsis", "\u{E0B6}"),
    ("file", "\u{E0C0}"),
    ("folder", "\u{E0D7}"),
    ("git_branch", "\u{E0E2}"),
    ("info", "\u{E0F9}"),
    ("minus", "\u{E11C}"),
    ("moon", "\u{E11E}"),
    ("plus", "\u{E13D}"),
    ("save", "\u{E14D}"),
    ("sun", "\u{E178}"),
    ("terminal", "\u{E181}"),
    ("x", "\u{E1B2}"),
];

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

pub fn badge_check<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E241}")
}

pub fn bookmark<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E060}")
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

pub fn chevrons_up_down<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E211}")
}

pub fn circle_alert<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E077}")
}

pub fn circle_check<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E226}")
}

pub fn circle_check_big<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E07C}")
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

pub fn file<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0C0}")
}

pub fn folder<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0D7}")
}

pub fn git_branch<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0E2}")
}

pub fn info<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0F9}")
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

pub fn plus<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E13D}")
}

pub fn save<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E14D}")
}

pub fn sun<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E178}")
}

pub fn terminal<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E181}")
}

pub fn x<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E1B2}")
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
