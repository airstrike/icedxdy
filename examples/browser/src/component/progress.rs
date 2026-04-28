use std::collections::BTreeSet;
use std::time::Duration;

use iced::Task;
use iced::widget::{Id, slider};
use iced::Length;
use icedxdy::Theme;
use icedxdy::widget::markdown;
use icedxdy::widget::{column, progress, row, text};

use crate::Message;
use crate::example;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Progress",
    description: "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
    examples: &[
        example::Meta {
            id: "demo",
            title: "Progress",
            description: None,
            snippet: "examples/browser/snippets/progress-demo.md",
        },
        example::Meta {
            id: "controlled",
            title: "Controlled",
            description: Some("A progress bar that can be controlled by a slider."),
            snippet: "examples/browser/snippets/progress-controlled.md",
        },
        example::Meta {
            id: "label",
            title: "Label",
            description: Some("Use a Field component to add a label to the progress bar."),
            snippet: "examples/browser/snippets/progress-label.md",
        },
    ],
};

/// Initial value while we wait for the delay-fire (matches React's
/// `useState(13)`).
const INITIAL_VALUE: f32 = 13.0;

/// Target value after the 500ms delay (matches `setProgress(66)`).
const TARGET_VALUE: f32 = 66.0;

/// Delay before the indicator jumps to the target — mirrors the
/// `setTimeout(..., 500)` in `progress-demo.tsx`. The widget owns the
/// 150ms easing from `INITIAL_VALUE` to `TARGET_VALUE` once we publish
/// the new value, so the page only schedules the jump itself.
const DELAY_MS: u64 = 500;

const ID_DEMO: usize = 0;
const ID_CONTROLLED: usize = 1;
const ID_LABEL: usize = 2;

pub struct ProgressPage {
    state: page::State,
    /// Current displayed value for the Demo bar. Set by `update()` when
    /// the delay fires; the widget eases from the previous value to this
    /// new value automatically.
    value: f32,
    /// Slider-driven value for the Controlled example.
    controlled_value: f32,
}

impl ProgressPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, &META),
            value: INITIAL_VALUE,
            controlled_value: 50.0,
        })
    }
}

impl page::Page for ProgressPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }

    fn meta(&self) -> &'static page::Meta {
        &META
    }

    fn snippets(&self) -> &[Option<Vec<markdown::Item>>] {
        &self.state.snippets
    }

    fn visible(&self) -> &BTreeSet<usize> {
        &self.state.visible
    }

    fn example_ids(&self) -> &[Id] {
        &self.state.example_ids
    }

    fn scrollable_id(&self) -> Id {
        self.state.scrollable_id.clone()
    }

    fn code_expanded(&self) -> &BTreeSet<usize> {
        &self.state.code_expanded
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        match event {
            page::Message::ExampleShown(ID_DEMO) => {
                self.value = INITIAL_VALUE;
                let pi = self.state.page_index;
                Task::perform(
                    async {
                        tokio::time::sleep(Duration::from_millis(DELAY_MS)).await;
                    },
                    move |_| {
                        Message::Page(
                            pi,
                            page::Message::ValueChanged(ID_DEMO, TARGET_VALUE),
                        )
                    },
                )
            }
            page::Message::ValueChanged(ID_DEMO, v) => {
                self.value = v;
                Task::none()
            }
            page::Message::ValueChanged(ID_CONTROLLED, v) => {
                self.controlled_value = v;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, index: usize) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match index {
            // Demo: nova's `<Progress value={progress} className="w-[60%]" />`.
            // The preview region is ~450px wide after the 40px padding, so
            // we model `w-[60%]` as a 270px-wide bar (60% of ~450px).
            ID_DEMO => progress(self.value).width(Length::Fixed(270.0)).into(),
            // Controlled: progress bar driven by an iced stock slider. Width
            // 320px approximates Tailwind's `max-w-sm` (384px) within the
            // 450px preview area. Slider is iced's default — looks unstyled
            // until our own Slider widget lands.
            ID_CONTROLLED => column([
                progress(self.controlled_value)
                    .width(Length::Fixed(320.0))
                    .into(),
                slider(0.0..=100.0, self.controlled_value, move |v| {
                    Message::Page(pi, page::Message::ValueChanged(ID_CONTROLLED, v))
                })
                .step(1.0)
                .width(320)
                .into(),
            ])
            .spacing(16)
            .into(),
            // Label: an inline label row above a static 66% progress bar.
            // No Field/FieldLabel widget yet — approximated with a Row that
            // spreads `text::small` ends apart via a fill space.
            ID_LABEL => column([
                row()
                    .push(text::small("Upload progress").into())
                    .push(iced_widget::space::horizontal().into())
                    .push(text::small("66%").into())
                    .spacing(0)
                    .into(),
                progress(66.0).width(Length::Fixed(320.0)).into(),
            ])
            .spacing(8)
            .width(Length::Fixed(320.0))
            .into(),
            _ => unreachable!(),
        }
    }
}
