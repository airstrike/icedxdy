//! InputGroup - shadcn/ui faithful port.
//!
//! An Input (or a Textarea) and its addons inside one bordered field. The
//! group paints the Input's surface and focus ring around everything, and
//! strips the control's own:
//!
//! ```ignore
//! use icedxdy::widget::{input, input_group};
//!
//! input_group(input("Search..."))
//!     .addon(input_group::addon(icon::search()))
//!     .addon(input_group::addon("12 results").align(input_group::Align::InlineEnd))
//! ```
//!
//! # Variants
//!
//! An invalid control is a style on the group: `.style(input::Invalid)`
//! paints the Input's `aria-invalid` surface.

mod widget;

use iced_core::Length::Fill;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Alignment, Element, Padding};
use iced_widget::{Renderer, Widget};
use iced_widget::{container, text_editor};
use icedxdy_core::Theme;
use icedxdy_core::theme::input_group::{control, textarea};
use icedxdy_core::theme::text_input::{Default, StyleFn};

use crate::button::{self, Button};
use crate::column::Column;
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::input::Input;
use crate::row::Row;
use crate::text::{self, Text};
use crate::textarea::Textarea;
use crate::{Size, Weight};

/// The field an [`InputGroup`] is built around.
pub enum Control<'a, Message> {
    /// An `InputGroupInput`.
    Input(Input<'a, Message>),
    /// An `InputGroupTextarea`.
    Textarea(Textarea<'a, Message>),
}

impl<'a, Message> From<Input<'a, Message>> for Control<'a, Message> {
    fn from(input: Input<'a, Message>) -> Self {
        Self::Input(input)
    }
}

impl<'a, Message> From<Textarea<'a, Message>> for Control<'a, Message> {
    fn from(textarea: Textarea<'a, Message>) -> Self {
        Self::Textarea(textarea)
    }
}

/// Where an [`Addon`] sits (shadcn's `align`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::default::Default)]
pub enum Align {
    /// Before the control, on its line.
    #[default]
    InlineStart,
    /// After the control, on its line.
    InlineEnd,
    /// Above the control, full width.
    BlockStart,
    /// Below the control, full width.
    BlockEnd,
}

/// Content beside or around the control (`InputGroupAddon`): muted
/// `text-sm font-medium`, icons at 16px.
pub struct Addon<'a, Message> {
    content: Slot<'a, Message>,
    align: Align,
}

/// Create an [`Addon`] before the control.
pub fn addon<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Addon<'a, Message> {
    Addon {
        content: content.into(),
        align: Align::InlineStart,
    }
}

impl<'a, Message: 'a> Addon<'a, Message> {
    /// Set where the addon sits.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
}

/// Muted `text-sm` text for an addon (`InputGroupText`).
pub fn text<'a>(content: impl iced_core::text::IntoFragment<'a>) -> Text<'a> {
    text::muted(content)
}

/// A compact ghost button for an addon (`InputGroupButton`, `size="xs"`).
/// Chain `.square()` for `size="icon-xs"`.
pub fn button<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Button<'a, Message> {
    button::button(content).size(Size::Xs).style(button::Ghost)
}

/// A control and its addons in one bordered field.
pub struct InputGroup<'a, Message> {
    control: Control<'a, Message>,
    addons: Vec<Addon<'a, Message>>,
    disabled: bool,
    style: StyleFn<'a, Theme>,
}

/// Create an [`InputGroup`] around `control`, an Input or a Textarea.
pub fn input_group<'a, Message>(
    control: impl Into<Control<'a, Message>>,
) -> InputGroup<'a, Message> {
    InputGroup {
        control: control.into(),
        addons: Vec::new(),
        disabled: false,
        style: Default.into(),
    }
}

impl<'a, Message: 'a> InputGroup<'a, Message> {
    /// Add an addon; addons sharing an alignment keep their order.
    pub fn addon(mut self, addon: Addon<'a, Message>) -> Self {
        self.addons.push(addon);
        self
    }

    /// Disable the control and dim the group.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the surface style: `input::Invalid`, or any Input style.
    pub fn style(mut self, style: impl Into<StyleFn<'a, Theme>>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for InputGroup<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let InputGroup {
            control: field,
            addons,
            disabled,
            style,
        } = *self;

        let is_textarea = matches!(field, Control::Textarea(_));
        let mut field: Slot<'a, Message> = match field {
            Control::Input(input) => input
                .width(Fill)
                .style(Box::new(control) as StyleFn<'a, Theme>)
                .into(),
            Control::Textarea(area) => area
                .style(Box::new(textarea) as text_editor::StyleFn<'a, Theme>)
                .into(),
        };
        field.inherit(&Inherited::default().with_disabled(disabled));

        let (mut start, mut end, mut above, mut below) = (vec![], vec![], vec![], vec![]);

        for Addon { mut content, align } in addons {
            // `gap-2 py-1.5 text-sm font-medium text-muted-foreground`,
            // icons at `size-4`; dimmed with a disabled group.
            content.inherit(
                &Inherited::default()
                    .with_size(Size::Default)
                    .with_weight(Weight::Medium)
                    .with_disabled(disabled),
            );

            let padding = match align {
                Align::InlineStart => Padding::ZERO.left(8),
                Align::InlineEnd => Padding::ZERO.right(8),
                Align::BlockStart => Padding::ZERO.top(8).left(10).right(10),
                Align::BlockEnd => Padding::ZERO.bottom(8).left(10).right(10),
            };

            let addon = Container::new(content)
                .padding(padding)
                .align_y(Vertical::Center)
                .class(Box::new(|theme: &Theme| {
                    container::Style::default().color(theme.muted.foreground)
                }) as container::StyleFn<'static, Theme>);

            match align {
                Align::InlineStart => start.push(Slot::from(addon)),
                Align::InlineEnd => end.push(Slot::from(addon)),
                Align::BlockStart => above.push(Slot::from(addon.width(Fill))),
                Align::BlockEnd => below.push(Slot::from(addon.width(Fill))),
            }
        }

        let line: Slot<'a, Message> = if start.is_empty() && end.is_empty() {
            field
        } else {
            Row::with_children(start.into_iter().chain([field]).chain(end))
                .align_y(if is_textarea {
                    Alignment::Start
                } else {
                    Alignment::Center
                })
                .width(Fill)
                .into()
        };

        let content: Element<'a, Message, Theme, Renderer> = if above.is_empty() && below.is_empty()
        {
            line.compose()
        } else {
            Column::with_children(above.into_iter().chain([line]).chain(below))
                .width(Fill)
                .align_x(Horizontal::Left)
                .into()
        };

        widget::Surface::new(content, style)
            .disabled(disabled)
            .boxed()
    }
}

impl<'a, Message: Clone + 'a> From<InputGroup<'a, Message>> for Slot<'a, Message> {
    fn from(group: InputGroup<'a, Message>) -> Self {
        Slot::new(group)
    }
}

impl<'a, Message: Clone + 'a> From<InputGroup<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(group: InputGroup<'a, Message>) -> Self {
        Box::new(group).compose()
    }
}
