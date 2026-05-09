// TODO: API Reference section once a table widget lands.
//
// The live docs page defers the prop matrix to Radix's own docs; the page
// itself only demonstrates a single `Progress` rendered at 33. Captured
// rows: `Progress.value` (`Option<f32>` — current progress 0..=100; `None`
// for indeterminate) and `Progress.max` (`f32`, default `100`). Captured
// here so the gap is auditable when the table widget lands.
//
// Intentional skips per CLAUDE.md: progress-rtl (RTL).

use std::time::Duration;

use iced::{Fill, Task};
use icedxdy::Theme;
use icedxdy::widget::{column, container, progress, row, slider, space, text};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Progress",
    description: "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "",
    },
    page::Block::Md("examples/browser/snippets/progress-usage.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "label",
                title: "Label",
                description: Some("Wrap the progress bar in a `field(...)` to add a label."),
                snippet: "examples/browser/snippets/progress-label.md",
            },
            page::Block::Example {
                id: "controlled",
                title: "Controlled",
                description: Some("A progress bar that can be controlled by a slider."),
                snippet: "examples/browser/snippets/progress-controlled.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/progress-api-reference.md"),
];

/// Initial value before the delay fires (matches the 13 the shadcn demo
/// starts at).
const INITIAL_VALUE: f32 = 13.0;

/// Target value after the 500ms delay (matches the demo's 66 target).
const TARGET_VALUE: f32 = 66.0;

/// Delay before the indicator jumps to the target — mirrors the 500ms
/// timeout in `progress-demo.tsx`.
const DELAY_MS: u64 = 500;

/// Stable field ids for `ValueChanged` dispatch.
const F_HERO: usize = 100;
const F_CONTROLLED: usize = 1;

pub struct ProgressPage {
    state: page::State,
    /// Current displayed value for the hero bar.
    value: f32,
    /// Slider-driven value for the Controlled example.
    controlled_value: f32,
}

impl ProgressPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            value: INITIAL_VALUE,
            controlled_value: 50.0,
        })
    }
}

#[granita::reloadable]
impl page::Page for ProgressPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn on_visible(&mut self) -> Task<Message> {
        // Schedule the hero's 500ms delayed jump to 66.
        let pi = self.state.page_index;
        Task::perform(
            async {
                tokio::time::sleep(Duration::from_millis(DELAY_MS)).await;
            },
            move |_| Message::Page(pi, page::Message::ValueChanged(F_HERO, TARGET_VALUE)),
        )
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        match event {
            page::Message::ValueChanged(F_HERO, v) => {
                self.value = v;
                Task::none()
            }
            page::Message::ValueChanged(F_CONTROLLED, v) => {
                self.controlled_value = v;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match id {
            // Demo (hero): mirrors nova `progress-demo.tsx`. The bar caps
            // at 450px so it doesn't span the full hero card on wide
            // viewports.
            "demo" => container(progress(self.value).width(Fill))
                .max_width(450)
                .width(Fill)
                .into(),

            // Label: an inline label row above a static 66% progress bar.
            "label" => column![
                row![
                    text::label("Upload progress"),
                    space::horizontal(),
                    text::label("66%"),
                ]
                .spacing(0),
                progress(66.0).width(320),
            ]
            .spacing(8)
            .width(320)
            .into(),

            // Controlled: progress bar driven by an iced stock slider.
            "controlled" => column![
                progress(self.controlled_value).width(320),
                slider(0.0..=100.0, self.controlled_value, move |v| {
                    Message::Page(pi, page::Message::ValueChanged(F_CONTROLLED, v))
                })
                .step(1.0_f32)
                .width(320),
            ]
            .spacing(16)
            .into(),

            _ => unreachable!("unknown progress preview id: {id}"),
        }
    }
}
