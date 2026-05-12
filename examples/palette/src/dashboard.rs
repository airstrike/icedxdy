//! Main dashboard area — rows of tile cards.
//!
//! Tiles are laid out in a flexible grid: 2-column rows where each tile
//! claims a minimum width. The root scrollable opts in to horizontal
//! overflow, so narrow windows scroll rather than wrap — matching shadcn
//! `/create`'s dense single-page feel.

use iced::widget::space;
use iced::{Fill, Pixels};
use icedxdy::theme;
use icedxdy::widget::{breadcrumb, column, container, row, separator, text};
use icedxdy::{Component, Theme};

use crate::tiles::{
    alerts, connection, danger, form, notifications, quickstart, targets, tints, transactions,
};
use crate::{Message, State};

const TILE_MIN_WIDTH: f32 = 360.0;
const MAIN_PAD: f32 = 24.0;
const GAP: f32 = 24.0;

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let breadcrumbs = breadcrumb([
        breadcrumb::link("Workspace", Message::Noop),
        breadcrumb::link("Demos", Message::Noop),
        breadcrumb::page("Overview"),
    ]);

    let title = text::h1("Overview");
    let subtitle = text::p(
        "Every widget in icedxdy/widget exercised exactly once, with tints used \
         thematically rather than as standalone swatches.",
    )
    .style(theme::text::Muted)
    .width(Fill);

    let header = column![breadcrumbs, title, subtitle].spacing(8);

    let divider = separator::horizontal();
    let content = column![header, divider, grid(state)]
        .spacing(GAP)
        .padding(MAIN_PAD)
        .width(Fill);

    container(content).width(Fill).height(Fill).into()
}

fn grid(state: &State) -> Component<'_, Message> {
    column([
        row![wrap(quickstart::view(state)), wrap(connection::view(state)),]
            .spacing(GAP)
            .width(Fill)
            .into(),
        row![wrap(transactions::view(state)), wrap(targets::view(state)),]
            .spacing(GAP)
            .width(Fill)
            .into(),
        row![wrap(alerts::view()), wrap(notifications::view(state)),]
            .spacing(GAP)
            .width(Fill)
            .into(),
        row![wrap(form::view(state)), wrap(danger::view(state)),]
            .spacing(GAP)
            .width(Fill)
            .into(),
        row![wrap(tints::view(state))]
            .spacing(GAP)
            .width(Fill)
            .into(),
        // Trailing space gives the bottom edge of the last row room to
        // breathe under the horizontal scrollbar.
        Component::new(space::vertical()),
    ])
    .spacing(GAP)
    .width(Fill)
    .into()
}

/// Stretch a tile to share its row's width while keeping a floor at
/// `TILE_MIN_WIDTH`. The minimum is what makes the horizontal scroll
/// affordance meaningful — without it, a single super-narrow row would
/// just squash all tiles instead of overflowing.
///
/// Takes `iced::Element` because tile views mix 'static and 'a-borrowed
/// returns; `Component` is invariant over its lifetime so it can't span
/// the gap, but `iced::Element` is covariant and lifts cleanly.
fn wrap<'a>(content: iced::Element<'a, Message, Theme>) -> Component<'a, Message> {
    let wrapped: iced::Element<'a, Message, Theme> = container(content)
        .width(Fill)
        .min_width(Pixels(TILE_MIN_WIDTH))
        .into();
    Component::from(wrapped)
}
