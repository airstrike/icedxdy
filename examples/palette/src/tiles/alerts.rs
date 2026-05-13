//! Alerts tile — four alert variants stacked, each exercising a tint's
//! thematic context (info / success / warning / destructive).
//!
//! Exercises: alert (default + destructive), text variants (small),
//! container, column, custom-styled containers for the info / success /
//! warning surfaces (the alert widget itself only carries the default and
//! destructive variants; the rest are thematic containers).

use iced::Fill;
use icedxdy::Component;
use icedxdy::card;
use icedxdy::core::{Border, Color, Palette};
use icedxdy::theme;
use icedxdy::widget::{Column, alert, column, container, text};
use icedxdy::{Theme, theme::container::Style};

use crate::Message;

pub fn view() -> iced::Element<'static, Message, Theme> {
    let info = tinted_alert(
        |t: &Theme| t.info,
        "Heads up",
        "icedxdy ships only the alert variants shadcn does — the rest come from container + tint.",
    );
    let success = tinted_alert(
        |t: &Theme| t.success,
        "Saved",
        "Your dashboard layout is autosaved to local state every keystroke.",
    );
    let warning = tinted_alert(
        |t: &Theme| t.warning,
        "Storage is 80% full",
        "Archive older sessions to reclaim space before exports start failing.",
    );

    let destructive = alert::destructive()
        .title("Payment failed")
        .description("We couldn't charge the card on file. Update billing to keep this account active.");

    let body: Column<'_, Message> = column([
        info,
        success,
        warning,
        Component::from(destructive),
    ])
    .spacing(12)
    .width(Fill);

    card::card(
        column([
            card::header("Alerts").description("Each row sits inside its own tint surface.").into(),
            card::content(body).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .into()
}

/// A small alert-shaped block painted in a tint that the stock alert
/// widget doesn't carry as a variant (info / success / warning).
/// Mirrors shadcn's `bg-{tint}/10 text-{tint}-foreground border-{tint}/20`
/// cascade by building the container style closure directly.
fn tinted_alert(
    getter: fn(&Theme) -> Palette,
    title: &'static str,
    description: &'static str,
) -> Component<'static, Message> {
    let body: Column<'static, Message> = column([
        text::small(title).medium().style(theme::text::Foreground).into(),
        text::small(description).style(theme::text::Muted).into(),
    ])
    .spacing(4)
    .width(Fill);

    container(body)
        .padding([12, 16])
        .width(Fill)
        .style(move |t: &Theme| {
            let p = getter(t);
            Style {
                background: Some(tint_fill(p).into()),
                text_color: Some(p.foreground),
                border: Border {
                    radius: icedxdy::radius::LG.into(),
                    width: 1.0,
                    color: tint_border(p),
                },
                ..Style::default()
            }
        })
        .into()
}

/// Tint surface fill: 10% of `base` in light, 20% in dark — matches
/// shadcn's `bg-{tint}/10 dark:bg-{tint}/20` for low-emphasis surfaces.
fn tint_fill(p: Palette) -> Color {
    if p.is_dark {
        p.base.alpha(0.20)
    } else {
        p.base.alpha(0.10)
    }
}

/// Tint surface border: 20% of `base` in light, 35% in dark.
fn tint_border(p: Palette) -> Color {
    if p.is_dark {
        p.base.alpha(0.35)
    } else {
        p.base.alpha(0.20)
    }
}
