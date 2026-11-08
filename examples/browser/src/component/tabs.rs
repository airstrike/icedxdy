// API Reference is rendered via the `tabs-api-reference.md` snippet at the
// bottom of BLOCKS.
//
// Intentional skips:
// - tabs-rtl — RTL is not in scope (per CLAUDE.md).
// - tabs-line, tabs-vertical, tabs-disabled, tabs-icons — the list's
//   `line` variant, vertical orientation and disabled triggers are not
//   ported yet.

use iced::Task;
use iced::Widget;

use icedxdy::card;
use icedxdy::widget::{column, tabs, text};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Tabs",
    description: "A set of layered sections of content, known as tab panels, that are \
                  displayed one at a time.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/tabs-demo.md",
    },
    page::Block::Md("examples/browser/snippets/tabs-api-reference.md"),
];

/// The tabs page. The demo is uncontrolled.
pub struct TabsPage {
    state: page::State,
}

impl TabsPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn panel<'a>(title: &'a str, description: &'a str, body: &'a str) -> Slot<'a, Message> {
    Slot::from(
        card(
            column![
                card::header(title).description(description),
                Slot::from(card::content(text::muted(body)).boxed()),
            ]
            .spacing(16)
            .padding([16, 0]),
        )
        .width(iced::Fill)
        .boxed(),
    )
}

impl page::Page for TabsPage {
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

    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event).unwrap_or_else(Task::none)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // shadcn `tabs-demo.tsx`.
            "demo" => tabs(|t| {
                column![
                    t.list([
                        t.trigger("overview", "Overview"),
                        t.trigger("analytics", "Analytics"),
                        t.trigger("reports", "Reports"),
                        t.trigger("settings", "Settings"),
                    ]),
                    t.content(
                        "overview",
                        panel(
                            "Overview",
                            "View your key metrics and recent project activity. Track \
                             progress across all your active projects.",
                            "You have 12 active projects and 3 pending tasks.",
                        ),
                    ),
                    t.content(
                        "analytics",
                        panel(
                            "Analytics",
                            "Track performance and user engagement metrics. Monitor trends \
                             and identify growth opportunities.",
                            "Page views are up 25% compared to last month.",
                        ),
                    ),
                    t.content(
                        "reports",
                        panel(
                            "Reports",
                            "Generate and download your detailed reports. Export data in \
                             multiple formats for analysis.",
                            "You have 5 reports ready and available to export.",
                        ),
                    ),
                    t.content(
                        "settings",
                        panel(
                            "Settings",
                            "Manage your account preferences and options. Customize your \
                             experience to fit your needs.",
                            "Configure notifications, security, and themes.",
                        ),
                    ),
                ]
                .spacing(8)
                .into()
            })
            .default_value("overview")
            .width(400)
            .into(),

            _ => unreachable!("unknown tabs preview id: {id}"),
        }
    }
}
