use iced::Element;
// TODO: API Reference section once a table widget lands.
//
// The live Typography page is a styling guide rather than a component-with-
// props page — it does not surface a structured API Reference table.
// Captured shapes: `h1`, `h2`, `h3`, `h4`, `p`, `blockquote`, `ul`/`li`,
// `code` (semantic HTML elements with Tailwind utility classes), plus
// `Lead`, `Large`, `Small`, `Muted` (utility helpers). Restyle via
// `.style(...)` with a `theme::text::*` variant. Captured here so the gap
// is auditable when the table widget lands.
//
// Intentional skips per CLAUDE.md: typography-rtl (RTL).

use iced::Widget;
use iced::widget::column;
use iced::{Fill, Task};
use icedxdy::Size::Sm;
use icedxdy::widget::{container, rule, text};
use icedxdy::{Theme, radius, theme};
use sweeten::widget::table;

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Typography",
    description: "Styles for headings, paragraphs, lists...etc.",
};

pub static BLOCKS: &[page::Block] = &[
    // Article hero: empty-title example so the Joke Tax article gets the
    // same card chrome every other page's hero uses. `render_example`
    // applies `min_height(PREVIEW_HEIGHT)` (not fixed height) for
    // empty-title cards, so the long article grows past 288px naturally
    // instead of clipping.
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "",
    },
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
    page::Block::Md("examples/browser/snippets/text-api-reference.md"),
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
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // Article hero — verbatim shadcn `typography-demo.tsx`.
            "demo" => column![
                Element::from(text::h1("Taxing Laughter: The Joke Tax Chronicles")),
                Element::from(text::lead(
                    "Once upon a time, in a far-off land, there was a very lazy king \
                     who spent all day lounging on his throne. One day, his advisors \
                     came to him with a problem: the kingdom was running out of money."
                )),
                Element::from(text::h2("The King's Plan")),
                Element::from(text::p(
                    "The king thought long and hard, and finally came up with a \
                     brilliant plan: he would tax the jokes in the kingdom."
                )),
                Element::from(text::blockquote(
                    "\"After all,\" he said, \"everyone enjoys a good joke, so it's \
                     only fair that they should pay for the privilege.\""
                )),
                Element::from(text::h3("The Joke Tax")),
                Element::from(text::p(
                    "The king's subjects were not amused. They grumbled and \
                     complained, but the king was firm:"
                )),
                Element::from(text::list([
                    "1st level of puns: 5 gold coins",
                    "2nd level of jokes: 10 gold coins",
                    "3rd level of one-liners : 20 gold coins",
                ])),
                Element::from(text::p(
                    "As a result, people stopped telling jokes, and the kingdom \
                     fell into a gloom. But there was one person who refused to let \
                     the king's foolishness get him down: a court jester named \
                     Jokester."
                )),
                Element::from(text::h3("Jokester's Revolt")),
                Element::from(text::p(
                    "Jokester began sneaking into the castle in the middle of the \
                     night and leaving jokes all over the place: under the king's \
                     pillow, in his soup, even in the royal toilet. The king was \
                     furious, but he couldn't seem to stop Jokester."
                )),
                Element::from(text::p(
                    "And then, one day, the people of the kingdom discovered that \
                     the jokes left by Jokester were so funny that they couldn't \
                     help but laugh. And once they started laughing, they couldn't \
                     stop."
                )),
                Element::from(text::h3("The People's Rebellion")),
                Element::from(text::p(
                    "The people of the kingdom, feeling uplifted by the laughter, \
                     started to tell jokes and puns again, and soon the entire \
                     kingdom was in on the joke."
                )),
                table_card(),
                Element::from(text::p(
                    "The king, seeing how much happier his subjects were, realized \
                     the error of his ways and repealed the joke tax. Jokester was \
                     declared a hero, and the kingdom lived happily ever after."
                )),
                Element::from(text::p(
                    "The moral of the story is: never underestimate the power of a \
                     good laugh and always be careful of bad ideas."
                )),
            ]
            .spacing(16)
            .width(Fill)
            .boxed(),

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
                 error of his ways and repealed the joke tax.",
            )
            .into(),

            "blockquote" => text::blockquote(
                "\"After all,\" he said, \"everyone enjoys a good joke, so it's only \
                 fair that they should pay for the privilege.\"",
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
                 expects a response.",
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
    column![Element::from(text::h2(content)), rule::horizontal(1)]
        .spacing(8)
        .boxed()
}

/// Wrap inline_code in a small container with bg-muted + padding + small
/// rounded corners — matches `bg-muted px-[0.3rem] py-[0.2rem] rounded`.
fn inline_code_card<'a>(code: &'a str) -> iced::Element<'a, Message, Theme> {
    container(text::inline_code(code))
        .padding([3, 5])
        .class(theme::container::CardMuted + radius(Sm))
        .into()
}

/// Render the two-column nova typography-table.
fn table_card<'a>() -> iced::Element<'a, Message, Theme> {
    let rows = &[
        ("Empty", "Overflowing"),
        ("Modest", "Satisfied"),
        ("Full", "Ecstatic"),
    ];

    let header = |label| Element::from(text::p(label).bold());

    let columns = [
        table::column(Some(header("King's Treasury")), |row: &(_, _)| {
            Element::from(text::p(row.0))
        })
        .width(Fill),
        table::column(Some(header("People's happiness")), |row: &(_, _)| {
            Element::from(text::p(row.1))
        })
        .width(Fill),
    ];

    table::table(columns, rows)
        .width(Fill)
        .padding_x(16)
        .padding_y(8)
        .separator(1)
        .border(1)
        .boxed()
}
