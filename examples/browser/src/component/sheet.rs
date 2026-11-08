// API Reference is rendered via the `sheet-api-reference.md` snippet at the
// bottom of BLOCKS.
//
// Intentional skips:
// - sheet-rtl — RTL is not in scope (per CLAUDE.md).

use iced::Task;

use icedxdy::widget::{button, column, field, input, row, sheet, text};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Sheet",
    description: "Extends the Dialog component to display content that complements the main \
                  content of the screen.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/sheet-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "side",
                title: "Side",
                description: Some(
                    "Use `.side(..)` to set the edge of the screen the sheet appears from.",
                ),
                snippet: "examples/browser/snippets/sheet-side.md",
            },
            page::Block::Example {
                id: "no-close-button",
                title: "No Close Button",
                description: Some("Use `.show_close_button(false)` to hide the close button."),
                snippet: "examples/browser/snippets/sheet-no-close-button.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/sheet-api-reference.md"),
];

/// The sheet page. Every sheet is uncontrolled.
pub struct SheetPage {
    state: page::State,
}

impl SheetPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// shadcn's "Edit profile" sheet, opened by an outline button.
fn profile<'a>(
    side: sheet::Side,
    label: &'static str,
    show_close_button: bool,
) -> Slot<'a, Message> {
    sheet(
        move |s| s.trigger(button(label).style(button::Outline)),
        |s| {
            column![
                sheet::header(
                    "Edit profile",
                    "Make changes to your profile here. Click save when you're done.",
                ),
                column![
                    field(input("").default_value("Pedro Duarte").width(iced::Fill)).label("Name"),
                    field(input("").default_value("@peduarte").width(iced::Fill)).label("Username"),
                ]
                .spacing(24)
                .padding([0, 16]),
                sheet::footer([
                    s.close(button("Save changes").width(iced::Fill)),
                    s.close(button("Close").style(button::Outline).width(iced::Fill)),
                ]),
            ]
            .height(iced::Fill)
            .into()
        },
    )
    .side(side)
    .show_close_button(show_close_button)
    .into()
}

impl page::Page for SheetPage {
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
            // shadcn `sheet-demo.tsx`.
            "demo" => profile(sheet::Side::Right, "Open", true).into(),

            // shadcn `sheet-side.tsx`.
            "side" => row![
                profile(sheet::Side::Top, "Top", true),
                profile(sheet::Side::Right, "Right", true),
                profile(sheet::Side::Bottom, "Bottom", true),
                profile(sheet::Side::Left, "Left", true),
            ]
            .spacing(8)
            .into(),

            // shadcn `sheet-no-close-button.tsx`.
            "no-close-button" => column![
                profile(sheet::Side::Right, "Open Sheet", false),
                text::muted("Dismiss with Escape, a press outside, or the footer buttons."),
            ]
            .spacing(8)
            .into(),

            _ => unreachable!("unknown sheet preview id: {id}"),
        }
    }
}
