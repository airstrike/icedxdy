use iced::widget::{column, row};
use iced::{Fill, Task};
use icedxdy::Size::Sm;
use icedxdy::widget::{container, rule, text};
use icedxdy::{Theme, radius, theme};

use crate::Message;
use crate::page;

// Intentional skips per CLAUDE.md: typography-rtl (RTL). Typography on
// shadcn does not expose an API Reference page (it is a collection of
// text helpers, not a single component) — match that here.
pub static META: page::Meta = page::Meta {
    title: "Typography",
    description: "Styles for headings, paragraphs, lists...etc.",
};

pub static BLOCKS: &[page::Block] = &[
    // Article hero: rendered as inline page-level prose with no card chrome.
    page::Block::Inline { id: "demo" },
    page::Block::Md("examples/browser/snippets/text-installation.md"),
    page::Block::Md("examples/browser/snippets/text-usage.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "h1",
                title: "Heading 1",
                description: None,
                snippet: "examples/browser/snippets/text-h1.md",
            },
            page::Block::Example {
                id: "h2",
                title: "Heading 2",
                description: None,
                snippet: "examples/browser/snippets/text-h2.md",
            },
            page::Block::Example {
                id: "h3",
                title: "Heading 3",
                description: None,
                snippet: "examples/browser/snippets/text-h3.md",
            },
            page::Block::Example {
                id: "h4",
                title: "Heading 4",
                description: None,
                snippet: "examples/browser/snippets/text-h4.md",
            },
            page::Block::Example {
                id: "p",
                title: "Paragraph",
                description: None,
                snippet: "examples/browser/snippets/text-p.md",
            },
            page::Block::Example {
                id: "blockquote",
                title: "Blockquote",
                description: None,
                snippet: "examples/browser/snippets/text-blockquote.md",
            },
            page::Block::Example {
                id: "table",
                title: "Table",
                description: None,
                snippet: "examples/browser/snippets/text-table.md",
            },
            page::Block::Example {
                id: "list",
                title: "List",
                description: None,
                snippet: "examples/browser/snippets/text-list.md",
            },
            page::Block::Example {
                id: "inline-code",
                title: "Inline code",
                description: None,
                snippet: "examples/browser/snippets/text-inline-code.md",
            },
            page::Block::Example {
                id: "lead",
                title: "Lead",
                description: None,
                snippet: "examples/browser/snippets/text-lead.md",
            },
            page::Block::Example {
                id: "large",
                title: "Large",
                description: None,
                snippet: "examples/browser/snippets/text-large.md",
            },
            page::Block::Example {
                id: "small",
                title: "Small",
                description: None,
                snippet: "examples/browser/snippets/text-small.md",
            },
            page::Block::Example {
                id: "muted",
                title: "Muted",
                description: None,
                snippet: "examples/browser/snippets/text-muted.md",
            },
        ],
    },
];

pub struct TextPage {
    state: page::State,
}

impl TextPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for TextPage {
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
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // Article hero — verbatim shadcn `typography-demo.tsx`. Rendered
            // as inline page-level prose (Block::Inline, no card chrome).
            "demo" => column![
                text::h1("Taxing Laughter: The Joke Tax Chronicles"),
                text::lead(
                    "Once upon a time, in a far-off land, there was a very lazy king \
                     who spent all day lounging on his throne. One day, his advisors \
                     came to him with a problem: the kingdom was running out of money."
                ),
                text::h2("The King's Plan"),
                text::p(
                    "The king thought long and hard, and finally came up with a \
                     brilliant plan: he would tax the jokes in the kingdom."
                ),
                text::blockquote(
                    "\"After all,\" he said, \"everyone enjoys a good joke, so it's \
                     only fair that they should pay for the privilege.\""
                ),
                text::h3("The Joke Tax"),
                text::p(
                    "The king's subjects were not amused. They grumbled and \
                     complained, but the king was firm:"
                ),
                text::list([
                    "1st level of puns: 5 gold coins",
                    "2nd level of jokes: 10 gold coins",
                    "3rd level of one-liners : 20 gold coins",
                ]),
                text::p(
                    "As a result, people stopped telling jokes, and the kingdom \
                     fell into a gloom. But there was one person who refused to let \
                     the king's foolishness get him down: a court jester named \
                     Jokester."
                ),
                text::h3("Jokester's Revolt"),
                text::p(
                    "Jokester began sneaking into the castle in the middle of the \
                     night and leaving jokes all over the place: under the king's \
                     pillow, in his soup, even in the royal toilet. The king was \
                     furious, but he couldn't seem to stop Jokester."
                ),
                text::p(
                    "And then, one day, the people of the kingdom discovered that \
                     the jokes left by Jokester were so funny that they couldn't \
                     help but laugh. And once they started laughing, they couldn't \
                     stop."
                ),
                text::h3("The People's Rebellion"),
                text::p(
                    "The people of the kingdom, feeling uplifted by the laughter, \
                     started to tell jokes and puns again, and soon the entire \
                     kingdom was in on the joke."
                ),
                table_card(),
                text::p(
                    "The king, seeing how much happier his subjects were, realized \
                     the error of his ways and repealed the joke tax. Jokester was \
                     declared a hero, and the kingdom lived happily ever after."
                ),
                text::p(
                    "The moral of the story is: never underestimate the power of a \
                     good laugh and always be careful of bad ideas."
                ),
            ]
            .spacing(16)
            .width(Fill)
            .into(),

            // h1 — `text-center` per nova `typography-h1.tsx`
            "h1" => container(text::h1("Taxing Laughter: The Joke Tax Chronicles"))
                .center_x(Fill)
                .into(),

            // h2 — `border-b pb-2` per nova `typography-h2.tsx`
            "h2" => h2_with_border("The People of the Kingdom"),

            "h3" => text::h3("The Joke Tax").into(),
            "h4" => text::h4("People stopped telling jokes").into(),

            "p" => text::p(
                "The king, seeing how much happier his subjects were, realized the \
                 error of his ways and repealed the joke tax."
            )
            .into(),

            "blockquote" => text::blockquote(
                "\"After all,\" he said, \"everyone enjoys a good joke, so it's only \
                 fair that they should pay for the privilege.\""
            )
            .into(),

            "table" => table_card(),

            "list" => text::list([
                "1st level of puns: 5 gold coins",
                "2nd level of jokes: 10 gold coins",
                "3rd level of one-liners : 20 gold coins",
            ])
            .into(),

            "inline-code" => inline_code_card("@radix-ui/react-alert-dialog"),

            "lead" => text::lead(
                "A modal dialog that interrupts the user with important content and \
                 expects a response."
            )
            .into(),

            "large" => text::large("Are you absolutely sure?").into(),
            "small" => text::small("Email address").medium().into(),
            "muted" => text::muted("Enter your email address.").into(),

            _ => unreachable!("unknown text preview id: {id}"),
        }
    }
}

/// Wrap an h2 with a 1px horizontal rule underneath — the iced equivalent
/// of nova's `border-b pb-2 ...` cascade on `<h2>`.
fn h2_with_border<'a>(content: &'a str) -> iced::Element<'a, Message, Theme> {
    column![text::h2(content), rule::horizontal(1)]
        .spacing(8)
        .into()
}

/// Wrap inline_code in a small container with bg-muted + padding + small
/// rounded corners — matches `bg-muted px-[0.3rem] py-[0.2rem] rounded`.
fn inline_code_card<'a>(code: &'a str) -> iced::Element<'a, Message, Theme> {
    container(text::inline_code(code))
        .padding([3, 5])
        .class(theme::container::CardMuted + radius(Sm))
        .into()
}

/// Render a small two-column table approximating nova's typography-table.
fn table_card<'a>() -> iced::Element<'a, Message, Theme> {
    let cell = |s: &'a str| container(text::p(s)).width(Fill).padding([8, 16]);
    let header_cell =
        |s: &'a str| container(text::p(s).bold()).width(Fill).padding([8, 16]);

    column![
        rule::horizontal(1),
        row![header_cell("King's Treasury"), header_cell("People's happiness")],
        rule::horizontal(1),
        row![cell("Empty"), cell("Overflowing")],
        rule::horizontal(1),
        row![cell("Modest"), cell("Satisfied")],
        rule::horizontal(1),
        row![cell("Full"), cell("Ecstatic")],
        rule::horizontal(1),
    ]
    .width(Fill)
    .into()
}
