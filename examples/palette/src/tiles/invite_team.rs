//! Invite Team — email + role picker + Invite, plus a small members list.

use iced::Fill;
use iced::Widget;
use iced::widget::space;
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::column::Column;
use icedxdy::widget::{avatar, badge, button, column, container, input, row, text};
use icedxdy::{Slot, Theme};

use crate::{Message, State};

struct Member {
    initials: &'static str,
    name: &'static str,
    email: &'static str,
    role: &'static str,
}

const MEMBERS: &[Member] = &[
    Member {
        initials: "AL",
        name: "Ada Lovelace",
        email: "ada@icedxdy.dev",
        role: "Admin",
    },
    Member {
        initials: "GH",
        name: "Grace Hopper",
        email: "grace@icedxdy.dev",
        role: "Editor",
    },
];

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let invite_row = row![
        input("email@example.com")
            .value(&state.invite_email)
            .width(Fill)
            .on_input(Message::InviteEmailChanged)
            .on_submit(Message::Noop),
        button("Viewer  \u{25BE}")
            .style(button::Outline)
            .on_press(Message::Noop),
        button("Invite").on_press(Message::Noop),
    ]
    .spacing(8)
    .width(Fill);

    let members: Vec<Slot<'_, Message>> = MEMBERS.iter().map(member_row).collect();

    let body: Column<'_, Message> = column(std::iter::once(Slot::from(invite_row)).chain(members))
        .spacing(12)
        .width(Fill);

    card::card(
        column([
            card::header("Invite Team")
                .description("Add members to your workspace")
                .into(),
            card::content(container(body).padding([0, 16]).width(Fill)).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .height(312.0)
    .boxed()
}

fn member_row(m: &Member) -> Slot<'static, Message> {
    let identity: Column<'static, Message> = column([
        text::small(m.name).medium().into(),
        text::xs(m.email).style(theme::text::Muted).into(),
    ])
    .spacing(2);

    row![
        Slot::from(avatar(None).fallback(m.initials).size(avatar::Default)),
        identity,
        space::horizontal(),
        badge(m.role).style(badge::Secondary),
    ]
    .spacing(12)
    .width(Fill)
    .into()
}
