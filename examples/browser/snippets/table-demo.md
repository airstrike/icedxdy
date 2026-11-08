```rust
use icedxdy::widget::{table, text};

let children: [Slot<'_, Message>; 4] = [
    table::header([table::row([
        table::head("Invoice").width(100),
        table::head("Status").width(Fill),
        table::head("Method").width(Fill),
        table::head("Amount").width(100).align_x(Right),
    ])])
    .into(),
    table::body(INVOICES.iter().map(|[invoice, status, method, amount]| {
        table::row([
            table::cell(text::small(*invoice).medium()).width(100),
            table::cell(text::small(*status)).width(Fill),
            table::cell(text::small(*method)).width(Fill),
            table::cell(text::small(*amount)).width(100).align_x(Right),
        ])
    }))
    .into(),
    table::footer([table::row([
        table::cell(text::small("Total").medium()).width(Fill),
        table::cell(text::small("$2,500.00").medium()).width(100).align_x(Right),
    ])])
    .into(),
    table::caption("A list of your recent invoices.").into(),
];

table::table(children).width(Fill)
```
