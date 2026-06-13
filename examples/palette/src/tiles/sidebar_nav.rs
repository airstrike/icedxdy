//! Preview-02 sidebar-nav card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/sidebar-nav.tsx`.

use iced::Fill;
use icedxdy::widget::{column, row, sidebar, text};
use icedxdy::{Component, Theme, card};

use crate::{Message, State, icon};

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(_state: &State) -> iced::Element<'static, Message, Theme> {
    row![
        nav_card([
            group(
                "Overview",
                [
                    nav_item("Dashboard", icon::square().size(16).into(), true),
                    nav_item("Transactions", icon::arrow_right().size(16).into(), false),
                    nav_item("Investments", icon::chevron_up().size(16).into(), false),
                ],
            ),
            sidebar::separator().into(),
            group(
                "Planning",
                [
                    nav_item("Goals", icon::circle().size(16).into(), false),
                    nav_item("Budget", icon::square().size(16).into(), false),
                    nav_item("Reports", icon::menu().size(16).into(), false),
                    nav_item("Documents", icon::archive().size(16).into(), false),
                ],
            ),
        ]),
        nav_card([
            group(
                "Account",
                [
                    nav_item("Profile", icon::circle().size(16).into(), false),
                    nav_item("Billing", icon::square().size(16).into(), true),
                    nav_item("Notifications", icon::circle_help().size(16).into(), false),
                    nav_item("Security", icon::check().size(16).into(), false),
                ],
            ),
            sidebar::separator().into(),
            group(
                "Support",
                [
                    nav_item("Help Center", icon::circle_help().size(16).into(), false),
                    nav_item("Contact Us", icon::ellipsis().size(16).into(), false),
                    nav_item("Status", icon::refresh_cw().size(16).into(), false),
                ],
            ),
        ]),
    ]
    .spacing(24)
    .width(Fill)
    .into()
}

fn nav_card<const N: usize>(
    children: [Component<'static, Message>; N],
) -> Component<'static, Message> {
    card::card(column(children).spacing(0).width(Fill))
        .width(Fill)
        .into()
}

fn group<const N: usize>(
    label: &'static str,
    items: [Component<'static, Message>; N],
) -> Component<'static, Message> {
    sidebar::group([
        sidebar::group_label(label).into(),
        sidebar::group_content(items).into(),
    ])
    .into()
}

fn nav_item(
    label: &'static str,
    icon: Component<'static, Message>,
    active: bool,
) -> Component<'static, Message> {
    let content = row![icon, text::small(label)].spacing(8).width(Fill);
    let button = sidebar::menu_button(content)
        .active(active)
        .on_press(Message::Noop);

    sidebar::menu_item([button.into()]).into()
}
