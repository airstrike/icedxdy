```rust
use icedxdy::widget::{table, text};

let children: [Slot<'_, Message>; 4] = [
    header.into(),
    table::body(INVOICES.iter().take(3).map(invoice_row)).into(),
    table::footer([table::row([
        table::cell(text::small("Total").medium()).width(Fill),
        table::cell(text::small("$2,500.00").medium()).width(100).align_x(Right),
    ])])
    .into(),
    table::caption("A list of your recent invoices.").into(),
];

table::table(children).width(Fill)
```
