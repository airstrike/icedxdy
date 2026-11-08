```rust
use icedxdy::Horizontal;
use icedxdy::widget::{column, field, radio_group, text};

radio_group(|g| {
    column![
        field(g.item("option1").disabled(true))
            .orientation(Horizontal)
            .label(text::small("Disabled")),
        field(g.item("option2")).orientation(Horizontal).label(text::small("Option 2")),
        field(g.item("option3")).orientation(Horizontal).label(text::small("Option 3")),
    ]
    .spacing(8)
    .into()
})
.default_value("option2")
```
