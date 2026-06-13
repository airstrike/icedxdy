//! Preview-02 kitchen-island card.
//!
//! Source: shadcn `registry/bases/radix/blocks/preview-02/cards/kitchen-island.tsx`.

use iced::Fill;
use icedxdy::card;
use icedxdy::core::{Border, Color, oklch};
use icedxdy::theme;
use icedxdy::widget::{Size, column, container, item, slider, space, text, toggle_group};
use icedxdy::{Component, Space, Theme};

use crate::{KitchenMode, KitchenScene, Message, State};

#[granita::preview(state = State::dark(), group = "preview-02")]
pub fn view(state: &State) -> iced::Element<'static, Message, Theme> {
    let modes = toggle_group([
        mode_item(state, KitchenMode::Hue, "Hue"),
        mode_item(state, KitchenMode::Color, "Color"),
        mode_item(state, KitchenMode::Ambient, "Ambient"),
    ])
    .variant(toggle_group::Variant::Outline);

    let scenes = toggle_group([
        scene_item(state, KitchenScene::Cooking, "Cooking"),
        scene_item(state, KitchenScene::Dining, "Dining"),
        scene_item(state, KitchenScene::Nightlight, "Nightlight"),
        scene_item(state, KitchenScene::Focus, "Focus"),
    ])
    .size(Size::Sm)
    .variant(toggle_group::Variant::Outline);

    let preview = item::muted([
        item::media_image(color_tile(oklch(0.72, 0.12, 65.0))).into(),
        item::content([
            item::title("Warm Island").into(),
            item::description("Dining scene - 2700K").into(),
        ])
        .into(),
    ]);

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
    .into()
}

fn mode_item(
    state: &State,
    mode: KitchenMode,
    label: &'static str,
) -> toggle_group::Item<'static, Message> {
    toggle_group::item(label)
        .selected(state.kitchen_mode == mode)
        .on_press(Message::SetKitchenMode(mode))
}

fn scene_item(
    state: &State,
    scene: KitchenScene,
    label: &'static str,
) -> toggle_group::Item<'static, Message> {
    toggle_group::item(label)
        .selected(state.kitchen_scene == scene)
        .on_press(Message::SetKitchenScene(scene))
}

fn slider_row(label: &'static str, value: f32) -> Component<'static, Message> {
    column![
        text::small(label).style(theme::text::Muted),
        slider(0.0..=100.0, value, |_| Message::Noop).width(Fill),
    ]
    .spacing(Space::U2)
    .width(Fill)
    .into()
}

fn color_tile(color: Color) -> Component<'static, Message> {
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
