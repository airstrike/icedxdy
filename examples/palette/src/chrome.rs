//! Top chrome bar — shadcn site nav · search · create actions.

use iced::widget::{space, svg};
use iced::{Center, Fill};
use iced_widget::button::Status;
use icedxdy::core::{Border, Color, oklch};
use icedxdy::theme::container::background;
use icedxdy::widget::input::{Input, input};
use icedxdy::widget::row::Row;
use icedxdy::widget::{Size, Slot, button, container, row, rule, text};
use icedxdy::{Theme, theme};

use crate::{Message, Mode, State, icon};

const LOGO_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/logo.svg"
));

const NAV_LABELS: &[&str] = &[
    "Docs",
    "Components",
    "Blocks",
    "Charts",
    "Directory",
    "Create",
];

const HEADER_HEIGHT: f32 = 56.0;
const SEARCH_WIDTH: f32 = 256.0;

pub fn view(state: &State) -> iced::Element<'_, Message, Theme> {
    let brand = container(
        svg(svg::Handle::from_memory(LOGO_BYTES))
            .width(22)
            .height(22),
    )
    .width(32.0)
    .height(32.0)
    .align_x(Center)
    .align_y(Center);

    let nav_items: Vec<Slot<'_, Message>> = NAV_LABELS
        .iter()
        .map(|label| {
            button(*label)
                .style(button::Ghost)
                .size(Size::Sm)
                .on_press(Message::Noop)
                .into()
        })
        .collect();
    let nav = Row::with_children(nav_items).spacing(0).align_y(Center);

    let search_input: Input<'_, Message> = input("Search documentation...")
        .width(Fill)
        .on_input(|_| Message::Noop)
        .on_submit(Message::Noop)
        .style(search_input_style());
    let search = container(search_input)
        .width(SEARCH_WIDTH)
        .max_width(SEARCH_WIDTH);

    let mode_button = match state.mode {
        Mode::Light => button(icon::moon().size(16))
            .style(button::Ghost)
            .size(Size::Sm)
            .square()
            .on_press(Message::SetMode(Mode::Dark)),
        Mode::Dark => button(icon::sun().size(16))
            .style(button::Ghost)
            .size(Size::Sm)
            .square()
            .on_press(Message::SetMode(Mode::Light)),
    };

    let github = row![
        icon::github().size(14),
        text::small(state.github_stars.as_deref().unwrap_or("...")),
    ]
    .spacing(6)
    .align_y(Center);

    let auth = row![
        separator(),
        button(github)
            .style(button::Ghost)
            .size(Size::Sm)
            .on_press(Message::Noop),
        separator(),
        mode_button,
        separator(),
        button("Open in v0")
            .style(button::Outline)
            .size(Size::Sm)
            .style(chrome_outline_button_style())
            .on_press(Message::Noop),
        button("Get Code")
            .size(Size::Sm)
            .style(chrome_primary_button_style())
            .on_press(Message::Noop),
    ]
    .spacing(8)
    .align_y(Center);

    let bar = row![brand, nav, space::horizontal(), search, auth,]
        .spacing(8)
        .align_y(Center)
        .width(Fill);

    container(bar)
        .padding([12, 24])
        .width(Fill)
        .height(HEADER_HEIGHT)
        .style(background)
        .into()
}

fn separator<'a>() -> Slot<'a, Message> {
    let line: iced::Element<'a, Message, Theme> = rule::vertical(1).into();
    container(line).height(16.0).into()
}

fn search_input_style() -> theme::text_input::StyleFn<'static, Theme> {
    Box::new(|_, _| theme::text_input::Style {
        background: control().into(),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: border(),
        },
        icon: muted(),
        placeholder: muted(),
        value: foreground(),
        selection: foreground().alpha(0.25),
        ring: None,
    })
}

fn chrome_outline_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
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

fn chrome_primary_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
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
    oklch(0.145, 0.0, 0.0)
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
