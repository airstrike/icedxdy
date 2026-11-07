//! Left rail — `/create`-style customizer card.

use iced::widget::{space, svg};
use iced::{Center, Fill};
use iced_widget::button::Status;
use icedxdy::Theme;
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::{Size, Slot, button, column, container, row, rule, scrollable, text};

use crate::{Message, State, icon};

const RAIL_WIDTH: f32 = 224.0;
const FIELD_HEIGHT: f32 = 54.0;
const HEADER_HEIGHT: f32 = 60.0;
const FOOTER_PAD_X: f32 = 14.0;
const SECTION_PAD_X: f32 = 12.0;
const SECTION_PAD_Y: f32 = 12.0;
const TRAILING_ICON_SIZE: f32 = 16.0;
const NOVA_ICON: &[u8] = br##"
<svg xmlns="http://www.w3.org/2000/svg" width="128" height="128" viewBox="0 0 24 24" fill="none" role="img" color="currentColor">
  <path d="M2 12C2 9.19974 2 7.79961 2.54497 6.73005C3.02433 5.78924 3.78924 5.02433 4.73005 4.54497C5.79961 4 7.19974 4 10 4H14C16.8003 4 18.2004 4 19.27 4.54497C20.2108 5.02433 20.9757 5.78924 21.455 6.73005C22 7.79961 22 9.19974 22 12C22 14.8003 22 16.2004 21.455 17.27C20.9757 18.2108 20.2108 18.9757 19.27 19.455C18.2004 20 16.8003 20 14 20H10C7.19974 20 5.79961 20 4.73005 19.455C3.78924 18.9757 3.02433 18.2108 2.54497 17.27C2 16.2004 2 14.8003 2 12Z" stroke="currentColor" stroke-width="2" stroke-linejoin="round"/>
</svg>
"##;
const LUCIDE_LOGO_ICON: &[u8] = br##"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24">
  <path d="M14 12a4 4 0 0 0-8 0 8 8 0 1 0 16 0 11.97 11.97 0 0 0-4-8.944"/>
  <path d="M10 12a4 4 0 0 0 8 0 8 8 0 1 0-16 0 11.97 11.97 0 0 0 4.063 9"/>
</svg>
"##;
const RADIUS_ICON: &[u8] = br##"
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none">
  <g transform="rotate(90 12 12)">
    <path d="M4 20v-5C4 8.925 8.925 4 15 4h5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
  </g>
</svg>
"##;
const MENU_ACCENT_ICON: &[u8] = br##"
<svg xmlns="http://www.w3.org/2000/svg" width="128" height="128" viewBox="0 0 24 24" fill="none">
  <path d="M19 12.1294L12.9388 18.207C11.1557 19.9949 10.2641 20.8889 9.16993 20.9877C8.98904 21.0041 8.80705 21.0041 8.62616 20.9877C7.53195 20.8889 6.64039 19.9949 4.85726 18.207L2.83687 16.1811C1.72104 15.0622 1.72104 13.2482 2.83687 12.1294M19 12.1294L10.9184 4.02587M19 12.1294H2.83687M10.9184 4.02587L2.83687 12.1294M10.9184 4.02587L8.89805 2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="M22 20C22 21.1046 21.1046 22 20 22C18.8954 22 18 21.1046 18 20C18 18.8954 20 17 20 17C20 17 22 18.8954 22 20Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
"##;

#[derive(Debug, Clone, Copy)]
enum Trailing {
    Dot,
    IconLibrary,
    Menu,
    MenuAccent,
    Nova,
    Radius,
    Text(&'static str),
}

pub fn view(_state: &State) -> iced::Element<'_, Message, Theme> {
    let fields = column![
        section([picker("Style", "Nova", Trailing::Nova),]),
        divider(),
        section([
            picker("Base Color", "Neutral", Trailing::Dot),
            picker("Theme", "Neutral", Trailing::Dot),
            picker("Chart Color", "Neutral", Trailing::Dot),
        ]),
        divider(),
        section([
            picker("Heading", "Geist", Trailing::Text("Aa")),
            picker("Font", "Geist", Trailing::Text("Aa")),
        ]),
        divider(),
        section([
            picker("Icon Library", "Lucide", Trailing::IconLibrary),
            picker("Radius", "Default", Trailing::Radius),
        ]),
        divider(),
        section([
            picker("Menu", "Default / Solid", Trailing::Menu),
            picker("Menu Accent", "Subtle", Trailing::MenuAccent),
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

fn menu_header() -> Slot<'static, Message> {
    let trigger = row![
        text::small("Menu").medium().style(foreground_text_style()),
        space::horizontal(),
        menu_header_icon(),
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

fn section(fields: impl IntoIterator<Item = Slot<'static, Message>>) -> Slot<'static, Message> {
    container(column(fields).spacing(13).width(Fill))
        .padding([SECTION_PAD_Y, SECTION_PAD_X])
        .width(Fill)
        .into()
}

fn picker(label: &'static str, value: &'static str, trailing: Trailing) -> Slot<'static, Message> {
    let copy = column![
        text::xs(label).style(muted_text_style()),
        text::small(value).medium().style(foreground_text_style()),
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

fn trailing_view(trailing: Trailing) -> Slot<'static, Message> {
    match trailing {
        Trailing::Dot => container(space::vertical())
            .width(TRAILING_ICON_SIZE)
            .height(TRAILING_ICON_SIZE)
            .style(dot_style)
            .into(),
        Trailing::IconLibrary => svg_icon(LUCIDE_LOGO_ICON),
        Trailing::Menu => lucide(icon::menu().size(16)),
        Trailing::MenuAccent => svg_icon(MENU_ACCENT_ICON),
        Trailing::Nova => svg_icon(NOVA_ICON),
        Trailing::Radius => svg_icon(RADIUS_ICON),
        Trailing::Text(label) => text_icon(label),
    }
}

fn text_icon(label: &'static str) -> Slot<'static, Message> {
    icon_slot(text::p(label).style(foreground_text_style()))
}

fn lucide(icon: impl Into<Slot<'static, Message>>) -> Slot<'static, Message> {
    icon_slot(icon)
}

fn svg_icon(bytes: &'static [u8]) -> Slot<'static, Message> {
    let icon = svg(svg::Handle::from_memory(bytes))
        .width(TRAILING_ICON_SIZE)
        .height(TRAILING_ICON_SIZE)
        .style(|theme: &Theme, _| iced_widget::svg::Style {
            color: Some(theme.sidebar.foreground.into()),
        });

    icon_slot(icon)
}

fn icon_slot(icon: impl Into<Slot<'static, Message>>) -> Slot<'static, Message> {
    container(icon)
        .width(TRAILING_ICON_SIZE)
        .height(TRAILING_ICON_SIZE)
        .align_x(Center)
        .align_y(Center)
        .into()
}

fn menu_header_icon() -> Slot<'static, Message> {
    let icon = icon::menu().size(20);

    container(icon)
        .width(20.0)
        .height(20.0)
        .align_x(Center)
        .align_y(Center)
        .into()
}

fn preset_actions() -> Slot<'static, Message> {
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

fn get_code() -> Slot<'static, Message> {
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

fn divider() -> Slot<'static, Message> {
    let line: iced::Element<'static, Message, Theme> =
        rule::horizontal(1).style(divider_style()).into();

    container(line).width(Fill).height(1.0).into()
}

fn rail_style(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.sidebar.foreground),
        background: Some(theme.sidebar.background.into()),
        border: Border {
            radius: 18.0.into(),
            width: 1.0,
            color: theme.sidebar.border,
        },
        ..Style::default()
    }
}

fn control_style(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.sidebar.foreground),
        background: None,
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: theme.sidebar.border,
        },
        ..Style::default()
    }
}

fn dot_style(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.foreground.into()),
        border: Border {
            radius: 999.0.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

fn foreground_text_style() -> iced_widget::text::StyleFn<'static, Theme> {
    Box::new(|theme| iced_widget::text::Style {
        color: Some(theme.sidebar.foreground.into()),
        selection: None,
    })
}

fn muted_text_style() -> iced_widget::text::StyleFn<'static, Theme> {
    Box::new(|theme| iced_widget::text::Style {
        color: Some(theme.muted.foreground.into()),
        selection: None,
    })
}

fn divider_style() -> rule::StyleFn<'static, Theme> {
    Box::new(|theme| rule::Style {
        background: iced::Background::Color(theme.sidebar.border.into()),
        ..rule::Style::default()
    })
}

fn rail_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|theme, status| {
        let background = match status {
            Status::Hovered | Status::Pressed => theme.sidebar.accent.base,
            Status::Active | Status::Disabled => theme.sidebar.background,
        };

        iced_widget::button::Style {
            background: Some(iced::Background::Color(background.into())),
            text_color: theme.sidebar.foreground.into(),
            border: iced::Border {
                radius: 8.0.into(),
                width: 1.0,
                color: theme.sidebar.border.into(),
            },
            ..Default::default()
        }
    })
}

fn rail_primary_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|theme, status| {
        let background = match status {
            Status::Disabled => theme.sidebar.foreground.alpha(0.55),
            Status::Pressed => theme.sidebar.foreground.alpha(0.86),
            Status::Hovered | Status::Active => theme.sidebar.foreground,
        };

        iced_widget::button::Style {
            background: Some(iced::Background::Color(background.into())),
            text_color: theme.sidebar.background.into(),
            border: iced::Border {
                radius: 8.0.into(),
                ..iced::Border::default()
            },
            ..Default::default()
        }
    })
}
