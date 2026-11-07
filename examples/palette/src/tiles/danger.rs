//! Danger zone tile — armed-delete pattern. The button switches between
//! outlined-destructive (idle) and filled-destructive (armed) via the
//! style API's `outline + destructive` vs bare `destructive` swap.
//!
//! Exercises: button (style API: outline + destructive / bare destructive),
//! field (horizontal-card with active border tracking armed state),
//! checkbox, text variants (small / muted / inline_code), container,
//! row, column.

use iced::Fill;
use iced::Widget;
use icedxdy::Slot;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::core::style::{destructive, outline};
use icedxdy::theme;
use icedxdy::theme::container::code_block;
use icedxdy::widget::{button, checkbox, column, container, field, row, space, text};

use crate::{Message, State};

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    // The `armed` branch resolves to a bare `destructive` Tint; the
    // idle branch resolves to `outline + destructive`. Both flow through
    // the new style API's `From<...> for iced_widget::button::StyleFn`
    // bridge, so the button's `.style(...)` accepts either.
    let delete_button = if state.armed {
        let mut b = button("Confirm delete").style(destructive).width(180.0);
        if state.connected {
            b = b.on_press(Message::Delete);
        }
        b
    } else {
        let mut b = button("Delete account")
            .style(outline + destructive)
            .width(180.0);
        if state.connected {
            b = b.on_press(Message::ArmDelete);
        }
        b
    };

    let armed_check = field(checkbox(state.armed).on_toggle(|_| Message::ArmDelete))
        .label("Acknowledge this is irreversible")
        .description("Required before the delete fires.");

    // shadcn's `<code>` chip: bg-muted + small padding + radius. The
    // `text::inline_code` helper sets only the font/size/weight; the
    // container chrome lives at the call site per the helper's docs.
    let code_chip = container(text::inline_code("delete --hard"))
        .padding([2, 6])
        .style(code_block);

    let footnote = row![
        Slot::from(code_chip),
        text::xs("propagates across replicas in ~30s.").style(theme::text::Muted),
    ]
    .spacing(8)
    .width(Fill);

    let actions = row![space::horizontal(), delete_button,]
        .spacing(8)
        .width(Fill);

    let body = column([Slot::from(armed_check), footnote.into(), actions.into()])
        .spacing(12)
        .width(Fill);

    card::card(
        column([
            card::header("Danger zone")
                .description("Armed-delete swaps outline → filled destructive on the same button.")
                .into(),
            card::content(container(body).padding([0, 16]).width(Fill)).into(),
        ])
        .spacing(16)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}
