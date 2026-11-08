// The tooltip's arrow is drawn in half-pixel rows: iced cannot rotate the
// `size-2.5` square shadcn uses.

use iced::Task;

use icedxdy::widget::{button, kbd, row, text, tooltip};
use icedxdy::{Size, Slot, Theme};

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Tooltip",
    description: "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/tooltip-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "sides",
                title: "Side",
                description: Some("Use `.side(..)` to change the position of the tooltip."),
                snippet: "examples/browser/snippets/tooltip-sides.md",
            },
            page::Block::Example {
                id: "keyboard",
                title: "With Keyboard Shortcut",
                description: None,
                snippet: "examples/browser/snippets/tooltip-keyboard.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled Button",
                description: Some("Show a tooltip on a disabled button."),
                snippet: "examples/browser/snippets/tooltip-disabled.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/tooltip-api-reference.md"),
];

pub struct TooltipPage {
    state: page::State,
}

impl TooltipPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// An outline button showing "Add to library" on `side`.
fn side<'a>(label: &'a str, side: tooltip::Side) -> Slot<'a, Message> {
    tooltip(button(label).style(button::Outline), "Add to library")
        .side(side)
        .into()
}

impl page::Page for TooltipPage {
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
            // shadcn `tooltip-demo.tsx`.
            "demo" => tooltip(button("Hover").style(button::Outline), "Add to library").into(),

            // shadcn `tooltip-sides.tsx`.
            "sides" => row![
                side("Left", tooltip::Side::Left),
                side("Top", tooltip::Side::Top),
                side("Bottom", tooltip::Side::Bottom),
                side("Right", tooltip::Side::Right),
            ]
            .spacing(8)
            .into(),

            // shadcn `tooltip-keyboard.tsx`.
            "keyboard" => tooltip(
                button(icon::save())
                    .size(Size::Sm)
                    .square()
                    .style(button::Outline),
                row![
                    text::Text::new("Save Changes"),
                    kbd("S").style(kbd::Tooltip)
                ]
                .spacing(6)
                .align_y(iced::Center),
            )
            .into(),

            // shadcn `tooltip-disabled.tsx`.
            "disabled" => tooltip(
                button("Disabled").style(button::Outline).disabled(true),
                "This feature is currently unavailable",
            )
            .into(),

            _ => unreachable!("unknown tooltip preview id: {id}"),
        }
    }
}
