use std::collections::BTreeSet;

use iced::padding;
use iced::widget::{Id, column, row};
use iced::{Fill, Length, Task};
use icedxdy::widget::markdown;
use icedxdy::widget::{container, rule, text};
use icedxdy::Size::Sm;
use icedxdy::{radius, theme, Theme};

use crate::example;
use crate::page;
use crate::Message;

pub static META: page::Meta = page::Meta {
    title: "Typography",
    description: "Styles for headings, paragraphs, lists...etc.",
    examples: &[
        example::Meta {
            id: "h1",
            title: "Heading 1",
            description: None,
            snippet: "examples/browser/snippets/text-h1.md",
        },
        example::Meta {
            id: "h2",
            title: "Heading 2",
            description: None,
            snippet: "examples/browser/snippets/text-h2.md",
        },
        example::Meta {
            id: "h3",
            title: "Heading 3",
            description: None,
            snippet: "examples/browser/snippets/text-h3.md",
        },
        example::Meta {
            id: "h4",
            title: "Heading 4",
            description: None,
            snippet: "examples/browser/snippets/text-h4.md",
        },
        example::Meta {
            id: "p",
            title: "Paragraph",
            description: None,
            snippet: "examples/browser/snippets/text-p.md",
        },
        example::Meta {
            id: "blockquote",
            title: "Blockquote",
            description: None,
            snippet: "examples/browser/snippets/text-blockquote.md",
        },
        example::Meta {
            id: "table",
            title: "Table",
            description: None,
            snippet: "examples/browser/snippets/text-table.md",
        },
        example::Meta {
            id: "list",
            title: "List",
            description: None,
            snippet: "examples/browser/snippets/text-list.md",
        },
        example::Meta {
            id: "inline-code",
            title: "Inline code",
            description: None,
            snippet: "examples/browser/snippets/text-inline-code.md",
        },
        example::Meta {
            id: "lead",
            title: "Lead",
            description: None,
            snippet: "examples/browser/snippets/text-lead.md",
        },
        example::Meta {
            id: "large",
            title: "Large",
            description: None,
            snippet: "examples/browser/snippets/text-large.md",
        },
        example::Meta {
            id: "small",
            title: "Small",
            description: None,
            snippet: "examples/browser/snippets/text-small.md",
        },
        example::Meta {
            id: "muted",
            title: "Muted",
            description: None,
            snippet: "examples/browser/snippets/text-muted.md",
        },
    ],
};

pub struct TextPage {
    state: page::State,
}

impl TextPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, &META),
        })
    }
}

impl page::Page for TextPage {
    fn page_index(&self) -> usize { self.state.page_index }
    fn meta(&self) -> &'static page::Meta {
        &META
    }

    fn snippets(&self) -> &[Option<Vec<markdown::Item>>] {
        &self.state.snippets
    }

    fn visible(&self) -> &BTreeSet<usize> {
        &self.state.visible
    }

    fn example_ids(&self) -> &[Id] {
        &self.state.example_ids
    }

    fn scrollable_id(&self) -> Id {
        self.state.scrollable_id.clone()
    }

    fn code_expanded(&self) -> &BTreeSet<usize> {
        &self.state.code_expanded
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

    /// Typography hero. Per the shadcn docs page convention, the
    /// `typography-demo.tsx` article is rendered as inline page-level prose
    /// above the "Examples" h2 — NOT as a labeled card. The body composes
    /// the joke-tax tale verbatim using our text helpers; the embedded link
    /// "a brilliant plan" is rendered as flat prose because the iced text
    /// widget doesn't natively model inline `<a>` styling without
    /// rich_text plumbing that would balloon this page.
    fn hero(&self) -> Option<page::Hero<'_, Message, Theme>> {
        Some(page::Hero::Inline(
            column![
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
                blockquote_card(
                    "\"After all,\" he said, \"everyone enjoys a good joke, so it's \
                     only fair that they should pay for the privilege.\""
                ),
                text::h3("The Joke Tax"),
                text::p(
                    "The king's subjects were not amused. They grumbled and \
                     complained, but the king was firm:"
                ),
                indented_list(&[
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
        ))
    }

    fn preview(&self, index: usize) -> iced::Element<'_, Message, Theme> {
        match index {
            // h1 — `text-center` per nova `typography-h1.tsx`
            0 => container(text::h1("Taxing Laughter: The Joke Tax Chronicles"))
                .center_x(Fill)
                .into(),

            // h2 — `border-b pb-2` per nova `typography-h2.tsx`
            1 => h2_with_border("The People of the Kingdom"),

            // h3
            2 => text::h3("The Joke Tax").into(),

            // h4
            3 => text::h4("People stopped telling jokes").into(),

            // p
            4 => text::p(
                "The king, seeing how much happier his subjects were, realized the \
                 error of his ways and repealed the joke tax."
            )
            .into(),

            // blockquote
            5 => blockquote_card(
                "\"After all,\" he said, \"everyone enjoys a good joke, so it's only \
                 fair that they should pay for the privilege.\""
            ),

            // table
            6 => table_card(),

            // list — `ml-6` (24px left indent) per nova `typography-list.tsx`
            7 => indented_list(&[
                "1st level of puns: 5 gold coins",
                "2nd level of jokes: 10 gold coins",
                "3rd level of one-liners : 20 gold coins",
            ]),

            // inline-code
            8 => inline_code_card("@radix-ui/react-alert-dialog"),

            // lead
            9 => text::lead(
                "A modal dialog that interrupts the user with important content and \
                 expects a response."
            )
            .into(),

            // large
            10 => text::large("Are you absolutely sure?").into(),

            // small
            11 => text::small("Email address").medium().into(),

            // muted
            12 => text::muted("Enter your email address.").into(),

            _ => unreachable!(),
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

/// Compose blockquote's left-border + horizontal padding via row + Rule.
/// The Text widget can't model `border-l-2 pl-6` on its own, so the
/// decoration lives at the call site per `.claude/audit/text.md`. Italic
/// is now applied inside `text::blockquote` itself via `font::Style::Italic`.
fn blockquote_card<'a>(quote: &'a str) -> iced::Element<'a, Message, Theme> {
    row![rule::vertical(2), text::blockquote(quote)]
        .spacing(24)
        .height(Length::Shrink)
        .into()
}

/// Wrap inline_code in a small container with bg-muted + padding + small
/// rounded corners — matches `bg-muted px-[0.3rem] py-[0.2rem] rounded`
/// from shadcn.
fn inline_code_card<'a>(code: &'a str) -> iced::Element<'a, Message, Theme> {
    container(text::inline_code(code))
        .padding([3, 5])
        .style(theme::container::CardMuted + radius(Sm))
        .into()
}

/// Render a bulleted list with nova's `ml-6` (24px) left indent. Wraps
/// the column of `text::list` items in a container with a 24px left
/// padding, since `text::list` itself is flush-left.
fn indented_list<'a>(items: &'a [&'a str]) -> iced::Element<'a, Message, Theme> {
    let body = column(items.iter().map(|s| text::list(*s).into())).spacing(8);
    container(body).padding(padding::left(24)).into()
}

/// Render a small two-column table approximating nova's typography-table.
/// Composes a column of `row![left, right]` with horizontal rules between
/// rows — iced has no native table widget, and shadcn's table is just a
/// styled `<table>` with `border` cells.
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
