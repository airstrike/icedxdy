//! Preview-02 kitchen-island card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/kitchen-island.tsx`.

use iced::Fill;
use iced::Widget;
use icedxdy::card;
use icedxdy::core::{Border, Color, oklch};
use icedxdy::theme;
use icedxdy::widget::{Size, column, container, item, slider, space, text, toggle_group};
use icedxdy::{Slot, Space, Theme};

use crate::{KitchenMode, KitchenScene, Message, State};

pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let mode = state.kitchen_mode;
    let modes = toggle_group(|| MODES.map(|(_, label)| toggle_group::item(label, label)))
        .value([label_of(&MODES, mode)])
        .on_value_change(move |pressed| {
            Message::SetKitchenMode(chosen(&MODES, &pressed).unwrap_or(mode))
        })
        .style(toggle_group::Outline);

    let scene = state.kitchen_scene;
    let scenes = toggle_group(|| SCENES.map(|(_, label)| toggle_group::item(label, label)))
        .value([label_of(&SCENES, scene)])
        .on_value_change(move |pressed| {
            Message::SetKitchenScene(chosen(&SCENES, &pressed).unwrap_or(scene))
        })
        .size(Size::Sm)
        .style(toggle_group::Outline);

    let preview = item::item([
        item::media(color_tile(oklch(0.72, 0.12, 65.0)))
            .style(item::Image)
            .into(),
        item::content([
            item::title("Warm Island").into(),
            item::description("Dining scene - 2700K").into(),
        ])
        .into(),
    ])
    .style(item::Muted);

    card::card(
        column![
            card::header("Kitchen Island"),
            card::content(
                column![
                    modes,
                    preview,
                    column![text::small("Scenes").medium(), scenes,]
                        .spacing(Space::U2)
                        .width(Fill),
                    slider_row("Brightness", 72.0),
                    slider_row("Color Temp", 44.0),
                ]
                .spacing(Space::U4)
                .width(Fill)
            ),
        ]
        .spacing(Space::U4)
        .padding([16, 0])
        .width(Fill),
    )
    .width(Fill)
    .boxed()
}

const MODES: [(KitchenMode, &str); 3] = [
    (KitchenMode::Hue, "Hue"),
    (KitchenMode::Color, "Color"),
    (KitchenMode::Ambient, "Ambient"),
];

const SCENES: [(KitchenScene, &str); 4] = [
    (KitchenScene::Cooking, "Cooking"),
    (KitchenScene::Dining, "Dining"),
    (KitchenScene::Nightlight, "Nightlight"),
    (KitchenScene::Focus, "Focus"),
];

/// The toggle-group value of `current`.
fn label_of<T: PartialEq>(options: &[(T, &'static str)], current: T) -> &'static str {
    options
        .iter()
        .find(|(option, _)| *option == current)
        .map_or("", |(_, label)| label)
}

/// The option a single-mode toggle group pressed, if any.
fn chosen<T: Copy>(options: &[(T, &'static str)], pressed: &[String]) -> Option<T> {
    let pressed = pressed.first()?;

    options
        .iter()
        .find(|(_, label)| label == pressed)
        .map(|(option, _)| *option)
}

fn slider_row(label: &'static str, value: f32) -> Slot<'static, Message> {
    column![
        text::small(label).style(theme::text::Muted),
        slider(0.0..=100.0).default_value(value).width(Fill),
    ]
    .spacing(Space::U2)
    .width(Fill)
    .into()
}

fn color_tile(color: Color) -> Slot<'static, Message> {
    container(space::vertical())
        .width(Fill)
        .height(Fill)
        .style(move |_| theme::container::Style {
            background: Some(color.into()),
            border: Border {
                radius: 6.0.into(),
                ..Border::default()
            },
            ..Default::default()
        })
        .into()
}
