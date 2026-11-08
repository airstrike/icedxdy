//! End-to-end checks that markers, owners and the control operations link
//! widgets the way Radix's `htmlFor` and `asChild` triggers do.
//!
//! Children are fixed-size blocks clicked by position, so the checks do not
//! depend on text shaping in the headless renderer.

use iced_core::keyboard::key::Named;
use iced_core::time::Instant;
use iced_core::{Element, Event, Point, window};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::operation::control;
use icedxdy_widget::radio::radio;
use icedxdy_widget::{
    Inherited, Slot, button, checkbox, collapsible, column, dialog, field, row, sheet, sidebar,
    slider, switch, tabs,
};

type Simulator<'a> = iced_test::Simulator<'a, Message, Theme, Renderer>;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Checked(bool),
    Open(bool),
    Pressed,
    Inner,
    Panel(&'static str),
    Tab(String),
    Chose(u8),
    Value(f32),
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

#[test]
fn a_field_label_chooses_its_radio() {
    let mut ui = simulator(labelled(radio("", 1, None, Message::Chose)));

    click(&mut ui, 10.0, 10.0);

    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [Message::Chose(1)]);
}

#[test]
fn a_radio_under_a_disabled_ancestor_ignores_its_label() {
    let mut control = Slot::from(radio("", 1, None, Message::Chose));
    control.inherit(&Inherited::default().with_disabled(true));
    let mut ui = simulator(labelled(control));

    click(&mut ui, 10.0, 10.0);

    assert_eq!(ui.into_messages().count(), 0);
}

#[test]
fn a_slider_publishes_where_it_is_pressed() {
    let mut ui = simulator(
        slider(0.0..=100.0)
            .width(100.0)
            .on_value_change(Message::Value),
    );

    click(&mut ui, 50.0, 8.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Value(50.0)]
    );
}

#[test]
fn a_disabled_slider_ignores_presses() {
    let mut ui = simulator(
        slider(0.0..=100.0)
            .width(100.0)
            .disabled(true)
            .on_value_change(Message::Value),
    );

    click(&mut ui, 50.0, 8.0);

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

/// Two 56px triggers on a 32px list, and a panel below that publishes its
/// own name.
fn two_tabs<'a>() -> icedxdy_widget::Tabs<'a, Message> {
    tabs(|t| {
        column![
            t.list([
                t.trigger("first", block(40.0, 16.0)),
                t.trigger("second", block(40.0, 16.0)),
            ]),
            t.content(
                "first",
                button(block(60.0, 20.0)).on_press(Message::Panel("first"))
            ),
            t.content(
                "second",
                button(block(60.0, 20.0)).on_press(Message::Panel("second"))
            ),
        ]
        .into()
    })
}

#[test]
fn a_tab_trigger_switches_the_panel() {
    let element: Element<'_, Message, Theme, Renderer> = two_tabs().default_value("first").into();
    let mut ui = simulator(element);

    click(&mut ui, 5.0, 48.0);
    click(&mut ui, 80.0, 16.0);
    click(&mut ui, 5.0, 48.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Panel("first"), Message::Panel("second")]
    );
}

#[test]
fn controlled_tabs_report_the_new_value() {
    let element: Element<'_, Message, Theme, Renderer> = two_tabs()
        .value("first")
        .on_value_change(Message::Tab)
        .into();
    let mut ui = simulator(element);

    click(&mut ui, 80.0, 16.0);
    click(&mut ui, 5.0, 48.0);
    click(&mut ui, 10.0, 16.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Tab("second".to_owned()), Message::Panel("first")],
        "the app owns the value; reselecting the active tab reports nothing"
    );
}

#[test]
fn a_top_sheet_takes_its_contents_height() {
    let element: Element<'_, Message, Theme, Renderer> = sheet(
        |s| s.trigger(button(block(40.0, 20.0))),
        |_| {
            column![block(100.0, 500.0)]
                .height(iced_core::Length::Fill)
                .into()
        },
    )
    .side(sheet::Side::Top)
    .show_close_button(false)
    .open(true)
    .on_open_change(Message::Open)
    .into();
    let mut ui = simulator(element);

    click(&mut ui, 500.0, 450.0);
    click(&mut ui, 500.0, 600.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(false)],
        "the panel is 500px tall, past half of the 768px window"
    );
}

/// A dialog whose trigger sits at the window's top left. Its 384px panel is
/// centered in the 1024×768 window: an inner button at y 352..384 and a
/// close button below it at y 384..416, both from x 336.
fn edit_dialog<'a>() -> icedxdy_widget::Dialog<'a, Message> {
    dialog(
        |d| d.trigger(button(block(40.0, 20.0))),
        |d| {
            column![
                button(block(60.0, 20.0)).on_press(Message::Inner),
                d.close(button(block(40.0, 20.0))),
            ]
            .into()
        },
    )
    .show_close_button(false)
}

#[test]
fn a_dialog_opens_from_its_trigger_and_closes_from_inside() {
    let element: Element<'_, Message, Theme, Renderer> = edit_dialog().into();
    let mut ui = simulator(element);

    click(&mut ui, 340.0, 360.0);
    click(&mut ui, 5.0, 16.0);
    click(&mut ui, 340.0, 360.0);
    click(&mut ui, 340.0, 400.0);

    // A press inside the overlay changes the component without a message
    // for the app; the runtime rebuilds the interface after it, as here.
    let element: Element<'_, Message, Theme, Renderer> = edit_dialog().into();
    let mut ui = ui.rebuild(element);
    click(&mut ui, 340.0, 360.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Inner],
        "the panel shows only between its trigger and its close button"
    );
}

#[test]
fn a_press_outside_or_escape_dismisses_a_controlled_dialog() {
    let element: Element<'_, Message, Theme, Renderer> = edit_dialog()
        .open(true)
        .on_open_change(Message::Open)
        .into();
    let mut ui = simulator(element);

    click(&mut ui, 10.0, 700.0);
    let _ = ui.simulate([simulator::press_key(Named::Escape, None)]);
    click(&mut ui, 340.0, 360.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(false), Message::Open(false), Message::Inner],
        "a controlled dialog asks to close and stays open"
    );
}

/// A sidebar holding a button, beside a main area whose first child is the
/// 28px trigger: at x 256 while the sidebar is open, at x 0 once it slides
/// away.
fn layout<'a>() -> sidebar::Provider<'a, Message> {
    sidebar::provider(|s| {
        row![
            s.sidebar(button(block(100.0, 20.0)).on_press(Message::Inner)),
            column![sidebar::trigger()],
        ]
        .into()
    })
}

#[test]
fn a_sidebar_trigger_anywhere_in_the_provider_toggles_it() {
    let element: Element<'_, Message, Theme, Renderer> = layout().into();
    let mut ui = simulator(element);

    click(&mut ui, 5.0, 16.0);
    click(&mut ui, 270.0, 14.0);
    click(&mut ui, 5.0, 16.0);
    click(&mut ui, 5.0, 16.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Inner, Message::Inner],
        "closed, the trigger moves under the cursor and reopens the sidebar"
    );
}

#[test]
fn a_controlled_sidebar_reports_the_toggle() {
    let element: Element<'_, Message, Theme, Renderer> =
        layout().open(true).on_open_change(Message::Open).into();
    let mut ui = simulator(element);

    click(&mut ui, 270.0, 14.0);
    click(&mut ui, 5.0, 16.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(false), Message::Inner]
    );
}
