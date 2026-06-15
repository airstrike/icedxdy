//! Left rail — `/create`-style customizer card.

use iced::widget::space;
use iced::{Center, Fill};
use iced_widget::button::Status;
use icedxdy::Theme;
use icedxdy::core::{Border, Color, oklch};
use icedxdy::theme::container::Style;
use icedxdy::widget::{Component, Size, button, column, container, row, rule, scrollable, text};

use crate::{Message, State, icon};

const RAIL_WIDTH: f32 = 224.0;
const FIELD_HEIGHT: f32 = 54.0;
const HEADER_HEIGHT: f32 = 60.0;
const FOOTER_PAD_X: f32 = 14.0;
const SECTION_PAD_X: f32 = 12.0;
const SECTION_PAD_Y: f32 = 12.0;

#[derive(Debug, Clone, Copy)]
enum Trailing {
    Dot,
    Menu,
    Paintbrush,
    Radius,
    Square,
    Settings,
    Text(&'static str),
}

pub fn view(_state: &State) -> iced::Element<'_, Message, Theme> {
    let fields = column![
        section([
            picker("Style", "Nova", Trailing::Square),
            picker("Base Color", "Neutral", Trailing::Dot),
            picker("Theme", "Neutral", Trailing::Dot),
            picker("Chart Color", "Neutral", Trailing::Dot),
        ]),
        divider(),
        section([
            picker("Heading", "Inter", Trailing::Text("Aa")),
            picker("Font", "Inter", Trailing::Text("Aa")),
        ]),
        divider(),
        section([
            picker("Icon Library", "Lucide", Trailing::Settings),
            picker("Radius", "Default", Trailing::Radius),
        ]),
        divider(),
        section([
            picker("Menu", "Default / Solid", Trailing::Menu),
            picker("Menu Accent", "Subtle", Trailing::Paintbrush),
        ]),
    ]
    .spacing(0)
    .width(Fill);

    let content = column![
        menu_header(),
        divider(),
        scrollable(fields).width(Fill).height(Fill),
        divider(),
        preset_actions(),
        divider(),
        get_code(),
    ]
    .spacing(0)
    .width(Fill)
    .height(Fill);

    container(content)
        .width(RAIL_WIDTH)
        .height(Fill)
        .clip(true)
        .style(rail_style)
        .into()
}

fn menu_header() -> Component<'static, Message> {
    let trigger = row![
        text::small("Menu").medium().style(text_style(foreground())),
        space::horizontal(),
        icon::menu().size(20),
    ]
    .align_y(Center)
    .width(Fill);

    container(
        container(trigger)
            .width(Fill)
            .center_y(36.0)
            .padding([0, 7])
            .style(control_style),
    )
    .height(HEADER_HEIGHT)
    .padding([12.0, SECTION_PAD_X])
    .width(Fill)
    .into()
}

fn section(
    fields: impl IntoIterator<Item = Component<'static, Message>>,
) -> Component<'static, Message> {
    container(column(fields).spacing(10).width(Fill))
        .padding([SECTION_PAD_Y, SECTION_PAD_X])
        .width(Fill)
        .into()
}

fn picker(
    label: &'static str,
    value: &'static str,
    trailing: Trailing,
) -> Component<'static, Message> {
    let copy = column![
        text::xs(label).style(text_style(muted())),
        text::small(value).medium().style(text_style(foreground())),
    ]
    .spacing(1)
    .width(Fill);

    let content = row![copy, trailing_view(trailing)]
        .spacing(10)
        .align_y(Center)
        .width(Fill);

    container(content)
        .height(FIELD_HEIGHT)
        .padding([8, 10])
        .width(Fill)
        .style(control_style)
        .into()
}

fn trailing_view(trailing: Trailing) -> Component<'static, Message> {
    match trailing {
        Trailing::Dot => container(space::vertical())
            .width(16.0)
            .height(16.0)
            .style(dot_style)
            .into(),
        Trailing::Square => lucide(icon::square().size(16)),
        Trailing::Menu => lucide(icon::menu().size(16)),
        Trailing::Radius => lucide(icon::radius().size(16)),
        Trailing::Settings => lucide(icon::settings().size(16)),
        Trailing::Paintbrush => lucide(icon::paintbrush().size(16)),
        Trailing::Text(label) => text_icon(label),
    }
}

fn text_icon(label: &'static str) -> Component<'static, Message> {
    container(text::small(label).style(text_style(foreground())))
        .width(24.0)
        .align_x(Center)
        .into()
}

fn lucide(icon: impl Into<Component<'static, Message>>) -> Component<'static, Message> {
    container(icon).width(24.0).align_x(Center).into()
}

fn preset_actions() -> Component<'static, Message> {
    let content = column![
        button::outline("--preset b0")
            .size(Size::Sm)
            .width(Fill)
            .style(rail_button_style())
            .on_press(Message::Noop),
        button::outline("Open Preset")
            .size(Size::Sm)
            .width(Fill)
            .style(rail_button_style())
            .on_press(Message::Noop),
        button::outline("Shuffle")
            .size(Size::Sm)
            .width(Fill)
            .style(rail_button_style())
            .on_press(Message::Noop),
    ]
    .spacing(8)
    .width(Fill);

    container(content)
        .padding([12.0, FOOTER_PAD_X])
        .width(Fill)
        .into()
}

fn get_code() -> Component<'static, Message> {
    container(
        button("Get Code")
            .size(Size::Sm)
            .width(Fill)
            .style(rail_primary_button_style())
            .on_press(Message::Noop),
    )
    .padding([12.0, FOOTER_PAD_X])
    .width(Fill)
    .into()
}

fn divider() -> Component<'static, Message> {
    let line: iced::Element<'static, Message, Theme> =
        rule::horizontal(1).style(divider_style()).into();

    container(line).width(Fill).height(1.0).into()
}

fn rail_style(_theme: &Theme) -> Style {
    Style {
        text_color: Some(foreground()),
        background: Some(surface().alpha(0.90).into()),
        border: Border {
            radius: 18.0.into(),
            width: 1.0,
            color: foreground().alpha(0.10),
        },
        ..Style::default()
    }
}

fn control_style(_theme: &Theme) -> Style {
    Style {
        text_color: Some(foreground()),
        background: Some(control().into()),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: border(),
        },
        ..Style::default()
    }
}

fn dot_style(_theme: &Theme) -> Style {
    Style {
        background: Some(muted().into()),
        border: Border {
            radius: 999.0.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

fn text_style(color: Color) -> iced_widget::text::StyleFn<'static, Theme> {
    Box::new(move |_| iced_widget::text::Style {
        color: Some(color.into()),
    })
}

fn divider_style() -> rule::StyleFn<'static, Theme> {
    Box::new(|_| rule::Style {
        background: iced::Background::Color(border().into()),
        ..rule::Style::default()
    })
}

fn rail_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|_, status| {
        let background = match status {
            Status::Hovered | Status::Pressed => control_hover(),
            Status::Active | Status::Disabled => control(),
        };

        iced_widget::button::Style {
            background: Some(iced::Background::Color(background.into())),
            text_color: foreground().into(),
            border: iced::Border {
                radius: 8.0.into(),
                width: 1.0,
                color: border().into(),
            },
            ..Default::default()
        }
    })
}

fn rail_primary_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|_, status| {
        let background = match status {
            Status::Disabled => foreground().alpha(0.55),
            Status::Pressed => foreground().alpha(0.86),
            Status::Hovered | Status::Active => foreground(),
        };

        iced_widget::button::Style {
            background: Some(iced::Background::Color(background.into())),
            text_color: surface().into(),
            border: iced::Border {
                radius: 8.0.into(),
                ..iced::Border::default()
            },
            ..Default::default()
        }
    })
}

fn surface() -> Color {
    oklch(0.205, 0.0, 0.0)
}

fn control() -> Color {
    oklch(0.19, 0.0, 0.0)
}

fn control_hover() -> Color {
    oklch(0.24, 0.0, 0.0)
}

fn border() -> Color {
    oklch(0.29, 0.0, 0.0)
}

fn foreground() -> Color {
    oklch(0.985, 0.0, 0.0)
}

fn muted() -> Color {
    oklch(0.708, 0.0, 0.0)
}
