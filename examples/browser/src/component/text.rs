use std::collections::BTreeSet;

use iced::widget::{Id, column, row};
use iced::{Length, Task};
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
            id: "demo",
            title: "Typography",
            description: None,
            snippet: "examples/browser/snippets/text-demo.md",
        },
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
            id: "inline-code",
            title: "Inline code",
            description: None,
            snippet: "examples/browser/snippets/text-inline-code.md",
        },
        example::Meta {
            id: "list",
            title: "List",
            description: None,
            snippet: "examples/browser/snippets/text-list.md",
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

    fn preview(&self, index: usize) -> iced::Element<'_, Message, Theme> {
        match index {
            // Demo: condensed combined preview mirroring shadcn's TypographyDemo.
            0 => column![
                text::h1("Taxing Laughter: The Joke Tax Chronicles"),
                text::lead(
                    "Once upon a time, in a far-off land, there was a very lazy king who \
                     spent all day lounging on his throne. One day, his advisors came to him \
                     with a problem: the kingdom was running out of money."
                ),
                text::h2("The King's Plan"),
                text::p(
                    "The king thought long and hard, and finally came up with a brilliant \
                     plan: he would tax the jokes in the kingdom."
                ),
                blockquote_card(
                    "\"After all,\" he said, \"everyone enjoys a good joke, so it's only \
                     fair that they should pay for the privilege.\""
                ),
                text::h3("The Joke Tax"),
                text::p(
                    "The king's subjects were not amused. They grumbled and complained, \
                     but the king was firm:"
                ),
                column![
                    text::list("1st level of puns: 5 gold coins"),
                    text::list("2nd level of jokes: 10 gold coins"),
                    text::list("3rd level of one-liners : 20 gold coins"),
                ]
                .spacing(8),
                text::p(
                    "The king, seeing how much happier his subjects were, realized the error \
                     of his ways and repealed the joke tax."
                ),
            ]
            .spacing(16)
            .into(),

            // h1
            1 => text::h1("Taxing Laughter: The Joke Tax Chronicles").into(),

            // h2
            2 => text::h2("The People of the Kingdom").into(),

            // h3
            3 => text::h3("The Joke Tax").into(),

            // h4
            4 => text::h4("People stopped telling jokes").into(),

            // p
            5 => text::p(
                "The king, seeing how much happier his subjects were, realized the error of \
                 his ways and repealed the joke tax."
            )
            .into(),

            // blockquote
            6 => blockquote_card(
                "\"After all,\" he said, \"everyone enjoys a good joke, so it's only fair \
                 that they should pay for the privilege.\""
            ),

            // inline-code
            7 => inline_code_card("@radix-ui/react-alert-dialog"),

            // list
            8 => column![
                text::list("1st level of puns: 5 gold coins"),
                text::list("2nd level of jokes: 10 gold coins"),
                text::list("3rd level of one-liners : 20 gold coins"),
            ]
            .spacing(8)
            .into(),

            // lead
            9 => text::lead(
                "A modal dialog that interrupts the user with important content and expects \
                 a response."
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

/// Compose blockquote's left-border + horizontal padding via row + Rule.
/// The Text widget can't model `border-l-2 pl-6` on its own, so the
/// decoration lives at the call site per `.claude/audit/text.md`. Italic
/// is intentionally dropped — adding `italic()` to Text is a separate
/// shape change.
fn blockquote_card<'a>(quote: &'a str) -> iced::Element<'a, Message, Theme> {
    row![
        rule::vertical(2),
        text::blockquote(quote),
    ]
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
