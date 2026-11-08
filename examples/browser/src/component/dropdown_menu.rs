// API Reference is rendered via the `dropdown-menu-api-reference.md`
// snippet at the bottom of BLOCKS.
//
// Intentional skips:
// - dropdown-menu-rtl: RTL is not in scope (per CLAUDE.md).
// - dropdown-menu-avatar, -complex, -icons, -checkboxes-icons,
//   -radio-icons: the same parts with avatars and more icons; not ported
//   yet.

use iced::widget::component::{Component, component};
use iced::{Element, Task, Widget};

use icedxdy::widget::{button, column, dropdown_menu, menu, row, text};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Dropdown Menu",
    description: "Displays a menu to the user, such as a set of actions or functions, \
                  triggered by a button.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/dropdown-menu-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A basic dropdown menu with labels and separators."),
                snippet: "examples/browser/snippets/dropdown-menu-basic.md",
            },
            page::Block::Example {
                id: "submenu",
                title: "Submenu",
                description: Some("Use `menu::sub(..)` to nest secondary actions."),
                snippet: "examples/browser/snippets/dropdown-menu-submenu.md",
            },
            page::Block::Example {
                id: "shortcuts",
                title: "Shortcuts",
                description: Some("Add `.shortcut(..)` to show keyboard hints."),
                snippet: "examples/browser/snippets/dropdown-menu-shortcuts.md",
            },
            page::Block::Example {
                id: "checkboxes",
                title: "Checkboxes",
                description: Some("Use `menu::checkbox_item` for toggles."),
                snippet: "examples/browser/snippets/dropdown-menu-checkboxes.md",
            },
            page::Block::Example {
                id: "radio-group",
                title: "Radio Group",
                description: Some("Use `menu::radio_item` for exclusive choices."),
                snippet: "examples/browser/snippets/dropdown-menu-radio-group.md",
            },
            page::Block::Example {
                id: "destructive",
                title: "Destructive",
                description: Some("Use `.style(menu::Destructive)` for irreversible actions."),
                snippet: "examples/browser/snippets/dropdown-menu-destructive.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/dropdown-menu-api-reference.md"),
];

pub struct DropdownMenuPage {
    state: page::State,
}

impl DropdownMenuPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn open<'a, M: Clone + 'a>() -> impl Fn(&dropdown_menu::Handle) -> Slot<'a, M> {
    |m| m.trigger(button("Open").style(button::Outline))
}

/// An item with a leading icon.
fn with_icon<'a, Message: 'a>(
    glyph: iced::widget::Text<'a, Theme>,
    label: &'a str,
) -> Slot<'a, Message> {
    row![glyph.size(16), text(label)]
        .spacing(6)
        .align_y(iced::Center)
        .into()
}

/// shadcn `dropdown-menu-checkboxes.tsx`: three toggles the menu shows and
/// flips, held by a user `Component` rather than app state.
struct Appearance;

impl<'a> Component<'a, Message, Theme> for Appearance {
    type State = Option<[bool; 3]>;
    type Event = (usize, bool);

    fn update(
        &self,
        state: &mut Option<[bool; 3]>,
        (index, checked): (usize, bool),
        _renderer: &iced::Renderer,
    ) -> Option<Message> {
        let mut shown = state.unwrap_or([true, false, false]);
        shown[index] = checked;
        *state = Some(shown);
        None
    }

    fn view(&self, state: &Option<[bool; 3]>) -> impl Widget<(usize, bool), Theme> + 'a {
        let [status, activity, panel] = state.unwrap_or([true, false, false]);

        Element::from(
            dropdown_menu(open(), move |_| {
                menu::group([
                    menu::label("Appearance"),
                    menu::checkbox_item("Status Bar", status, |c| (0, c)).into(),
                    menu::checkbox_item("Activity Bar", activity, |c| (1, c))
                        .disabled(true)
                        .into(),
                    menu::checkbox_item("Panel", panel, |c| (2, c)).into(),
                ])
            })
            .width(160),
        )
    }
}

/// shadcn `dropdown-menu-radio-group.tsx`: the panel position, held by a
/// user `Component`.
struct Position;

impl<'a> Component<'a, Message, Theme> for Position {
    type State = Option<&'static str>;
    type Event = &'static str;

    fn update(
        &self,
        state: &mut Option<&'static str>,
        position: &'static str,
        _renderer: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(position);
        None
    }

    fn view(&self, state: &Option<&'static str>) -> impl Widget<&'static str, Theme> + 'a {
        let position = state.unwrap_or("bottom");

        Element::from(
            dropdown_menu(open(), move |_| {
                menu::group([
                    menu::label("Panel Position"),
                    menu::radio_item("Top", "top", Some(position), |p| p).into(),
                    menu::radio_item("Bottom", "bottom", Some(position), |p| p).into(),
                    menu::radio_item("Right", "right", Some(position), |p| p).into(),
                ])
            })
            .width(128),
        )
    }
}

impl page::Page for DropdownMenuPage {
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
            // shadcn `dropdown-menu-demo.tsx`.
            "demo" => dropdown_menu(open(), |_| {
                column![
                    menu::group([
                        menu::label("My Account"),
                        menu::item("Profile").shortcut("⇧⌘P").into(),
                        menu::item("Billing").shortcut("⌘B").into(),
                        menu::item("Settings").shortcut("⌘S").into(),
                    ]),
                    menu::separator(),
                    menu::group([
                        menu::item("Team").into(),
                        menu::sub(
                            "Invite users",
                            column![
                                menu::item("Email"),
                                menu::item("Message"),
                                menu::separator(),
                                menu::item("More..."),
                            ],
                        ),
                        menu::item("New Team").shortcut("⌘+T").into(),
                    ]),
                    menu::separator(),
                    menu::group([
                        menu::item("GitHub").into(),
                        menu::item("Support").into(),
                        menu::item("API").disabled(true).into(),
                    ]),
                    menu::separator(),
                    menu::item("Log out").shortcut("⇧⌘Q"),
                ]
                .into()
            })
            .align(dropdown_menu::Align::Start)
            .width(160)
            .into(),

            // shadcn `dropdown-menu-basic.tsx`.
            "basic" => dropdown_menu(open(), |_| {
                column![
                    menu::group([
                        menu::label("My Account"),
                        menu::item("Profile").into(),
                        menu::item("Billing").into(),
                        menu::item("Settings").into(),
                    ]),
                    menu::separator(),
                    menu::item("GitHub"),
                    menu::item("Support"),
                    menu::item("API").disabled(true),
                ]
                .into()
            })
            .into(),

            // shadcn `dropdown-menu-submenu.tsx`.
            "submenu" => dropdown_menu(open(), |_| {
                menu::group([
                    menu::item("Team").into(),
                    menu::sub(
                        "Invite users",
                        column![
                            menu::item("Email"),
                            menu::item("Message"),
                            menu::sub(
                                "More options",
                                column![
                                    menu::item("Calendly"),
                                    menu::item("Slack"),
                                    menu::separator(),
                                    menu::item("Webhook"),
                                ],
                            ),
                            menu::separator(),
                            menu::item("Advanced..."),
                        ],
                    ),
                    menu::item("New Team").shortcut("⌘+T").into(),
                ])
            })
            .into(),

            // shadcn `dropdown-menu-shortcuts.tsx`.
            "shortcuts" => dropdown_menu(open(), |_| {
                column![
                    menu::group([
                        menu::label("My Account"),
                        menu::item("Profile").shortcut("⇧⌘P").into(),
                        menu::item("Billing").shortcut("⌘B").into(),
                        menu::item("Settings").shortcut("⌘S").into(),
                    ]),
                    menu::separator(),
                    menu::item("Log out").shortcut("⇧⌘Q"),
                ]
                .into()
            })
            .into(),

            "checkboxes" => component(Appearance).boxed(),

            "radio-group" => component(Position).boxed(),

            // shadcn `dropdown-menu-destructive.tsx`.
            "destructive" => dropdown_menu(
                |m| m.trigger(button("Actions").style(button::Outline)),
                |_| {
                    column![
                        menu::group([
                            menu::item(with_icon(icon::pencil(), "Edit")).into(),
                            menu::item(with_icon(icon::share(), "Share")).into(),
                        ]),
                        menu::separator(),
                        menu::item(with_icon(icon::trash_2(), "Delete")).style(menu::Destructive),
                    ]
                    .into()
                },
            )
            .into(),

            _ => unreachable!("unknown dropdown menu preview id: {id}"),
        }
    }
}
