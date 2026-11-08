// Intentional skips: kbd-input-group (InputGroup is not ported) and
// kbd-tooltip (its tooltips sit inside a ButtonGroup, whose items are
// buttons; the keyboard tooltip is on the Tooltip page).

use iced::Task;

use icedxdy::Theme;
use icedxdy::widget::{button, column, kbd, kbd_group, row, text};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Kbd",
    description: "Used to display textual user input from keyboard.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/kbd-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "group",
                title: "Group",
                description: Some("Use `kbd_group!` to group keyboard keys together."),
                snippet: "examples/browser/snippets/kbd-group.md",
            },
            page::Block::Example {
                id: "button",
                title: "Button",
                description: Some("Use a `kbd` inside a `button` to display a keyboard key."),
                snippet: "examples/browser/snippets/kbd-button.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/kbd-api-reference.md"),
];

pub struct KbdPage {
    state: page::State,
}

impl KbdPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for KbdPage {
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
            // shadcn `kbd-demo.tsx`.
            "demo" => column![
                kbd_group![kbd("⌘"), kbd("⇧"), kbd("⌥"), kbd("⌃")],
                kbd_group![kbd("Ctrl"), text::small("+"), kbd("B")],
            ]
            .spacing(16)
            .align_x(iced::Center)
            .into(),

            // shadcn `kbd-group.tsx`.
            "group" => row![
                text::muted("Use"),
                kbd_group![kbd("Ctrl + B"), kbd("Ctrl + K")],
                text::muted("to open the command palette"),
            ]
            .spacing(4)
            .align_y(iced::Center)
            .into(),

            // shadcn `kbd-button.tsx`.
            "button" => button!["Accept", kbd("⏎")].style(button::Outline).into(),

            _ => unreachable!("unknown kbd preview id: {id}"),
        }
    }
}
