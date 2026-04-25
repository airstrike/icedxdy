// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// 1c849340784a7f1b8e43181cee40d14bea6140b803c91377d7d180b8b952e8c6
use iced::widget::text::{self, Text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("arrow_up", "\u{E04A}"),
    ("arrow_up_right", "\u{E04D}"),
    ("badge_check", "\u{E241}"),
    ("bookmark", "\u{E060}"),
    ("check", "\u{E06C}"),
    ("circle_alert", "\u{E077}"),
    ("circle_check", "\u{E226}"),
    ("circle_check_big", "\u{E07C}"),
    ("copy", "\u{E09E}"),
    ("git_branch", "\u{E0E2}"),
    ("info", "\u{E0F9}"),
    ("moon", "\u{E11E}"),
    ("sun", "\u{E178}"),
    ("terminal", "\u{E181}"),
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

pub fn terminal<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E181}")
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
