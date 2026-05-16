//! Layout probe — a throwaway `#[granita::preview]` for diagnosing the
//! `Length::Bounded` completion on the patched iced branch (the
//! `max_width` / `min_width` / `Fixed`-floor work). Not wired into the
//! component browser UI; delete once the layout fix lands on iced.

use iced::widget::{column, container, text};
use iced::{Color, Fill, Length, color};
use icedxdy::Theme;

use crate::Message;

fn red() -> Color {
    color!(0xdb3345)
}

fn blue() -> Color {
    color!(0x3375db)
}

/// A flat-colored box, so layout-resolved sizes are visible in a screenshot.
fn paint<'a>(
    bg: Color,
    child: impl Into<iced::Element<'a, Message, Theme>>,
) -> container::Container<'a, Message, Theme> {
    container(child).style(move |_: &Theme| container::Style {
        background: Some(bg.into()),
        ..container::Style::default()
    })
}

fn case<'a>(
    label: &'a str,
    demo: impl Into<iced::Element<'a, Message, Theme>>,
) -> iced::Element<'a, Message, Theme> {
    column![text(label).size(13), demo.into()].spacing(6).into()
}

/// Four layout cases that exercise the `Length::Bounded` work. Read the
/// rendered box sizes against the expectations in each comment.
#[granita::preview(group = "Layout probe")]
fn bounded_layout() -> iced::Element<'static, Message, Theme> {
    // A — a Fixed(60x40) box inside a Fixed(200x120) box.
    //     Correct: blue stays 60x40, parked top-left of red.
    //     Bug:     blue stretches to 60x120 — Fixed floored up to the
    //              parent's inherited `min`.
    let a = case(
        "A  Fixed 60x40 in Fixed 200x120  -> inner must stay 60x40",
        paint(red(), paint(blue(), text("")).width(60.0).height(40.0))
            .width(200.0)
            .height(120.0),
    );

    // B — a Fill-width box inside a max_width(150) box.
    //     Correct: red + blue cap at 150 wide.
    let b = case(
        "B  max_width(150), Fill inner  -> must cap at 150 wide",
        paint(red(), paint(blue(), text("")).width(Fill).height(40.0))
            .max_width(150.0)
            .height(60.0),
    );

    // C — a Shrink box inside a min_width(200) box.
    //     Correct: red floors at 200 wide.
    let c = case(
        "C  min_width(200), Shrink inner  -> must floor at 200 wide",
        paint(red(), paint(blue(), text("x")).padding(8))
            .width(Length::Shrink.min(200.0))
            .height(60.0),
    );

    // D — a Fill-height box inside a Fixed(200x120) box (control).
    //     Correct: blue is 80x120.
    let d = case(
        "D  Fill-height inner in Fixed 200x120  -> inner must be 80x120",
        paint(red(), paint(blue(), text("")).width(80.0).height(Fill))
            .width(200.0)
            .height(120.0),
    );

    container(column![a, b, c, d].spacing(24))
        .padding(20)
        .into()
}
