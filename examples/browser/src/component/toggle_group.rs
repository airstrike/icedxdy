// Intentional skips: toggle-group-font-weight-selector (a styled card
// layout around a group) and toggle-group-rtl (RTL).

use iced::Task;

use icedxdy::widget::{column, toggle_group};
use icedxdy::{Size, Theme, Vertical};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Toggle Group",
    description: "A set of two-state buttons that can be toggled on or off.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/toggle-group-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "outline",
                title: "Outline",
                description: Some("Use `.style(toggle_group::Outline)` for an outline style."),
                snippet: "examples/browser/snippets/toggle-group-outline.md",
            },
            page::Block::Example {
                id: "sizes",
                title: "Size",
                description: Some("Use `.size(..)` to change the size of the toggle group."),
                snippet: "examples/browser/snippets/toggle-group-sizes.md",
            },
            page::Block::Example {
                id: "spacing",
                title: "Spacing",
                description: Some("Use `.spacing(..)` to add space between the items."),
                snippet: "examples/browser/snippets/toggle-group-spacing.md",
            },
            page::Block::Example {
                id: "vertical",
                title: "Vertical",
                description: Some("Use `.orientation(Vertical)` for a vertical toggle group."),
                snippet: "examples/browser/snippets/toggle-group-vertical.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: None,
                snippet: "examples/browser/snippets/toggle-group-disabled.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/toggle-group-api-reference.md"),
];

pub struct ToggleGroupPage {
    state: page::State,
}

impl ToggleGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// Bold, italic and underline items.
fn formatting<'a>() -> [toggle_group::Item<'a, Message>; 3] {
    [
        toggle_group::item("bold", icon::bold()),
        toggle_group::item("italic", icon::italic()),
        toggle_group::item("underline", icon::underline()),
    ]
}

/// Top, bottom, left and right items.
fn sides<'a>() -> [toggle_group::Item<'a, Message>; 4] {
    ["Top", "Bottom", "Left", "Right"].map(|side| toggle_group::item(side.to_lowercase(), side))
}

impl page::Page for ToggleGroupPage {
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
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // shadcn `toggle-group-demo.tsx`.
            "demo" => toggle_group(formatting)
                .mode(toggle_group::Mode::Multiple)
                .style(toggle_group::Outline)
                .into(),

            // shadcn `toggle-group-outline.tsx`.
            "outline" => toggle_group(|| {
                [
                    toggle_group::item("all", "All"),
                    toggle_group::item("missed", "Missed"),
                ]
            })
            .default_value(["all"])
            .style(toggle_group::Outline)
            .into(),

            // shadcn `toggle-group-sizes.tsx`.
            "sizes" => column![
                toggle_group(sides)
                    .default_value(["top"])
                    .size(Size::Sm)
                    .style(toggle_group::Outline),
                toggle_group(sides)
                    .default_value(["top"])
                    .style(toggle_group::Outline),
            ]
            .spacing(16)
            .into(),

            // shadcn `toggle-group-spacing.tsx`: `spacing={2}` is 8px.
            "spacing" => toggle_group(sides)
                .default_value(["top"])
                .size(Size::Sm)
                .spacing(8)
                .style(toggle_group::Outline)
                .into(),

            // shadcn `toggle-group-vertical.tsx`: `spacing={1}` is 4px.
            "vertical" => toggle_group(formatting)
                .mode(toggle_group::Mode::Multiple)
                .default_value(["bold", "italic"])
                .orientation(Vertical)
                .spacing(4)
                .into(),

            // shadcn `toggle-group-disabled.tsx`.
            "disabled" => toggle_group(formatting)
                .mode(toggle_group::Mode::Multiple)
                .disabled(true)
                .into(),

            _ => unreachable!("unknown toggle group preview id: {id}"),
        }
    }
}
