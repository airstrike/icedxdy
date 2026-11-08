//! End-to-end checks that markers, owners and the control operations link
//! widgets the way Radix's `htmlFor` and `asChild` triggers do.
//!
//! Children are fixed-size blocks clicked by position, so the checks do not
//! depend on text shaping in the headless renderer.

use std::time::Duration;

use iced_core::keyboard::key::Named;
use iced_core::time::Instant;
use iced_core::{Element, Event, Point, mouse, window};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::operation::control;
use icedxdy_widget::radio::radio;
use icedxdy_widget::{
    Inherited, Slot, alert_dialog, button, checkbox, collapsible, column, dialog, dropdown_menu,
    field, menu, popover, radio_group, row, sheet, sidebar, slider, switch, tabs, textarea, toast,
    toaster, toggle, toggle_group,
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
    PressedSet(Vec<String>),
    Text(String),
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

#[test]
fn an_uncontrolled_toggle_flips_on_press_and_from_its_label() {
    let mut ui = simulator(labelled(
        toggle(block(16.0, 16.0)).on_pressed_change(Message::Checked),
    ));

    click(&mut ui, 10.0, 36.0);
    assert_eq!(is_checked(&mut ui), Some(true));
    click(&mut ui, 10.0, 10.0);
    assert_eq!(is_checked(&mut ui), Some(false));

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Checked(true), Message::Checked(false)]
    );
}

#[test]
fn a_controlled_toggle_publishes_without_changing() {
    let mut ui = simulator(labelled(
        toggle(block(16.0, 16.0))
            .pressed(true)
            .on_pressed_change(Message::Checked),
    ));

    click(&mut ui, 10.0, 36.0);

    assert_eq!(is_checked(&mut ui), Some(true), "the app owns the value");
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Checked(false)]
    );
}

#[test]
fn a_disabled_toggle_ignores_presses() {
    let mut ui = simulator(labelled(
        toggle(block(16.0, 16.0))
            .disabled(true)
            .on_pressed_change(Message::Checked),
    ));

    click(&mut ui, 10.0, 36.0);
    click(&mut ui, 10.0, 10.0);

    assert_eq!(ui.into_messages().count(), 0);
}

#[test]
fn a_radio_group_checks_one_item_at_a_time() {
    let element: Element<'_, Message, Theme, Renderer> =
        radio_group(|g| column![g.item("a"), g.item("b")].spacing(8).into())
            .default_value("a")
            .on_value_change(Message::Tab)
            .into();
    let mut ui = simulator(element);

    assert_eq!(is_checked(&mut ui), Some(true));
    click(&mut ui, 8.0, 32.0);
    assert_eq!(
        is_checked(&mut ui),
        Some(false),
        "the first item is unchecked"
    );
    click(&mut ui, 8.0, 32.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Tab("b".to_owned())],
        "choosing the checked item reports nothing"
    );
}

#[test]
fn a_field_label_chooses_its_radio_group_item() {
    let element: Element<'_, Message, Theme, Renderer> =
        radio_group(|g| labelled(g.item("b")).into())
            .value("a")
            .on_value_change(Message::Tab)
            .into();
    let mut ui = simulator(element);

    click(&mut ui, 10.0, 10.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Tab("b".to_owned())]
    );
}

fn two_toggles<'a>() -> icedxdy_widget::ToggleGroup<'a, Message> {
    toggle_group(|| {
        [
            toggle_group::item("a", block(16.0, 16.0)),
            toggle_group::item("b", block(16.0, 16.0)),
        ]
    })
    .on_value_change(Message::PressedSet)
}

#[test]
fn a_single_toggle_group_presses_and_releases_one_item() {
    let element: Element<'_, Message, Theme, Renderer> = two_toggles().into();
    let mut ui = simulator(element);

    click(&mut ui, 50.0, 16.0);
    click(&mut ui, 50.0, 16.0);
    click(&mut ui, 10.0, 16.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [
            Message::PressedSet(vec!["b".to_owned()]),
            Message::PressedSet(Vec::new()),
            Message::PressedSet(vec!["a".to_owned()]),
        ]
    );
}

#[test]
fn a_multiple_toggle_group_publishes_the_whole_set() {
    let element: Element<'_, Message, Theme, Renderer> = two_toggles()
        .mode(toggle_group::Mode::Multiple)
        .default_value(["a"])
        .into();
    let mut ui = simulator(element);

    click(&mut ui, 50.0, 16.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::PressedSet(vec!["a".to_owned(), "b".to_owned()])]
    );
}

#[test]
fn a_controlled_toggle_group_keeps_the_apps_value() {
    let element: Element<'_, Message, Theme, Renderer> = two_toggles().value(["a"]).into();
    let mut ui = simulator(element);

    click(&mut ui, 50.0, 16.0);
    click(&mut ui, 50.0, 16.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [
            Message::PressedSet(vec!["b".to_owned()]),
            Message::PressedSet(vec!["b".to_owned()]),
        ]
    );
}

#[test]
fn an_uncontrolled_textarea_reports_its_edits() {
    let element: Element<'_, Message, Theme, Renderer> = textarea("")
        .default_value("a")
        .on_input(Message::Text)
        .into();
    let mut ui = simulator(element);

    click(&mut ui, 100.0, 20.0);
    let _ = ui.typewrite("b");

    // The null renderer's editor keeps no text, so only the edit itself is
    // observable here.
    assert!(matches!(
        ui.into_messages().collect::<Vec<_>>().as_slice(),
        [Message::Text(_)]
    ));
}

#[test]
fn a_disabled_textarea_ignores_typing() {
    let element: Element<'_, Message, Theme, Renderer> =
        textarea("").disabled(true).on_input(Message::Text).into();
    let mut ui = simulator(element);

    click(&mut ui, 100.0, 20.0);
    let _ = ui.typewrite("b");

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

#[test]
fn an_alert_dialog_ignores_presses_outside_and_closes_from_cancel() {
    let element: Element<'_, Message, Theme, Renderer> = alert_dialog(
        |a| a.trigger(button(block(40.0, 20.0))),
        |a| a.cancel(button(block(60.0, 20.0))),
    )
    .open(true)
    .on_open_change(Message::Open)
    .into();
    let mut ui = simulator(element);

    click(&mut ui, 10.0, 700.0);
    click(&mut ui, 340.0, 384.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(false)],
        "only the cancel button closes it"
    );
}

/// A toaster whose 40×20 trigger sits at the top left and raises a toast
/// with an action publishing [`Message::Inner`]. The toast's card sits at
/// the bottom right; its 16px action button (no text in the null renderer)
/// spans x 968..984, y 704..728.
fn toasting<'a>() -> Element<'a, Message, Theme, Renderer> {
    toaster(toast::trigger(
        button(block(40.0, 20.0)),
        toast::toast("Saved").action("Undo", Message::Inner),
    ))
    .into()
}

fn redraw_at(ui: &mut Simulator<'_>, now: Instant) {
    let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(now))]);
}

#[test]
fn a_toast_action_publishes_and_dismisses_the_toast() {
    let mut ui = simulator(toasting());

    click(&mut ui, 10.0, 10.0);
    let mut ui = ui.rebuild(toasting());
    click(&mut ui, 976.0, 716.0);
    let mut ui = ui.rebuild(toasting());
    click(&mut ui, 976.0, 716.0);

    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [Message::Inner]);
}

#[test]
fn a_toast_times_out_by_itself() {
    let mut ui = simulator(toasting());

    click(&mut ui, 10.0, 10.0);
    let mut ui = ui.rebuild(toasting());
    let now = Instant::now();
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + Duration::from_secs(5));
    let mut ui = ui.rebuild(toasting());
    click(&mut ui, 976.0, 716.0);

    assert_eq!(ui.into_messages().count(), 0, "the toast is gone");
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

/// A popover whose trigger sits at the window's top left. Its start-aligned
/// panel opens 4px under the 32px trigger with 10px of padding: an inner
/// button at y 46..78 and a close button below it at y 78..110, from x 10.
fn edit_popover<'a>() -> icedxdy_widget::Popover<'a, Message> {
    popover(
        |p| p.trigger(button(block(40.0, 20.0))),
        |p| {
            column![
                button(block(60.0, 20.0)).on_press(Message::Inner),
                p.close(button(block(40.0, 20.0))),
            ]
            .into()
        },
    )
    .align(popover::Align::Start)
}

#[test]
fn a_popover_opens_from_its_trigger_and_closes_from_inside() {
    let element: Element<'_, Message, Theme, Renderer> = edit_popover().into();
    let mut ui = simulator(element);

    click(&mut ui, 15.0, 60.0);
    click(&mut ui, 5.0, 16.0);
    click(&mut ui, 15.0, 60.0);
    click(&mut ui, 15.0, 95.0);

    let element: Element<'_, Message, Theme, Renderer> = edit_popover().into();
    let mut ui = ui.rebuild(element);
    click(&mut ui, 15.0, 60.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Inner],
        "the panel shows only between its trigger and its close button"
    );
}

#[test]
fn a_popover_trigger_toggles_it_closed() {
    let element: Element<'_, Message, Theme, Renderer> = edit_popover()
        .open(true)
        .on_open_change(Message::Open)
        .into();
    let mut ui = simulator(element);

    click(&mut ui, 5.0, 16.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(false)],
        "the trigger closes it once, not closed and reopened"
    );
}

#[test]
fn a_press_outside_or_escape_dismisses_a_controlled_popover() {
    let element: Element<'_, Message, Theme, Renderer> = column![
        edit_popover().open(true).on_open_change(Message::Open),
        Slot::from(Space::new().height(400)),
        button(block(60.0, 20.0)).on_press(Message::Pressed),
    ]
    .into();
    let mut ui = simulator(element);

    click(&mut ui, 15.0, 445.0);
    let _ = ui.simulate([simulator::press_key(Named::Escape, None)]);
    click(&mut ui, 15.0, 60.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [
            Message::Open(false),
            Message::Pressed,
            Message::Open(false),
            Message::Inner
        ],
        "a popover is not modal: the page still gets the press that dismisses it"
    );
}

/// A menu whose trigger sits at the window's top left. Its start-aligned,
/// 128px panel opens 4px under the 32px trigger with `p-1`: 28px items at
/// y 40..68 (selectable), 68..96 (disabled) and 96..124 (a submenu, whose
/// panel opens to its right with its first item level with it, from x 132).
fn account_menu<'a>() -> icedxdy_widget::DropdownMenu<'a, Message> {
    dropdown_menu(
        |m| m.trigger(button(block(40.0, 20.0))),
        |_| {
            column![
                menu::item(block(60.0, 20.0)).on_select(Message::Inner),
                menu::item(block(60.0, 20.0))
                    .on_select(Message::Checked(true))
                    .disabled(true),
                menu::sub(
                    block(60.0, 20.0),
                    menu::item(block(60.0, 20.0)).on_select(Message::Pressed),
                ),
            ]
            .into()
        },
    )
    .align(dropdown_menu::Align::Start)
}

fn hover(ui: &mut Simulator<'_>, x: f32, y: f32) {
    let position = Point::new(x, y);
    ui.point_at(position);
    let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position })]);
}

#[test]
fn a_menu_item_publishes_and_closes_the_menu() {
    let element: Element<'_, Message, Theme, Renderer> = account_menu().into();
    let mut ui = simulator(element);

    click(&mut ui, 5.0, 16.0);
    click(&mut ui, 50.0, 80.0);
    click(&mut ui, 50.0, 54.0);

    let element: Element<'_, Message, Theme, Renderer> = account_menu().into();
    let mut ui = ui.rebuild(element);
    click(&mut ui, 50.0, 54.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Inner],
        "the disabled item does nothing; the selected one closes the menu"
    );
}

#[test]
fn a_menu_is_modal() {
    let element: Element<'_, Message, Theme, Renderer> = column![
        account_menu().open(true).on_open_change(Message::Open),
        Slot::from(Space::new().height(400)),
        button(block(60.0, 20.0)).on_press(Message::Pressed),
    ]
    .into();
    let mut ui = simulator(element);

    click(&mut ui, 15.0, 445.0);
    click(&mut ui, 5.0, 16.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(false), Message::Open(false)],
        "the page and the trigger only dismiss an open menu"
    );
}

#[test]
fn a_submenu_item_closes_the_whole_menu() {
    let element: Element<'_, Message, Theme, Renderer> = account_menu()
        .default_open(true)
        .on_open_change(Message::Open)
        .into();
    let mut ui = simulator(element);

    hover(&mut ui, 50.0, 110.0);
    hover(&mut ui, 140.0, 110.0);
    click(&mut ui, 140.0, 110.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Pressed, Message::Open(false)],
    );
}
