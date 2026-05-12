//! Main dashboard area — rows of tile cards.
//!
//! Tiles are laid out in a flexible grid: 2-column rows where each tile
//! claims a minimum width. The root scrollable opts in to horizontal
//! overflow, so narrow windows scroll rather than wrap — matching shadcn
//! `/create`'s dense single-page feel.

use iced::widget::space;
use iced::{Fill, Pixels};
use icedxdy::widget::{column, container, row};
use icedxdy::{Component, Theme};

use crate::tiles::{
    alerts, danger, form, notifications, quickstart, targets, transactions, two_factor,
};
use crate::{Message, State};

const TILE_MIN_WIDTH: f32 = 360.0;
const MAIN_PAD: f32 = 24.0;
const GAP: f32 = 24.0;

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let content = column![grid(state)]
        .spacing(GAP)
        .padding(MAIN_PAD)
        .width(Fill);

    container(content).width(Fill).height(Fill).into()
}

fn grid(state: &State) -> Component<'_, Message> {
    column([
        row![wrap(quickstart::view(state)), wrap(two_factor::view(state))]
            .spacing(GAP)
            .width(Fill)
            .into(),
        row![wrap(transactions::view(state)), wrap(notifications::view(state))]
            .spacing(GAP)
            .width(Fill)
            .into(),
        row![wrap(form::view(state)), wrap(alerts::view())]
            .spacing(GAP)
            .width(Fill)
            .into(),
        row![wrap(targets::view(state)), wrap(danger::view(state))]
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
