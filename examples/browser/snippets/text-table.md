```rust
use iced::Length::Fill;
use icedxdy::widget::text;
use sweeten::widget::table;

// Sweeten's enhanced `Table` ships outer border + sticky-header + header
// underline on top of iced's stock data-grid table. Two columns, three
// rows of static data; cells are produced by per-column view closures.
let rows: &[(&'static str, &'static str)] = &[
    ("Empty", "Overflowing"),
    ("Modest", "Satisfied"),
    ("Full", "Ecstatic"),
];

let header = |label: &'static str| text::p(label).bold().into();

let columns = [
    table::column(Some(header("King's Treasury")), |row: &(&str, &str)| {
        text::p(row.0)
    })
    .width(Fill),
    table::column(Some(header("People's happiness")), |row: &(&str, &str)| {
        text::p(row.1)
    })
    .width(Fill),
];

table::table(columns, rows)
    .width(Fill)
    .padding_x(16)
    .padding_y(8)
    .separator(1)
    .border(1)
```
