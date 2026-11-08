//! End-to-end checks that markers, owners and the control operations link
//! widgets the way Radix's `htmlFor` and `asChild` triggers do.
//!
//! Children are fixed-size blocks clicked by position, so the checks do not
//! depend on text shaping in the headless renderer.

use iced_core::time::Instant;
use iced_core::{Element, Event, Point, window};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::operation::control;
use icedxdy_widget::{Slot, button, checkbox, collapsible, column, field, switch};

type Simulator<'a> = iced_test::Simulator<'a, Message, Theme, Renderer>;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Checked(bool),
    Open(bool),
    Pressed,
    Inner,
}

/// A fixed-size stand-in for a label or button content.
fn block<'a>(width: f32, height: f32) -> Slot<'a, Message> {
    Slot::from(Space::new().width(width).height(height))
}

fn click(ui: &mut Simulator<'_>, x: f32, y: f32) {
    ui.point_at(Point::new(x, y));
    let _ = ui.simulate(simulator::click());
    let _ = ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
}

fn is_checked(ui: &mut Simulator<'_>) -> Option<bool> {
    let mut checked = control::checked();
    ui.operate(&mut checked);
    checked.get()
}

/// A vertical field: a 120×20 label at the top, its control below.
fn labelled<'a>(control: impl Into<Slot<'a, Message>>) -> Element<'a, Message, Theme, Renderer> {
    field(control).label(block(120.0, 20.0)).into()
}

#[test]
fn a_field_label_toggles_its_checkbox() {
    let mut ui = simulator(labelled(checkbox().on_checked_change(Message::Checked)));
    assert_eq!(is_checked(&mut ui), Some(false));

    click(&mut ui, 10.0, 10.0);

    assert_eq!(is_checked(&mut ui), Some(true));
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Checked(true)]
    );
}

#[test]
fn an_uncontrolled_switch_keeps_its_value_across_label_presses() {
    let mut ui = simulator(labelled(
        switch()
            .default_checked(true)
            .on_checked_change(Message::Checked),
    ));

    click(&mut ui, 10.0, 10.0);
    assert_eq!(is_checked(&mut ui), Some(false));
    click(&mut ui, 10.0, 10.0);
    assert_eq!(is_checked(&mut ui), Some(true));

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Checked(false), Message::Checked(true)]
    );
}

#[test]
fn a_controlled_checkbox_publishes_without_changing() {
    let mut ui = simulator(labelled(
        checkbox().checked(true).on_checked_change(Message::Checked),
    ));

    click(&mut ui, 10.0, 10.0);
    click(&mut ui, 10.0, 10.0);

    assert_eq!(is_checked(&mut ui), Some(true), "the app owns the value");
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Checked(false), Message::Checked(false)]
    );
}

#[test]
fn a_disabled_checkbox_ignores_its_label() {
    let mut ui = simulator(labelled(
        checkbox()
            .disabled(true)
            .on_checked_change(Message::Checked),
    ));

    click(&mut ui, 10.0, 10.0);

    assert_eq!(is_checked(&mut ui), Some(false));
    assert_eq!(ui.into_messages().count(), 0);
}

/// A 32px trigger button on top of a content button that publishes
/// [`Message::Inner`] while shown.
fn disclosure<'a>(
    trigger: impl Fn() -> icedxdy_widget::Button<'a, Message> + 'a,
) -> icedxdy_widget::Collapsible<'a, Message> {
    collapsible(move |c| {
        column![
            c.trigger(trigger()),
            c.content(button(block(60.0, 20.0)).on_press(Message::Inner)),
        ]
        .into()
    })
}

#[test]
fn a_collapsible_trigger_reveals_its_content() {
    let element: Element<'_, Message, Theme, Renderer> =
        disclosure(|| button(block(40.0, 20.0)).on_press(Message::Pressed)).into();
    let mut ui = simulator(element);

    click(&mut ui, 5.0, 48.0);
    click(&mut ui, 5.0, 16.0);
    click(&mut ui, 5.0, 48.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Pressed, Message::Inner],
        "the content shows only once opened, and the trigger's own message still fires"
    );
}

#[test]
fn a_controlled_collapsible_reports_instead_of_opening() {
    let element: Element<'_, Message, Theme, Renderer> = disclosure(|| button(block(40.0, 20.0)))
        .open(false)
        .on_open_change(Message::Open)
        .into();
    let mut ui = simulator(element);

    click(&mut ui, 5.0, 16.0);
    click(&mut ui, 5.0, 48.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(true)]
    );
}

#[test]
fn a_disabled_trigger_does_not_toggle() {
    let element: Element<'_, Message, Theme, Renderer> =
        disclosure(|| button(block(40.0, 20.0)).disabled(true)).into();
    let mut ui = simulator(element);

    click(&mut ui, 5.0, 16.0);
    click(&mut ui, 5.0, 48.0);

    assert_eq!(ui.into_messages().count(), 0);
}
