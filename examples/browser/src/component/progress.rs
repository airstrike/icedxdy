use std::collections::BTreeSet;
use std::time::Duration;

use iced::Task;
use iced::time::{self, Instant};
use iced::widget::{Id, slider};
use iced::{Length, Subscription};
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
/// `setTimeout(..., 500)` in `progress-demo.tsx`.
const DELAY_MS: u64 = 500;

/// Tailwind's `transition-all` default duration. This is the time we
/// take to interpolate `INITIAL_VALUE → TARGET_VALUE`.
const TRANSITION_MS: u64 = 150;

/// Token identifying the demo example's delayed-trigger event. Routed via
/// `page::Message::DelayedTrigger(ID_DEMO)` so future examples can claim
/// their own ids without colliding.
const ID_DEMO: usize = 0;
const ID_CONTROLLED: usize = 1;
const ID_LABEL: usize = 2;

pub struct ProgressPage {
    state: page::State,
    /// Current displayed value for the Demo bar.
    value: f32,
    /// When the transition began. `None` until the 500ms delay fires; once
    /// `Some`, every animation tick interpolates from `INITIAL_VALUE` to
    /// `TARGET_VALUE` over `TRANSITION_MS` from this instant.
    transition_started: Option<Instant>,
    /// Slider-driven value for the Controlled example.
    controlled_value: f32,
}

impl ProgressPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, &META),
            value: INITIAL_VALUE,
            transition_started: None,
            controlled_value: 50.0,
        })
    }

    /// Whether the eased transition is still in flight. Used to gate the
    /// subscription so we don't tick forever once the bar has settled.
    fn is_animating(&self) -> bool {
        match self.transition_started {
            None => false,
            Some(start) => start.elapsed() < Duration::from_millis(TRANSITION_MS),
        }
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
                self.transition_started = None;
                let pi = self.state.page_index;
                Task::perform(
                    async {
                        tokio::time::sleep(Duration::from_millis(DELAY_MS)).await;
                    },
                    move |_| Message::Page(pi, page::Message::DelayedTrigger(ID_DEMO)),
                )
            }
            page::Message::DelayedTrigger(ID_DEMO) => {
                if self.transition_started.is_none() {
                    self.transition_started = Some(Instant::now());
                }
                Task::none()
            }
            page::Message::ValueChanged(ID_CONTROLLED, v) => {
                self.controlled_value = v;
                Task::none()
            }
            page::Message::AnimationTick(_) => {
                if let Some(start) = self.transition_started {
                    let elapsed = start.elapsed().as_secs_f32() * 1000.0;
                    let total = TRANSITION_MS as f32;
                    let raw = (elapsed / total).clamp(0.0, 1.0);
                    let eased = ease_in_out(raw);
                    self.value = INITIAL_VALUE + (TARGET_VALUE - INITIAL_VALUE) * eased;
                    if raw >= 1.0 {
                        self.value = TARGET_VALUE;
                    }
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<page::Message> {
        if self.is_animating() {
            // 60fps frame ticks. We don't have access to `iced::window::frames`
            // without the `Tick` variant being driven by the windowing layer,
            // so a 16ms timer is the lowest-friction option that keeps the
            // 150ms transition feeling smooth (~9 frames).
            time::every(Duration::from_millis(16)).map(page::Message::AnimationTick)
        } else {
            Subscription::none()
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

/// Tailwind's `transition-all` easing — `cubic-bezier(0.4, 0, 0.2, 1)`,
/// a.k.a. CSS's `ease-in-out`. Implemented in closed form: solve
/// `bezier_x(t) = x` for the parametric `t` via Newton-Raphson, then
/// evaluate `bezier_y(t)`.
///
/// Control points (with the implicit P0 = (0, 0) and P3 = (1, 1)):
///   P1 = (0.4, 0), P2 = (0.2, 1)
///
/// The cubic Bezier basis with P0 = 0 / P3 = 1 reduces to:
///   B(t, c1, c2) = 3·(1−t)²·t·c1 + 3·(1−t)·t²·c2 + t³
///   B'(t, c1, c2) = 3·(1−t)²·c1 + 6·(1−t)·t·(c2 − c1) + 3·t²·(1 − c2)
fn ease_in_out(x: f32) -> f32 {
    const P1X: f32 = 0.4;
    const P1Y: f32 = 0.0;
    const P2X: f32 = 0.2;
    const P2Y: f32 = 1.0;

    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }

    // Newton-Raphson — quadratic convergence on this curve, ~4 iterations
    // gets us to f32 precision.
    let mut t = x;
    for _ in 0..6 {
        let bx = bezier(t, P1X, P2X);
        let dx = bezier_derivative(t, P1X, P2X);
        if dx.abs() < 1e-6 {
            break;
        }
        let next = t - (bx - x) / dx;
        if (next - t).abs() < 1e-6 {
            t = next;
            break;
        }
        t = next.clamp(0.0, 1.0);
    }

    bezier(t, P1Y, P2Y)
}

fn bezier(t: f32, c1: f32, c2: f32) -> f32 {
    let one_minus_t = 1.0 - t;
    3.0 * one_minus_t * one_minus_t * t * c1
        + 3.0 * one_minus_t * t * t * c2
        + t * t * t
}

fn bezier_derivative(t: f32, c1: f32, c2: f32) -> f32 {
    let one_minus_t = 1.0 - t;
    3.0 * one_minus_t * one_minus_t * c1
        + 6.0 * one_minus_t * t * (c2 - c1)
        + 3.0 * t * t * (1.0 - c2)
}
