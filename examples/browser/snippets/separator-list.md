```rust
use icedxdy::widget::{column, row, separator, space, text};

fn list_item<'a>(term: &'a str, value: &'a str) -> Slot<'a, Message> {
    row![text::small(term), space::horizontal(), text::muted(value)]
        .width(Fill)
        .into()
}

column![
    list_item("Item 1", "Value 1"),
    separator(),
    list_item("Item 2", "Value 2"),
    separator(),
    list_item("Item 3", "Value 3"),
]
.spacing(8)
.max_width(384.0)
```
