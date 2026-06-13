//! Main dashboard area — `/create`-style masonry columns.
//!
//! shadcn's preview iframe uses fixed-width bento columns inside a
//! horizontally-scrollable surface. Matching that geometry first makes the
//! remaining per-card fidelity work much easier to see.

use iced::{Alignment, Fill};
use icedxdy::widget::{Column, container, row};
use icedxdy::{Component, Theme};

use crate::tiles::{
    account_access, banner, card_overview, chart, claimable_balance, codespaces,
    contribution_history, contributions, cover_art, dividend_income, empty_connect_bank,
    empty_distribute, empty_explore_catalog, env_vars, faq, form, front_door, index_investing,
    invite_team, kitchen_island, loading_card, new_milestone, notification_settings, payments,
    payout_threshold, power_usage, preferences, qr_connect, quickstart, receiving_method,
    recent_transactions, release_catalog, roller_shades, savings_progress, savings_targets,
    showcase, sidebar_nav, social_links, stock_performance, syncing_state, targets, transactions,
    transfer_funds, two_factor, upcoming_payments,
};
use crate::{Message, PreviewPage, State};

const COLUMN_WIDTH: f32 = 380.0;
const WIDE_COLUMN_WIDTH: f32 = COLUMN_WIDTH * 2.0 + COLUMN_GAP;
const MAIN_PAD_X: f32 = 40.0;
const MAIN_PAD_Y: f32 = 40.0;
const COLUMN_GAP: f32 = 40.0;
const ROW_GAP: f32 = 40.0;

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    match state.preview_page {
        PreviewPage::One => view_one(state),
        PreviewPage::Two => view_two(state),
    }
}

fn view_one(state: &State) -> iced::Element<'_, Message, Theme> {
    let columns = row![
        stack([
            contribution_history::view(state),
            empty_distribute::view(state),
            qr_connect::view(state),
            dividend_income::view(state),
            index_investing::view(state),
            // FIXME(spinner-cpu): perpetual spinner() (via empty::media_icon)
            // request_redraw()s every frame → continuous whole-window repaint
            // at idle.
            syncing_state::view(state),
        ]),
        stack([
            payout_threshold::view(state),
            claimable_balance::view(state),
            preferences::view(state),
            savings_progress::view(state),
            kitchen_island::view(state),
        ]),
        stack_width(
            WIDE_COLUMN_WIDTH,
            [
                savings_targets::view(state),
                recent_transactions::view(state),
                nested_middle(state),
                release_catalog::view(state),
            ]
        ),
        stack([
            account_access::view(state),
            card_overview::view(state),
            transfer_funds::view(state),
            cover_art::view(state),
            loading_card::view(state),
        ]),
        stack([
            receiving_method::view(state),
            power_usage::view(state),
            empty_connect_bank::view(state),
            upcoming_payments::view(state),
            roller_shades::view(state),
        ]),
        stack([
            stock_performance::view(state),
            empty_explore_catalog::view(state),
            new_milestone::view(state),
            social_links::view(state),
            notification_settings::view(state),
        ]),
    ]
    .spacing(COLUMN_GAP)
    .align_y(Alignment::Start);

    let content = container(columns).padding([MAIN_PAD_Y, MAIN_PAD_X]);

    container(content).width(Fill).height(Fill).into()
}

fn nested_middle(state: &State) -> iced::Element<'_, Message, Theme> {
    let columns = row![
        stack([sidebar_nav::view(state), faq::view(state),]),
        stack([payments::view(state), front_door::view(state),]),
    ]
    .spacing(COLUMN_GAP)
    .align_y(Alignment::Start);

    container(columns).width(WIDE_COLUMN_WIDTH).into()
}

fn view_two(state: &State) -> iced::Element<'_, Message, Theme> {
    let columns = row![
        stack([
            showcase::theme_card(),
            showcase::typography_card(),
            codespaces::view(state),
        ]),
        stack([
            showcase::icon_grid(),
            showcase::controls_card(state),
            banner::view(state),
        ]),
        stack([
            env_vars::view(state),
            chart::view(state),
            invite_team::view(state),
        ]),
        stack([
            showcase::skeleton_card(),
            transactions::view(state),
            targets::view(state),
            form::view(state),
        ]),
        stack([
            contributions::view(state),
            quickstart::view(state),
            two_factor::view(state),
        ]),
    ]
    .spacing(COLUMN_GAP)
    .align_y(Alignment::Start);

    let content = container(columns).padding([MAIN_PAD_Y, MAIN_PAD_X]);

    container(content).width(Fill).height(Fill).into()
}

fn stack<'a>(
    tiles: impl IntoIterator<Item = iced::Element<'a, Message, Theme>>,
) -> Component<'a, Message> {
    stack_width(COLUMN_WIDTH, tiles)
}

fn stack_width<'a>(
    width: f32,
    tiles: impl IntoIterator<Item = iced::Element<'a, Message, Theme>>,
) -> Component<'a, Message> {
    let column: Column<'a, Message> = Column::with_children(tiles.into_iter().map(Component::from))
        .spacing(ROW_GAP)
        .width(width);

    container(column).width(width).into()
}
