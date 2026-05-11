//! Form tile — input + checkbox + switch wrapped in fields, plus
//! a primary submit button and a secondary cancel.
//!
//! Exercises: field, input (text_input), checkbox, switch, button
//! (primary / secondary), text variants (label / muted / error), row,
//! column, container.

use iced::Fill;
use icedxdy::Component;
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::{Column, button, checkbox, column, field, input, row, switch, text};
use icedxdy::Theme;

use crate::{Message, State};

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let email_invalid = !state.email.contains('@');

    let mut email_field = field(
        input("you@example.com", &state.email)
            .on_input(Message::EmailChanged)
            .width(Fill),
    )
    .label("Email")
    .description("We use this to confirm your subscription.")
    .width(Fill);

    if email_invalid {
        email_field = email_field.error("Email needs an @ to be valid.");
    }

    let name_field = field(
        input("Display name", &state.name)
            .on_input(Message::NameChanged)
            .width(Fill),
    )
    .label("Name")
    .description("Shown in transactions and notifications.")
    .width(Fill);

    let terms = field(
        checkbox(state.accept_terms).on_toggle(Message::AcceptTerms),
    )
    .label("Accept terms")
    .description("Required before we can save your preferences.");

    let emails = field(
        switch::switch(state.subscribe_emails).on_toggle(Message::SubscribeEmails),
    )
    .label("Weekly digest")
    .description("Sundays at 6pm UTC. Unsubscribe in one click.");

    let primary_cta = {
        let mut b = button("Save preferences").width(Fill);
        if state.connected {
            b = b.on_press(Message::SaveForm);
        }
        b
    };

    let secondary_cta = {
        let mut b = button::secondary("Cancel").width(Fill);
        if state.connected {
            b = b.on_press(Message::CancelForm);
        }
        b
    };

    let footer = row![primary_cta, secondary_cta].spacing(12).width(Fill);

    let footnote = text::xs(
        "Submit fires SaveForm; secondary fires CancelForm. Both noop in update — \
         the wiring is the demo.",
    )
    .style(theme::text::Muted);

    let body: Column<'_, Message> = column([
        Component::from(email_field),
        Component::from(name_field),
        Component::from(terms),
        Component::from(emails),
        Component::from(footer),
        footnote.into(),
    ])
    .spacing(16)
    .width(Fill);

    card::card(
        column([
            card::header("Account settings")
                .description("Demonstrates every form widget in one tile.")
                .into(),
            card::content(body).into(),
        ])
        .spacing(16)
        .padding([16, 0]),
    )
    .width(Fill)
    .into()
}
