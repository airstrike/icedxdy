//! Preview-02 receiving-method card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/receiving-method.tsx`.

use iced::Fill;
use iced::Widget;
use icedxdy::widget::{button, column, field, input, radio, text};
use icedxdy::{Slot, Theme, card};

use super::api_gap;
use crate::{Message, ReceivingMethod, State};

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let account = field::field(
        input("")
            .value(&state.receiving_account_holder)
            .on_input(Message::ReceivingAccountHolderChanged)
            .width(Fill),
    )
    .label("Account Holder Name")
    .width(Fill);

    let options = column![
        method_card(
            state,
            ReceivingMethod::BankTransfer,
            "Bank Transfer",
            "SWIFT / IBAN",
        ),
        method_card(state, ReceivingMethod::Paypal, "PayPal", "Instant Payout",),
    ]
    .spacing(12)
    .width(Fill);

    let routing = field::field(
        input("")
            .value(&state.receiving_routing)
            .on_input(Message::ReceivingRoutingChanged)
            .width(Fill),
    )
    .label("IBAN / Account Number")
    .width(Fill);

    card::card(
        column![
            card::header("Receiving Method").action(api_gap::marker()),
            card::content(
                column![
                    account,
                    column![text::p("Receiving Method").medium(), options,]
                        .spacing(10)
                        .width(Fill),
                    routing,
                ]
                .spacing(16)
                .width(Fill)
            ),
            card::footer(
                button("Save Payout Settings")
                    .width(Fill)
                    .on_press(Message::Noop)
            ),
        ]
        .spacing(16)
        .padding(iced::Padding::ZERO.top(16.0))
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}

fn method_card(
    state: &State,
    method: ReceivingMethod,
    title: &'static str,
    description: &'static str,
) -> Slot<'static, Message> {
    let selected = state.receiving_method == method;
    let control = radio(
        "",
        method,
        Some(state.receiving_method),
        Message::SetReceivingMethod,
    )
    .spacing(0.0);
    let label: Slot<'static, Message> = column![
        text::small(title).medium(),
        text::muted(description).size(13.0),
    ]
    .spacing(2)
    .into();

    field::field(control)
        .orientation(field::Horizontal)
        .label(label)
        .card(selected)
        .on_press(Message::SetReceivingMethod(method))
        .width(Fill)
        .into()
}
