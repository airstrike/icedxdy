//! Form tile — input + checkbox + switch wrapped in fields, plus
//! a primary submit button and a secondary cancel.
//!
//! The form is a user `Component`: its values are its own state, edited
//! through a private [`Event`], and only the submission reaches the app.
//! Pressing a field's label still toggles its checkbox or switch, because
//! the label links to its control through operations, which cross the
//! component boundary.
//!
//! Exercises: field, input (text_input), checkbox, switch, button
//! (primary / secondary), text variants (label / muted / error), row,
//! column, container.

use iced::Fill;
use iced::Widget;
use iced::widget::component::{Component, component};
use icedxdy::Slot;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::theme;
use icedxdy::widget::{Column, button, checkbox, column, field, input, row, switch, text};

use crate::{Message, State};

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    card::card(
        column([
            card::header("Account settings")
                .description("Demonstrates every form widget in one tile.")
                .into(),
            card::content(
                component(Form {
                    connected: state.connected(),
                })
                .boxed(),
            )
            .into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}

/// The account form. `connected` comes from the app: the form submits only
/// while the app is online.
struct Form {
    connected: bool,
}

/// What the form holds while it is edited.
struct Values {
    email: String,
    name: String,
    accept_terms: bool,
    weekly_digest: bool,
}

impl Default for Values {
    fn default() -> Self {
        Self {
            email: "ada@icedxdy.dev".to_owned(),
            name: "Ada Lovelace".to_owned(),
            accept_terms: true,
            weekly_digest: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Event {
    Email(String),
    Name(String),
    AcceptTerms(bool),
    WeeklyDigest(bool),
    Save,
    Cancel,
}

impl<'a> Component<'a, Message, Theme> for Form {
    type State = Values;
    type Event = Event;

    fn update(
        &self,
        values: &mut Values,
        event: Event,
        _renderer: &iced::Renderer,
    ) -> Option<Message> {
        match event {
            Event::Email(email) => values.email = email,
            Event::Name(name) => values.name = name,
            Event::AcceptTerms(accept) => values.accept_terms = accept,
            Event::WeeklyDigest(digest) => values.weekly_digest = digest,
            Event::Save => return Some(Message::SaveForm),
            Event::Cancel => return Some(Message::CancelForm),
        }

        None
    }

    fn view(&self, values: &Values) -> impl Widget<Event, Theme> + 'a {
        let email_invalid = !values.email.contains('@');

        let mut email_field = field(
            input("you@example.com")
                .value(values.email.clone())
                .on_input(Event::Email)
                .on_submit(Event::Save)
                .width(Fill),
        )
        .label("Email")
        .description("We use this to confirm your subscription.")
        .width(Fill);

        if email_invalid {
            email_field = email_field.error("Email needs an @ to be valid.");
        }

        let name_field = field(
            input("Display name")
                .value(values.name.clone())
                .on_input(Event::Name)
                .on_submit(Event::Save)
                .width(Fill),
        )
        .label("Name")
        .description("Shown in transactions and notifications.")
        .width(Fill);

        let terms = field(
            checkbox()
                .checked(values.accept_terms)
                .on_checked_change(Event::AcceptTerms),
        )
        .label("Accept terms")
        .description("Required before we can save your preferences.");

        let emails = field(
            switch::switch()
                .checked(values.weekly_digest)
                .on_checked_change(Event::WeeklyDigest),
        )
        .label("Weekly digest")
        .description("Sundays at 6pm UTC. Unsubscribe in one click.");

        let can_save = self.connected && values.accept_terms && !email_invalid;

        let footer = row![
            button("Save preferences")
                .width(Fill)
                .on_press(Event::Save)
                .disabled(!can_save),
            button("Cancel")
                .style(button::Secondary)
                .width(Fill)
                .on_press(Event::Cancel)
                .disabled(!self.connected),
        ]
        .spacing(12)
        .width(Fill);

        let footnote = text::xs(
            "The form owns its values; Save publishes SaveForm to the app only \
             once the terms are accepted and the email is valid.",
        )
        .style(theme::text::Muted);

        let body: Column<'a, Event> = column([
            Slot::from(email_field),
            Slot::from(name_field),
            Slot::from(terms),
            Slot::from(emails),
            Slot::from(footer),
            footnote.into(),
        ])
        .spacing(16)
        .width(Fill);

        iced::Element::from(body)
    }
}
