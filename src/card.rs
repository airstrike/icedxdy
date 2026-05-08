//! Card components per shadcn nova.
//!
//! Container component with header / content / footer slots and three
//! header-internal slots: `title`, `description`, and `action`. The header
//! is a builder — title is required (and auto-styled from a string),
//! description and action are independently optional via fluent methods.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::card;
//! use icedxdy::widget::{button, column};
//!
//! card(column![
//!     card::header("Login to your account")
//!         .description("Enter your email below to login to your account")
//!         .action(card::action(button::link("Sign Up").on_press(Message::SignUp))),
//!     card::content(my_form),
//!     card::footer_row([
//!         button("Login").on_press(Message::Submit).into(),
//!         button::outline("Login with Google").into(),
//!     ]),
//! ])
//! .width(384.0)
//! ```

use iced_core::Length;
use iced_core::text::IntoFragment;
use iced_widget::Column;
use iced_widget::Renderer;
use iced_widget::Row;
use iced_widget::container::Container;
use iced_widget::text::LineHeight;

use crate::Gap;
use crate::Theme;
use crate::theme;
use crate::widget::Component;
use crate::widget::space;
use crate::widget::text;

/// Padding values based on shadcn nova card.
mod padding {
    /// Section horizontal padding (`px-4` = 16px on header / content; the
    /// footer wraps this with full `p-4`).
    pub const SECTION: f32 = 16.0;
}

/// Card root — wraps content with shadcn's card surface (bg-card + ring +
/// rounded corners).
pub fn card<'a, Message>(
    content: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer> {
    Container::new(content).style(|t| theme::container::card(t).into())
}

/// A `<CardTitle>`-shaped Text — `text-base font-medium leading-snug`,
/// foreground color. Use directly when composing a header by hand;
/// [`header`] applies this to its first argument automatically when given
/// a string.
pub fn title<'a>(content: impl IntoFragment<'a>) -> text::Text<'a> {
    text::Text::new(content)
        .size(text::size::BASE)
        .medium()
        .line_height(LineHeight::Relative(1.375))
        .style(theme::text::Foreground)
}

/// A `<CardDescription>`-shaped Text — `text-sm text-muted-foreground`.
/// Equivalent to [`text::muted`]; exposed under `card::` so the slot
/// vocabulary mirrors shadcn's API surface (`<CardTitle>` /
/// `<CardDescription>` / `<CardAction>` round-trip cleanly).
pub fn description<'a>(content: impl IntoFragment<'a>) -> text::Text<'a> {
    text::muted(content)
}

/// A `<CardAction>`-shaped passthrough. shadcn positions the action via the
/// CSS-grid `col-start-2 row-span-2 row-start-1 self-start justify-self-end`
/// — top-right corner of the header. Iced doesn't have a CSS-grid analog,
/// so the [`Header`] builder achieves the same layout by pushing the action
/// flush-right with [`space::horizontal`].
///
/// This constructor is an identity — it documents the slot, lets call sites
/// read symmetrically (`card::action(...)` next to `card::title(...)` /
/// `card::description(...)`), and reserves a hook for any future
/// action-specific styling.
pub fn action<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Component<'a, Message> {
    content.into()
}

/// Builder for a card header.
///
/// Returned by [`header`]. Title is supplied at construction; description
/// and action are independently optional via [`Header::description`] and
/// [`Header::action`]. Renders as a column (or row, when an action is
/// present) with `px-4` horizontal padding and `gap-1` (4px) between title
/// and description, matching nova's `CardHeader` cascade.
pub struct Header<'a, Message> {
    title: Component<'a, Message>,
    description: Option<Component<'a, Message>>,
    action: Option<Component<'a, Message>>,
}

/// Build a header from a title.
///
/// Plain strings auto-style as [`title`]. Pre-styled `Text` or `Component`
/// values pass through unchanged — useful when the title needs custom
/// styling (rich text, icon, etc.).
///
/// Add a description and/or an action via the builder methods:
/// ```ignore
/// card::header("Login to your account")
///     .description("Enter your email below to login to your account")
///     .action(card::action(button::link("Sign Up").on_press(Message::SignUp)));
/// ```
pub fn header<'a, Message: 'a>(title: impl IntoTitle<'a, Message>) -> Header<'a, Message> {
    Header {
        title: title.into_title(),
        description: None,
        action: None,
    }
}

impl<'a, Message: 'a> Header<'a, Message> {
    /// Set the description slot. Plain strings auto-style as [`description`]
    /// (`text-sm text-muted-foreground`); pre-styled values pass through.
    pub fn description(mut self, description: impl IntoDescription<'a, Message>) -> Self {
        self.description = Some(description.into_description());
        self
    }

    /// Set the action slot — typically a top-right link, button, or badge.
    /// shadcn pins the action via `col-start-2 row-span-2 self-start
    /// justify-self-end`; the rendered Header pushes it flush-right with
    /// [`space::horizontal`] inside a row.
    pub fn action(mut self, action: impl Into<Component<'a, Message>>) -> Self {
        self.action = Some(action.into());
        self
    }
}

impl<'a, Message: 'a> From<Header<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(h: Header<'a, Message>) -> Self {
        let Header {
            title,
            description,
            action,
        } = h;

        // Stack title above (optional) description in a column with gap-1.
        let mut text_col: Column<'a, Message, Theme, Renderer> =
            Column::new().spacing(Gap::G1).width(Length::Fill);
        text_col = text_col.push(title.into_iced());
        if let Some(d) = description {
            text_col = text_col.push(d.into_iced());
        }

        // When an action is present, place it at the end of a row so it sits
        // flush against the right edge of the inner padded rect — the iced
        // analog of shadcn's `col-start-2 ... justify-self-end` grid pin.
        let padded_inner: iced_core::Element<'a, Message, Theme, Renderer> = match action {
            Some(a) => Row::new()
                .push(text_col)
                .push(space::horizontal())
                .push(a.into_iced())
                .align_y(iced_core::Alignment::Start)
                .width(Length::Fill)
                .into(),
            None => text_col.into(),
        };

        iced_widget::container(padded_inner)
            .padding(
                iced_core::Padding::ZERO
                    .left(padding::SECTION)
                    .right(padding::SECTION),
            )
            .width(Length::Fill)
            .into()
    }
}

impl<'a, Message: 'a> From<Header<'a, Message>> for Component<'a, Message> {
    fn from(h: Header<'a, Message>) -> Self {
        let element: iced_core::Element<'a, Message, Theme, Renderer> = h.into();
        Component::from(element)
    }
}

/// Convert a value into a card title. Plain strings auto-style as [`title`];
/// pre-styled `Text` and `Component` values pass through. Mirrors
/// `field::IntoLabel` — name lives under the `card` module so it's read as
/// `card::IntoTitle` (no composite name).
pub trait IntoTitle<'a, Message> {
    fn into_title(self) -> Component<'a, Message>;
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for &'a str {
    fn into_title(self) -> Component<'a, Message> {
        title(self).into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for String {
    fn into_title(self) -> Component<'a, Message> {
        title(self).into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for text::Text<'a> {
    fn into_title(self) -> Component<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoTitle<'a, Message> for Component<'a, Message> {
    fn into_title(self) -> Component<'a, Message> {
        self
    }
}

/// Convert a value into a card description. Plain strings auto-style as
/// [`description`]; pre-styled values pass through.
pub trait IntoDescription<'a, Message> {
    fn into_description(self) -> Component<'a, Message>;
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for &'a str {
    fn into_description(self) -> Component<'a, Message> {
        description(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for String {
    fn into_description(self) -> Component<'a, Message> {
        description(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for text::Text<'a> {
    fn into_description(self) -> Component<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for Component<'a, Message> {
    fn into_description(self) -> Component<'a, Message> {
        self
    }
}

/// Card header with title only (no description, no action). Convenience for
/// the most common shape.
///
/// Equivalent to `card::header(title)` — both flow through the same
/// rendering path. Kept as a named export so call sites that don't need
/// the builder shape stay terse.
pub fn header_simple<'a, Message: 'a>(
    title: impl IntoTitle<'a, Message>,
) -> Header<'a, Message> {
    header(title)
}

/// Card content section (`px-4` only).
pub fn content<'a, Message: 'a>(
    child: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer> {
    Container::new(child).padding(
        iced_core::Padding::ZERO
            .left(padding::SECTION)
            .right(padding::SECTION),
    )
}

/// Card footer section. Caller composes the inner shape (row of buttons,
/// stacked column, etc.); `footer` applies the `px-4` horizontal padding.
pub fn footer<'a, Message: 'a>(
    child: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer> {
    Container::new(child).padding(
        iced_core::Padding::ZERO
            .left(padding::SECTION)
            .right(padding::SECTION),
    )
}

/// Card footer as a row of children with 8px (`gap-2`) spacing.
pub fn footer_row<'a, Message: 'a>(
    children: impl IntoIterator<Item = iced_core::Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer> {
    let row = Row::with_children(children).spacing(Gap::G2);
    footer(row)
}
