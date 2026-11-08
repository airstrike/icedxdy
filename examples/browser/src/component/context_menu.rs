// API Reference is rendered via the `context-menu-api-reference.md` snippet
// at the bottom of BLOCKS.
//
// The trigger area's `border-dashed` draws solid: iced has no dashed
// borders.
//
// Intentional skips:
// - context-menu-rtl: RTL is not in scope (per CLAUDE.md).
// - context-menu-checkboxes, -destructive, -groups, -icons, -radio,
//   -shortcuts, -sides, -submenu: the same `menu::` parts the demo shows;
//   not ported one by one yet.

use iced::widget::component::{Component, component};
use iced::{Element, Fill, Task, Widget};

use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::{column, container, context_menu, menu, text};
use icedxdy::{Slot, Theme, radius};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Context Menu",
    description: "Displays a menu to the user, such as a set of actions or functions, \
                  triggered by a right click.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/context-menu-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[page::Block::Example {
            id: "basic",
            title: "Basic",
            description: Some("A simple context menu with a few actions."),
            snippet: "examples/browser/snippets/context-menu-basic.md",
        }],
    },
    page::Block::Md("examples/browser/snippets/context-menu-api-reference.md"),
];

pub struct ContextMenuPage {
    state: page::State,
}

impl ContextMenuPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// The `aspect-video max-w-xs rounded-xl border border-dashed` area.
fn area<'a, M: 'a>() -> Slot<'a, M> {
    container(text("Right click here"))
        .center(Fill)
        .width(320)
        .height(180)
        .style(|theme: &Theme| Style {
            border: Border {
                width: 1.0,
                color: theme.neutral.border,
                radius: radius::XL.into(),
            },
            ..Style::default()
        })
        .into()
}

/// The demo's toggles and person, held by a user `Component`.
#[derive(Debug, Clone, Copy)]
enum Choice {
    Bookmarks(bool),
    FullUrls(bool),
    Person(&'static str),
}

struct Demo;

impl<'a> Component<'a, Message, Theme> for Demo {
    type State = Option<(bool, bool, &'static str)>;
    type Event = Choice;

    fn update(
        &self,
        state: &mut Option<(bool, bool, &'static str)>,
        choice: Choice,
        _renderer: &iced::Renderer,
    ) -> Option<Message> {
        let (mut bookmarks, mut full_urls, mut person) = state.unwrap_or((true, false, "pedro"));

        match choice {
            Choice::Bookmarks(on) => bookmarks = on,
            Choice::FullUrls(on) => full_urls = on,
            Choice::Person(who) => person = who,
        }

        *state = Some((bookmarks, full_urls, person));
        None
    }

    fn view(&self, state: &Option<(bool, bool, &'static str)>) -> impl Widget<Choice, Theme> + 'a {
        let (bookmarks, full_urls, person) = state.unwrap_or((true, false, "pedro"));

        Element::from(
            context_menu(
                |_| area(),
                move |_| {
                    column![
                        menu::group([
                            menu::item("Back").shortcut("⌘[").into(),
                            menu::item("Forward").shortcut("⌘]").disabled(true).into(),
                            menu::item("Reload").shortcut("⌘R").into(),
                            menu::sub(
                                "More Tools",
                                [
                                    menu::item("Save Page...").into(),
                                    menu::item("Create Shortcut...").into(),
                                    menu::item("Name Window...").into(),
                                    menu::separator(),
                                    menu::item("Developer Tools").into(),
                                    menu::separator(),
                                    menu::item("Delete").style(menu::Destructive).into(),
                                ],
                            ),
                        ]),
                        menu::separator(),
                        menu::group([
                            menu::checkbox_item("Show Bookmarks", bookmarks, Choice::Bookmarks)
                                .into(),
                            menu::checkbox_item("Show Full URLs", full_urls, Choice::FullUrls)
                                .into(),
                        ]),
                        menu::separator(),
                        menu::group([
                            menu::label("People"),
                            menu::radio_item("Pedro Duarte", "pedro", Some(person), Choice::Person)
                                .into(),
                            menu::radio_item("Colm Tuite", "colm", Some(person), Choice::Person)
                                .into(),
                        ]),
                    ]
                    .width(Fill)
                    .into()
                },
            )
            .width(192),
        )
    }
}

impl page::Page for ContextMenuPage {
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
            // shadcn `context-menu-demo.tsx`.
            "demo" => component(Demo).boxed(),

            // shadcn `context-menu-basic.tsx`.
            "basic" => context_menu(
                |_| area(),
                |_| {
                    menu::group([
                        menu::item("Back").into(),
                        menu::item("Forward").disabled(true).into(),
                        menu::item("Reload").into(),
                    ])
                },
            )
            .into(),

            _ => unreachable!("unknown context menu preview id: {id}"),
        }
    }
}
