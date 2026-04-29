use std::collections::BTreeSet;
use std::time::Duration;

use iced::widget::Id;
use iced::{Fill, Task};
use icedxdy::Theme;
use icedxdy::widget::markdown;
use icedxdy::widget::{column, container, progress, row, slider, space, text};

use crate::Message;
use crate::example;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Progress",
    description: "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
    examples: &[
        example::Meta {
            id: "label",
            title: "Label",
            description: Some("Use a Field component to add a label to the progress bar."),
            snippet: "examples/browser/snippets/progress-label.md",
        },
        example::Meta {
            id: "controlled",
            title: "Controlled",
            description: Some("A progress bar that can be controlled by a slider."),
            snippet: "examples/browser/snippets/progress-controlled.md",
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

/// ID for the hero (demo) bar's value-change message. The hero isn't an
/// example card any more, but the same animation logic still drives it.
const ID_HERO: usize = 100;
const ID_LABEL: usize = 0;
const ID_CONTROLLED: usize = 1;

pub struct ProgressPage {
    state: page::State,
    /// Current displayed value for the hero bar. Set by `update()` when
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

    fn on_visible(&mut self) -> Task<Message> {
        // Schedule the hero's `setTimeout(setProgress(66), 500)` jump.
        // The widget eases the value-change from 13 to 66 internally over
        // 150ms via cubic-bezier(0.4, 0, 0.2, 1). On revisit, the page's
        // `value` is already at 66, so this fires idempotently — no animation.
        let pi = self.state.page_index;
        Task::perform(
            async {
                tokio::time::sleep(Duration::from_millis(DELAY_MS)).await;
            },
            move |_| Message::Page(pi, page::Message::ValueChanged(ID_HERO, TARGET_VALUE)),
        )
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        match event {
            page::Message::ValueChanged(ID_HERO, v) => {
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

    fn hero(&self) -> Option<page::Hero<'_, Message, Theme>> {
        // Mirrors nova `progress-demo.tsx`: `<Progress value={progress} className="w-[60%]" />`.
        // page.rs handles card chrome + centering. The bar caps at 450px
        // so it doesn't span the full hero card on wide viewports.
        let bar = container(progress(self.value).width(Fill))
            .max_width(450)
            .width(Fill);
        Some(page::Hero::Card(bar.into()))
    }

    fn preview(&self, index: usize) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match index {
            // Label: an inline label row above a static 66% progress bar.
            // No Field/FieldLabel widget yet — approximated with a Row that
            // spreads `text::small` ends apart via a fill space. The label
            // text picks up `.medium()` to match nova's `FieldLabel`
            // (`font-medium` per `radix-nova/ui/field.tsx`).
            ID_LABEL => column![
                row![
                    text::small("Upload progress").medium(),
                    space::horizontal(),
                    text::small("66%").medium(),
                ]
                .spacing(0),
                progress(66.0).width(320),
            ]
            .spacing(8)
            .width(320)
            .into(),
            // Controlled: progress bar driven by an iced stock slider. Width
            // 320px approximates Tailwind's `max-w-sm` (384px) within the
            // 450px preview area. Slider is iced's default — looks unstyled
            // until our own Slider widget lands.
            ID_CONTROLLED => column![
                progress(self.controlled_value).width(320),
                slider(0.0..=100.0, self.controlled_value, move |v| {
                    Message::Page(pi, page::Message::ValueChanged(ID_CONTROLLED, v))
                })
                .step(1.0)
                .width(320),
            ]
            .spacing(16)
            .into(),
            _ => unreachable!(),
        }
    }
}
