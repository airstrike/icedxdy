//! Table component slots.
//!
//! Mirrors shadcn's `Table` primitive shape while keeping icedxdy children
//! as arbitrary [`Component`] values.

use iced_core::Length::{Fill, Shrink};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Length, Padding};
use iced_widget::Renderer;
use icedxdy_core::theme::container::Style;
use icedxdy_core::{Border, Theme};

use crate::component::{Component, IntoElement};
use crate::{Size, Weight, column, container, rule};

/// Root table slot.
pub struct Table<'a, Message> {
    children: Vec<Component<'a, Message>>,
    width: Length,
}

/// Header row group.
pub struct Header<'a, Message> {
    rows: Vec<Row<'a, Message>>,
}

/// Body row group.
pub struct Body<'a, Message> {
    rows: Vec<Row<'a, Message>>,
}

/// Footer row group.
pub struct Footer<'a, Message> {
    rows: Vec<Row<'a, Message>>,
}

/// A table row.
pub struct Row<'a, Message> {
    cells: Vec<Cell<'a, Message>>,
    border: bool,
}

/// A table cell.
pub struct Cell<'a, Message> {
    content: Component<'a, Message>,
    kind: cell::Kind,
    width: Length,
    height: Length,
    padding: Padding,
    align_x: Horizontal,
    align_y: Vertical,
}

/// Caption slot.
pub struct Caption<'a, Message> {
    content: Component<'a, Message>,
}

/// Cell internals.
pub mod cell {
    /// Cell kind.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Kind {
        /// Body cell.
        Body,
        /// Header cell.
        Head,
    }
}

/// Create a [`Table`] from table child slots.
pub fn table<'a, Message: 'a, C>(children: impl IntoIterator<Item = C>) -> Table<'a, Message>
where
    C: Into<Component<'a, Message>>,
{
    Table::new(children)
}

/// Create a [`Header`] from rows.
pub fn header<'a, Message: 'a>(
    rows: impl IntoIterator<Item = Row<'a, Message>>,
) -> Header<'a, Message> {
    Header {
        rows: rows.into_iter().collect(),
    }
}

/// Create a [`Body`] from rows.
pub fn body<'a, Message: 'a>(
    rows: impl IntoIterator<Item = Row<'a, Message>>,
) -> Body<'a, Message> {
    Body {
        rows: rows.into_iter().collect(),
    }
}

/// Create a [`Footer`] from rows.
pub fn footer<'a, Message: 'a>(
    rows: impl IntoIterator<Item = Row<'a, Message>>,
) -> Footer<'a, Message> {
    Footer {
        rows: rows.into_iter().collect(),
    }
}

/// Create a [`Row`] from cells.
pub fn row<'a, Message: 'a>(
    cells: impl IntoIterator<Item = Cell<'a, Message>>,
) -> Row<'a, Message> {
    Row {
        cells: cells.into_iter().collect(),
        border: true,
    }
}

/// Create a body [`Cell`].
pub fn cell<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Cell<'a, Message> {
    Cell {
        content: content.into(),
        kind: cell::Kind::Body,
        width: Shrink,
        height: Shrink,
        padding: Padding::from(8.0),
        align_x: Horizontal::Left,
        align_y: Vertical::Center,
    }
}

/// Create a header [`Cell`].
pub fn head<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Cell<'a, Message> {
    cell(content).kind(cell::Kind::Head).height(40.0)
}

/// Create a [`Caption`].
pub fn caption<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Caption<'a, Message> {
    Caption {
        content: content.into(),
    }
}

impl<'a, Message: 'a> Table<'a, Message> {
    /// Create a table from table child slots.
    pub fn new<C>(children: impl IntoIterator<Item = C>) -> Self
    where
        C: Into<Component<'a, Message>>,
    {
        Self {
            children: children.into_iter().map(Into::into).collect(),
            width: Fill,
        }
    }

    /// Set the table width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: 'a> Row<'a, Message> {
    /// Set whether the row draws a bottom separator.
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }
}

impl<'a, Message: 'a> Cell<'a, Message> {
    /// Set the cell width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the cell height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set the cell padding.
    pub fn padding(mut self, padding: impl Into<icedxdy_core::Padding>) -> Self {
        let padding: icedxdy_core::Padding = padding.into();
        self.padding = padding.into();
        self
    }

    /// Set the horizontal alignment of the cell content.
    pub fn align_x(mut self, alignment: impl Into<Horizontal>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Set the vertical alignment of the cell content.
    pub fn align_y(mut self, alignment: impl Into<Vertical>) -> Self {
        self.align_y = alignment.into();
        self
    }

    fn kind(mut self, kind: cell::Kind) -> Self {
        self.kind = kind;
        self
    }
}

impl<'a, Message: 'a> From<Table<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(table: Table<'a, Message>) -> Self {
        Box::new(table).into_element()
    }
}

impl<'a, Message: 'a> From<Table<'a, Message>> for Component<'a, Message> {
    fn from(table: Table<'a, Message>) -> Self {
        Component::new(table)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Table<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Table { children, width } = *self;

        column(children).spacing(0).width(width).into()
    }
}

impl<'a, Message: 'a> From<Header<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(header: Header<'a, Message>) -> Self {
        Box::new(header).into_element()
    }
}

impl<'a, Message: 'a> From<Header<'a, Message>> for Component<'a, Message> {
    fn from(header: Header<'a, Message>) -> Self {
        Component::new(header)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Header<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Header { rows } = *self;
        let rows = rows.into_iter().map(Component::from);

        column(rows).spacing(0).width(Fill).into()
    }
}

impl<'a, Message: 'a> From<Body<'a, Message>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(body: Body<'a, Message>) -> Self {
        Box::new(body).into_element()
    }
}

impl<'a, Message: 'a> From<Body<'a, Message>> for Component<'a, Message> {
    fn from(body: Body<'a, Message>) -> Self {
        Component::new(body)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Body<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Body { mut rows } = *self;

        if let Some(last) = rows.last_mut() {
            last.border = false;
        }

        let rows = rows.into_iter().map(Component::from);

        column(rows).spacing(0).width(Fill).into()
    }
}

impl<'a, Message: 'a> From<Footer<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(footer: Footer<'a, Message>) -> Self {
        Box::new(footer).into_element()
    }
}

impl<'a, Message: 'a> From<Footer<'a, Message>> for Component<'a, Message> {
    fn from(footer: Footer<'a, Message>) -> Self {
        Component::new(footer)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Footer<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Footer { mut rows } = *self;

        if let Some(last) = rows.last_mut() {
            last.border = false;
        }

        let rows = rows.into_iter().map(Component::from);
        let content = column(rows).spacing(0).width(Fill);

        column([
            separator(),
            Component::from(container(content).width(Fill).style(footer_style)),
        ])
        .spacing(0)
        .width(Fill)
        .into()
    }
}

impl<'a, Message: 'a> From<Row<'a, Message>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(row: Row<'a, Message>) -> Self {
        Box::new(row).into_element()
    }
}

impl<'a, Message: 'a> From<Row<'a, Message>> for Component<'a, Message> {
    fn from(row: Row<'a, Message>) -> Self {
        Component::new(row)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Row<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Row { cells, border } = *self;
        let cells = cells.into_iter().map(Component::from);
        let row = crate::Row::with_children(cells)
            .align_y(iced_core::Alignment::Center)
            .width(Fill);

        if border {
            column([Component::from(row), separator()])
                .spacing(0)
                .width(Fill)
                .into()
        } else {
            row.into()
        }
    }
}

impl<'a, Message: 'a> From<Cell<'a, Message>> for iced_core::Element<'a, Message, Theme, Renderer> {
    fn from(cell: Cell<'a, Message>) -> Self {
        Box::new(cell).into_element()
    }
}

impl<'a, Message: 'a> From<Cell<'a, Message>> for Component<'a, Message> {
    fn from(cell: Cell<'a, Message>) -> Self {
        Component::new(cell)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Cell<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Cell {
            mut content,
            kind,
            width,
            height,
            padding,
            align_x,
            align_y,
        } = *self;

        content.size(Size::Default);
        if kind == cell::Kind::Head {
            content.weight(Weight::Medium);
        }

        container(content)
            .padding(padding)
            .width(width)
            .height(height)
            .align_x(align_x)
            .align_y(align_y)
            .into()
    }
}

impl<'a, Message: 'a> From<Caption<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(caption: Caption<'a, Message>) -> Self {
        Box::new(caption).into_element()
    }
}

impl<'a, Message: 'a> From<Caption<'a, Message>> for Component<'a, Message> {
    fn from(caption: Caption<'a, Message>) -> Self {
        Component::new(caption)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Caption<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Caption { mut content } = *self;
        content.size(Size::Default);

        container(content)
            .padding(Padding::ZERO.top(16.0))
            .width(Fill)
            .align_x(Horizontal::Center)
            .style(caption_style)
            .into()
    }
}

fn separator_style() -> rule::StyleFn<'static, Theme> {
    Box::new(|theme| rule::Style {
        background: theme.neutral.border.into(),
        ..rule::Style::default()
    })
}

fn separator<'a, Message: 'a>() -> Component<'a, Message> {
    let line: iced_core::Element<'a, Message, Theme, Renderer> =
        rule::horizontal(1).style(separator_style()).into();

    Component::from(line)
}

fn footer_style(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.alpha(0.5).into()),
        border: Border::default(),
        ..Style::default()
    }
}

fn caption_style(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.muted.foreground),
        ..Style::default()
    }
}
