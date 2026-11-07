// API Reference is rendered via the `card-api-reference.md` snippet at the
// bottom of BLOCKS. The live docs API Reference table covers `Card.size`
// (`Default` / `Sm`) plus the `Card`, `CardHeader`, `CardTitle`,
// `CardDescription`, `CardAction`, `CardContent`, `CardFooter` set; the
// snippet captures the icedxdy-shaped helper API (`card(...)`,
// `card::header(...).description(...).action(...)`, plus the section
// helpers).
//
// Intentional skips per CLAUDE.md: card-rtl (RTL).
//
// Scope notes:
// - The card-small example renders without a true `Sm`-size driver
//   (no `Card.size(Sm)` builder yet — out of scope this phase). Visual
//   delta vs the default size is small (text-base → text-sm on the title,
//   `gap-3 py-3 px-3` instead of `gap-4 py-4 px-4`); the example demonstrates
//   the same content shape shadcn ships.
// - The card-image example uses a muted-bg placeholder rectangle in lieu of
//   a raster asset (no checked-in image, no URL fetcher in iced) — same
//   stand-in `aspect_ratio.rs` adopted, with a 35%-black overlay layered on
//   top per shadcn's `absolute inset-0 z-30 aspect-video bg-black/35`.

use iced::Alignment::Center;
use iced::Fill;
use iced::Task;
use iced::Widget;
use iced::widget::stack;

use icedxdy::card;
use icedxdy::core::Border;
use icedxdy::core::Color;
use icedxdy::theme::container::Style;
use icedxdy::widget::{
    aspect_ratio, badge, button, column, container, field, input, row, space, text,
};
use icedxdy::{Gap, Size, Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Card",
    description: "Displays a card with header, content, and footer.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/card-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "small",
                title: "Size",
                description: Some(
                    "Use the size=\"sm\" prop to set the size of the card to small. The small size variant uses smaller spacing.",
                ),
                snippet: "examples/browser/snippets/card-small.md",
            },
            page::Block::Example {
                id: "image",
                title: "Image",
                description: Some(
                    "Add an image before the card header to create a card with an image.",
                ),
                snippet: "examples/browser/snippets/card-image.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/card-api-reference.md"),
];

// Stable input field ids (independent of block position so reordering blocks
// doesn't break value routing).
const F_DEMO_EMAIL: usize = 0;
const F_DEMO_PASSWORD: usize = 1;

pub struct CardPage {
    state: page::State,
    demo_email: String,
    demo_password: String,
}

impl CardPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            demo_email: String::new(),
            demo_password: String::new(),
        })
    }
}

/// Stand-in for the shadcn `<img>` shown above the card header. iced has
/// no built-in URL fetcher and the project ships no checked-in raster
/// asset, so the example uses a muted-bg rectangle — the same approach
/// `aspect_ratio.rs` adopted. The aspect ratio (16:9) and the 35% black
/// overlay above it are unaffected; that's what the example demonstrates.
fn image_placeholder<'a>() -> iced::Element<'a, Message, Theme> {
    container("")
        .width(Fill)
        .height(Fill)
        .style(|theme: &Theme| Style {
            background: Some(theme.muted.base.into()),
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..Style::default()
        })
        .into()
}

/// Translucent black tint layered on top of `image_placeholder`. Mirrors
/// shadcn's `absolute inset-0 z-30 aspect-video bg-black/35` — 35% alpha
/// over OKLCH black.
fn image_overlay<'a>() -> iced::Element<'a, Message, Theme> {
    container("")
        .width(Fill)
        .height(Fill)
        .style(|_t: &Theme| Style {
            background: Some(Color::BLACK.alpha(0.35).into()),
            ..Style::default()
        })
        .into()
}

impl page::Page for CardPage {
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
        match event {
            page::Message::InputChanged(field, value) => {
                match field {
                    F_DEMO_EMAIL => self.demo_email = value,
                    F_DEMO_PASSWORD => self.demo_password = value,
                    _ => {}
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match id {
            // Demo (hero): full Login form. Verbatim shadcn `card-demo.tsx`:
            // header (title + description + Sign Up link action), content
            // (Email + Password fields, Password row has a "Forgot your
            // password?" link tucked flush-right next to the label), footer
            // (Login + "Login with Google" buttons stacked column, full-width).
            // max-w-sm = 24rem = 384px.
            "demo" => {
                let email_field: Slot<'_, Message> = field(
                    input("m@example.com").value(&self.demo_email)
                        .on_input(move |v| {
                            Message::Page(pi, page::Message::InputChanged(F_DEMO_EMAIL, v))
                        })
                        .width(Fill),
                )
                .label("Email")
                .into();

                // shadcn pairs the Password label with the "Forgot your
                // password?" link in a flex row, separated by `ml-auto`.
                // Rendered as a row inside the Field's label slot.
                let password_label: Slot<'_, Message> = row![
                    text::label("Password"),
                    space::horizontal(),
                    button("Forgot your password?").style(button::Link)
                        .on_press(Message::Page(pi, page::Message::Noop)),
                ]
                .align_y(Center)
                .width(Fill)
                .into();

                let password_field: Slot<'_, Message> = field(
                    input("").value(&self.demo_password)
                        .on_input(move |v| {
                            Message::Page(pi, page::Message::InputChanged(F_DEMO_PASSWORD, v))
                        })
                        .width(Fill),
                )
                .label(password_label)
                .into();

                card(column![
                    card::header("Login to your account")
                        .description("Enter your email below to login to your account")
                        .action(card::action(
                            button("Sign Up").style(button::Link)
                                .on_press(Message::Page(pi, page::Message::Noop))
                        )),
                    card::content(
                        column![email_field, password_field]
                            .spacing(Gap::G6)
                            .width(Fill),
                    ),
                    // shadcn footer here uses `flex-col gap-2` with full-width
                    // buttons — column shape, not row.
                    card::footer(
                        column![
                            button("Login")
                                .on_press(Message::Page(pi, page::Message::Noop))
                                .width(Fill),
                            button("Login with Google").style(button::Outline)
                                .on_press(Message::Page(pi, page::Message::Noop))
                                .width(Fill),
                        ]
                        .spacing(Gap::G2)
                        .width(Fill)
                    ),
                ]
                .spacing(Gap::G4)
                .padding(iced::Padding::ZERO.top(Gap::G4).bottom(Gap::G4))
                .width(Fill))
                .width(384.0)
                .boxed()
            }

            // Small: header (title + description) + content (paragraph) +
            // footer (full-width outline `Action` button at sm size).
            // shadcn renders this with `size="sm"`; we ship the same content
            // shape at default size — see scope note at the top of this file.
            "small" => card(column![
                card::header("Small Card")
                    .description("This card uses the small size variant."),
                card::content(text::p(
                    "The card component supports a size prop that can be set to \"sm\" for a more compact appearance."
                )),
                card::footer(
                    button("Action").style(button::Outline)
                        .size(Size::Sm)
                        .on_press(Message::Page(pi, page::Message::Noop))
                        .width(Fill)
                ),
            ]
            .spacing(Gap::G4)
            .padding(iced::Padding::ZERO.top(Gap::G4).bottom(Gap::G4))
            .width(Fill))
            .width(384.0)
            .boxed(),

            // Image: image (16:9) with a 35% black tint overlay above the
            // header (Featured badge action + title + description) + footer
            // (full-width "View Event" button). max-w-sm = 384px.
            "image" => {
                let header: Slot<'_, Message> = card::header("Design systems meetup")
                    .description(
                        "A practical talk on component APIs, accessibility, and shipping faster.",
                    )
                    .action(card::action(badge("Featured").style(badge::Secondary)))
                    .into();

                card(column![
                    aspect_ratio(stack![image_placeholder(), image_overlay()], 16.0 / 9.0),
                    header,
                    card::footer(
                        button("View Event")
                            .on_press(Message::Page(pi, page::Message::Noop))
                            .width(Fill)
                    ),
                ]
                .spacing(Gap::G4)
                .padding(iced::Padding::ZERO.bottom(Gap::G4))
                .width(Fill))
                .width(384.0)
                .clip(true)
                .boxed()
            }

            _ => unreachable!("unknown card preview id: {id}"),
        }
    }
}
