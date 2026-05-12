//! Top chrome bar — brand · nav · search · auth.

use iced::widget::space;
use iced::{Center, Fill};
use icedxdy::theme::container::background;
use icedxdy::widget::input::{Input, input};
use icedxdy::widget::row::Row;
use icedxdy::widget::{Component, button, container, row, text};
use icedxdy::{Theme, theme};

use crate::{Message, Mode, State};

const NAV_LABELS: &[&str] = &[
    "Docs",
    "Components",
    "Blocks",
    "Charts",
    "Directory",
    "Themes",
];

const SEARCH_MAX_WIDTH: f32 = 420.0;

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let brand = text::large("icedxdy").style(theme::text::Foreground);

    let nav_items: Vec<Component<'_, Message>> = NAV_LABELS
        .iter()
        .map(|label| button::ghost(*label).on_press(Message::Noop).into())
        .collect();
    let nav = Row::with_children(nav_items).spacing(2).align_y(Center);

    let search_input: Input<'_, Message> = input("Search documentation...", "").width(Fill);
    let search = container(search_input)
        .max_width(SEARCH_MAX_WIDTH)
        .width(Fill);

    let mode_button = match state.mode {
        Mode::Light => button("Dark").on_press(Message::SetMode(Mode::Dark)),
        Mode::Dark => button("Light").on_press(Message::SetMode(Mode::Light)),
    };

    let auth = row![
        mode_button,
        button::ghost("Sign In").on_press(Message::Noop),
        button("Get for Free").on_press(Message::Noop),
    ]
    .spacing(8)
    .align_y(Center);

    let bar = row![
        brand,
        nav,
        space::horizontal(),
        search,
        space::horizontal(),
        auth,
    ]
    .spacing(16)
    .align_y(Center)
    .width(Fill);

    container(bar)
        .padding([12, 24])
        .width(Fill)
        .style(background)
        .into()
}
