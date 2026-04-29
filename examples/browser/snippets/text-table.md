```rust
use icedxdy::widget::{column, container, row, rule, text};
use iced::Length;

// iced has no native table widget; nova's `<table>` lays out as bordered
// rows of cells. We approximate with a column of `row![left, right]`
// separated by 1px horizontal rules.
let cell = |s: &str| container(text::p(s)).width(Length::Fill).padding([8, 16]);
let header_cell =
    |s: &str| container(text::p(s).bold()).width(Length::Fill).padding([8, 16]);

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
.width(Length::Fill)
```
