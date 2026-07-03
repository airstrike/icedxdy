//! `/create` preview showcase cards from the shadcn reference.

use iced::widget::{space, stack};
use iced::{Center, Fill};
use iced_widget::button::Status;
use icedxdy::Theme;
use icedxdy::card;
use icedxdy::core::{Border, Color, oklch};
use icedxdy::theme;
use icedxdy::theme::text_input;
use icedxdy::widget::button::Button;
use icedxdy::widget::row::Row;
use icedxdy::widget::{
    Component, Size, Skeleton, badge, button, checkbox, column, container, input, row, skeleton,
    slider, switch, text,
};

use crate::{Message, State, icon};

const CARD_THEME_HEIGHT: f32 = 292.0;
const CARD_TYPE_HEIGHT: f32 = 316.0;
const CARD_ICONS_HEIGHT: f32 = 114.0;
const CARD_CONTROLS_HEIGHT: f32 = 397.0;
const CARD_SKELETON_HEIGHT: f32 = 190.0;

pub fn theme_card() -> iced::Element<'static, Message, Theme> {
    let swatches = column![
        swatch_row([
            ("--backgr...", oklch(0.0, 0.0, 0.0)),
            ("--foregr...", oklch(0.985, 0.0, 0.0)),
            ("--primary", oklch(0.93, 0.0, 0.0)),
            ("--second...", oklch(0.17, 0.0, 0.0)),
            ("--muted", oklch(0.17, 0.0, 0.0)),
            ("--accent", oklch(0.17, 0.0, 0.0)),
        ]),
        swatch_row([
            ("--border", oklch(0.22, 0.0, 0.0)),
            ("--chart-1", oklch(0.78, 0.0, 0.0)),
            ("--chart-2", oklch(0.55, 0.0, 0.0)),
            ("--chart-3", oklch(0.37, 0.0, 0.0)),
            ("--chart-4", oklch(0.29, 0.0, 0.0)),
            ("--chart-5", oklch(0.17, 0.0, 0.0)),
        ]),
    ]
    .spacing(16)
    .width(Fill);

    let body = column![
        text::p("Nova - Inter").size(24.0).semibold(),
        text::muted(
            "Designers love packing quirky glyphs into\n\
             test phrases. This is a preview of the..."
        )
        .size(16.0),
        swatches,
    ]
    .spacing(12)
    .width(Fill);

    panel(body, CARD_THEME_HEIGHT)
}

pub fn typography_card() -> iced::Element<'static, Message, Theme> {
    let body = column![
        text::xs("INHERIT - INTER").style(theme::text::Muted),
        text::p("Designing with rhythm and\nhierarchy.")
            .size(24.0)
            .semibold(),
        text::muted(
            "A strong body style keeps long-form content\n\
             readable and balances the visual weight of\n\
             headings."
        )
        .size(16.0),
        text::muted(
            "Thoughtful spacing and cadence help paragraphs\n\
             scan quickly without feeling dense."
        )
        .size(16.0),
        space::vertical(),
        button::outline("Share Feedback")
            .size(Size::Sm)
            .width(Fill)
            .style(control_outline_button_style())
            .on_press(Message::Noop),
    ]
    .spacing(14)
    .width(Fill)
    .height(Fill);

    panel(body, CARD_TYPE_HEIGHT)
}

pub fn icon_grid() -> iced::Element<'static, Message, Theme> {
    let first = row![
        icon_button(icon::copy()),
        icon_button(icon::circle_help()),
        icon_button(icon::trash_2()),
        icon_button(icon::upload()),
        icon_button(icon::archive()),
        icon_button(icon::ellipsis()),
        icon_button(icon::refresh_cw()),
        icon_button(icon::plus()),
    ]
    .spacing(10)
    .width(Fill);

    let second = row![
        icon_button(icon::minus()),
        icon_button(icon::arrow_left()),
        icon_button(icon::arrow_right()),
        icon_button(icon::check()),
        icon_button(icon::chevron_down()),
        icon_button(icon::chevron_right()),
        icon_button(icon::search()),
        icon_button(icon::settings()),
    ]
    .spacing(10)
    .width(Fill);

    panel(
        column![first, second].spacing(12).width(Fill),
        CARD_ICONS_HEIGHT,
    )
}

pub fn controls_card(state: &State) -> iced::Element<'_, Message, Theme> {
    let tabs = row![
        button("Button").size(Size::Sm).on_press(Message::Noop),
        button::secondary("Secondary")
            .size(Size::Sm)
            .on_press(Message::Noop),
        button::outline("Outline")
            .size(Size::Sm)
            .style(control_outline_button_style())
            .on_press(Message::Noop),
        button::ghost("Ghost")
            .size(Size::Sm)
            .on_press(Message::Noop),
    ]
    .spacing(8)
    .align_y(Center)
    .width(Fill);

    let field = container(row![
        text::small("Two-factor authentication"),
        space::horizontal(),
        button::secondary("Enable")
            .size(Size::Sm)
            .on_press(Message::Noop),
    ])
    .padding([8, 12])
    .height(50.0)
    .width(Fill)
    .style(control_style);

    let badges = row![
        badge("Badge"),
        badge::secondary("Secondary"),
        badge::outline("Outline"),
        space::horizontal(),
        checkbox(state.accept_terms).on_toggle(Message::AcceptTerms),
        checkbox(state.showcase_middle_checkbox).on_toggle(Message::SetShowcaseMiddleCheckbox),
        checkbox(state.showcase_trailing_checkbox).on_toggle(Message::SetShowcaseTrailingCheckbox),
    ]
    .spacing(10)
    .align_y(Center)
    .width(Fill);

    let footer = row![
        button::outline("Alert Dialog")
            .size(Size::Sm)
            .style(control_outline_button_style())
            .on_press(Message::Noop),
        button::outline(
            row![
                text::small("Button Group"),
                space::horizontal(),
                icon::chevron_up().size(14)
            ]
            .spacing(10)
            .align_y(Center)
            .width(Fill),
        )
        .size(Size::Sm)
        .width(143.0)
        .style(control_outline_button_style())
        .on_press(Message::Noop),
        space::horizontal(),
        switch(state.connected).on_toggle(Message::ToggleConnection),
    ]
    .spacing(8)
    .align_y(Center)
    .width(Fill);

    let body = column![
        tabs,
        field,
        slider(
            0.0..=100.0,
            state.showcase_slider,
            Message::SetShowcaseSlider
        )
        .width(Fill),
        name_input(state),
        textarea_placeholder(),
        badges,
        footer,
    ]
    .spacing(16)
    .width(Fill);

    panel(body, CARD_CONTROLS_HEIGHT)
}

pub fn skeleton_card() -> iced::Element<'static, Message, Theme> {
    let lines = column![
        row![
            skeleton().width(40.0).height(40.0).radius(999.0),
            column![skeleton_line(118.0, 16.0), skeleton_line(180.0, 12.0),].spacing(12)
        ]
        .spacing(14)
        .align_y(Center),
        skeleton_line(258.0, 14.0),
        skeleton_line(230.0, 14.0),
        row![skeleton_line(96.0, 32.0), skeleton_line(78.0, 32.0)].spacing(10),
    ]
    .spacing(16)
    .width(Fill);

    panel(lines, CARD_SKELETON_HEIGHT)
}

fn swatch_row(items: [(&'static str, Color); 6]) -> Component<'static, Message> {
    Row::with_children(items.into_iter().map(|(label, color)| {
        column![
            swatch(color),
            text::xs(label).size(10.0).style(theme::text::Muted)
        ]
        .spacing(7)
        .width(48.0)
        .into()
    }))
    .spacing(11)
    .width(Fill)
    .into()
}

fn swatch(color: Color) -> Component<'static, Message> {
    container(space::vertical())
        .width(48.0)
        .height(48.0)
        .style(move |_| theme::container::Style {
            background: Some(color.into()),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: oklch(0.29, 0.0, 0.0),
            },
            ..Default::default()
        })
        .into()
}

fn icon_button<'a>(icon: impl Into<Component<'a, Message>>) -> Button<'a, Message> {
    button::outline(icon)
        .size(Size::Sm)
        .square()
        .style(control_outline_button_style())
        .on_press(Message::Noop)
}

fn panel<'a>(
    body: impl Into<Component<'a, Message>>,
    height: f32,
) -> iced::Element<'a, Message, Theme> {
    card::card(container(body.into()).padding(16).width(Fill).height(Fill))
        .width(Fill)
        .height(height)
        .into()
}

fn skeleton_line(width: f32, height: f32) -> Skeleton<'static, Message> {
    skeleton().width(width).height(height)
}

fn name_input(state: &State) -> Component<'_, Message> {
    let input = input("Name", &state.showcase_name)
        .width(Fill)
        .style(control_input_style())
        .on_input(Message::ShowcaseNameChanged)
        .on_submit(Message::Noop);

    let trailing = container(icon::search().size(16))
        .width(Fill)
        .height(32.0)
        .padding([0, 10])
        .align_x(iced::Right)
        .align_y(Center);

    stack![input, trailing].width(Fill).height(32.0).into()
}

fn textarea_placeholder() -> Component<'static, Message> {
    container(text::muted("Message").size(14.0))
        .width(Fill)
        .height(64.0)
        .padding([10, 10])
        .align_y(iced::Top)
        .style(control_style)
        .into()
}

fn control_style(_theme: &Theme) -> theme::container::Style {
    theme::container::Style {
        background: Some(oklch(0.16, 0.0, 0.0).into()),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: oklch(0.29, 0.0, 0.0),
        },
        ..Default::default()
    }
}

fn control_input_style() -> theme::text_input::StyleFn<'static, Theme> {
    Box::new(|theme, status| {
        let border = match status {
            text_input::Status::Focused { .. } => theme.neutral.ring,
            _ => oklch(0.29, 0.0, 0.0),
        };

        theme::text_input::Style {
            background: oklch(0.16, 0.0, 0.0).into(),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: border,
            },
            icon: theme.muted.foreground,
            placeholder: theme.muted.foreground,
            value: theme.neutral.foreground,
            selection: theme.neutral.foreground.alpha(0.25),
            ring: None,
        }
    })
}

fn control_outline_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|theme, status| {
        let background = match status {
            Status::Hovered | Status::Pressed => oklch(0.22, 0.0, 0.0),
            Status::Active | Status::Disabled => oklch(0.16, 0.0, 0.0),
        };

        iced_widget::button::Style {
            background: Some(iced::Background::Color(background.into())),
            text_color: theme.neutral.foreground.into(),
            border: iced::Border {
                radius: 8.0.into(),
                width: 1.0,
                color: oklch(0.29, 0.0, 0.0).into(),
            },
            ..Default::default()
        }
    })
}
