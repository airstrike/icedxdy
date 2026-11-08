// API Reference is rendered via the `collapsible-api-reference.md` snippet
// at the bottom of BLOCKS.
//
// Intentional skips:
// - collapsible-rtl — RTL is not in scope (per CLAUDE.md).
// - collapsible-settings — needs a numeric input group not ported yet.

use iced::Alignment::Center;
use iced::Fill;
use iced::Task;
use iced::Widget;

use icedxdy::card;
use icedxdy::core::{Border, Color};
use icedxdy::theme::container::Style;
use icedxdy::widget::{button, collapsible, column, container, row, space, text};
use icedxdy::{Size, Slot, Theme, radius};

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Collapsible",
    description: "An interactive component which expands/collapses a panel.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/collapsible-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: None,
                snippet: "examples/browser/snippets/collapsible-basic.md",
            },
            page::Block::Example {
                id: "file-tree",
                title: "File Tree",
                description: Some("Use nested collapsibles to build a file tree."),
                snippet: "examples/browser/snippets/collapsible-file-tree.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/collapsible-api-reference.md"),
];

/// The collapsible page. Every example is uncontrolled: each collapsible
/// keeps its open state in its own tree.
pub struct CollapsiblePage {
    state: page::State,
}

impl CollapsiblePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// A `rounded-md border px-4 py-2 text-sm` box.
fn bordered<'a>(content: impl Into<Slot<'a, Message>>) -> Slot<'a, Message> {
    container(content)
        .width(Fill)
        .padding([8, 16])
        .style(|theme: &Theme| Style {
            border: Border {
                width: 1.0,
                color: theme.neutral.border,
                radius: radius::MD.into(),
            },
            ..Style::default()
        })
        .into()
}

fn detail<'a>(title: &'a str, body: &'a str) -> Slot<'a, Message> {
    bordered(column![text::small(title).medium(), text::muted(body)])
}

/// One entry of the file tree: a folder is a nested collapsible.
enum Entry {
    File(&'static str),
    Folder(&'static str, &'static [Entry]),
}

const FILE_TREE: &[Entry] = &[
    Entry::Folder(
        "components",
        &[
            Entry::Folder(
                "ui",
                &[
                    Entry::File("button.tsx"),
                    Entry::File("card.tsx"),
                    Entry::File("dialog.tsx"),
                    Entry::File("input.tsx"),
                    Entry::File("select.tsx"),
                    Entry::File("table.tsx"),
                ],
            ),
            Entry::File("login-form.tsx"),
            Entry::File("register-form.tsx"),
        ],
    ),
    Entry::Folder(
        "lib",
        &[
            Entry::File("utils.ts"),
            Entry::File("cn.ts"),
            Entry::File("api.ts"),
        ],
    ),
    Entry::Folder(
        "hooks",
        &[
            Entry::File("use-media-query.ts"),
            Entry::File("use-debounce.ts"),
            Entry::File("use-local-storage.ts"),
        ],
    ),
    Entry::Folder(
        "types",
        &[Entry::File("index.d.ts"), Entry::File("api.d.ts")],
    ),
    Entry::Folder(
        "public",
        &[
            Entry::File("favicon.ico"),
            Entry::File("logo.svg"),
            Entry::File("images"),
        ],
    ),
    Entry::File("app.tsx"),
    Entry::File("layout.tsx"),
    Entry::File("globals.css"),
    Entry::File("package.json"),
    Entry::File("tsconfig.json"),
    Entry::File("README.md"),
    Entry::File(".gitignore"),
];

fn entry(entry: &'static Entry) -> Slot<'static, Message> {
    match entry {
        Entry::File(name) => button![icon::file(), *name]
            .size(Size::Sm)
            .width(Fill)
            .style(button::Link)
            .into(),
        Entry::Folder(name, children) => collapsible(move |c| {
            column![
                c.trigger(
                    button![
                        if c.is_open() {
                            icon::chevron_down()
                        } else {
                            icon::chevron_right()
                        },
                        icon::folder(),
                        *name
                    ]
                    .size(Size::Sm)
                    .width(Fill)
                    .style(button::Ghost),
                ),
                c.content(
                    container(column(children.iter().map(self::entry)).spacing(4))
                        .padding(iced::Padding::ZERO.top(4.0).left(20.0)),
                ),
            ]
            .into()
        })
        .width(Fill)
        .into(),
    }
}

impl page::Page for CollapsiblePage {
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
            // shadcn `collapsible-demo.tsx`.
            "demo" => collapsible(|c| {
                column![
                    container(
                        row![
                            text::small("Order #4189").semibold(),
                            space::horizontal(),
                            c.trigger(
                                button(icon::chevrons_up_down())
                                    .square()
                                    .style(button::Ghost),
                            ),
                        ]
                        .spacing(16)
                        .align_y(Center)
                        .width(Fill),
                    )
                    .padding([0, 16]),
                    bordered(row![
                        text::muted("Status"),
                        space::horizontal(),
                        text::small("Shipped").medium(),
                    ]),
                    c.content(
                        column![
                            detail("Shipping address", "100 Market St, San Francisco"),
                            detail("Items", "2x Studio Headphones"),
                        ]
                        .spacing(8),
                    ),
                ]
                .spacing(8)
                .into()
            })
            .width(350)
            .into(),

            // shadcn `collapsible-basic.tsx`: the panel tints `bg-muted`
            // while open.
            "basic" => card(
                card::content(
                    collapsible(|c| {
                        let is_open = c.is_open();
                        let chevron = if is_open {
                            icon::chevron_up()
                        } else {
                            icon::chevron_down()
                        };

                        container(column![
                            c.trigger(
                                button!["Product details", space::horizontal(), chevron,]
                                    .width(Fill)
                                    .style(button::Ghost),
                            ),
                            c.content(
                                column![
                                    text::small(
                                        "This panel can be expanded or collapsed to reveal \
                                     additional content.",
                                    ),
                                    button("Learn More").size(Size::Xs),
                                ]
                                .spacing(8)
                                .padding([0, 10, 10, 10]),
                            ),
                        ])
                        .width(Fill)
                        .style(move |theme: &Theme| Style {
                            background: is_open.then(|| theme.muted.base.into()),
                            border: Border {
                                width: 0.0,
                                color: Color::TRANSPARENT,
                                radius: radius::MD.into(),
                            },
                            ..Style::default()
                        })
                        .into()
                    })
                    .width(Fill),
                )
                .boxed(),
            )
            .width(Fill.max(384.0))
            .boxed(),

            // shadcn `collapsible-file-tree.tsx`.
            "file-tree" => {
                card(card::content(column(FILE_TREE.iter().map(entry)).spacing(4)).boxed())
                    .width(Fill.max(256.0))
                    .boxed()
            }

            _ => unreachable!("unknown collapsible preview id: {id}"),
        }
    }
}
