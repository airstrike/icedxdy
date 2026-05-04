```rust
use iced::Fill;
use icedxdy::widget::{table, text};

let rows = &[
    ("Empty", "Overflowing"),
    ("Modest", "Satisfied"),
    ("Full", "Ecstatic"),
];

let header = |label| text::p(label).bold().into();

let columns = [
    table::column(Some(header("King's Treasury")), |row: &(_, _)| text::p(row.0))
        .width(Fill),
    table::column(Some(header("People's happiness")), |row: &(_, _)| text::p(row.1))
        .width(Fill),
];

table::table(columns, rows)
    .width(Fill)
    .padding_x(16)
    .padding_y(8)
    .separator(1)
    .border(1)
```
