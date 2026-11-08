```rust
use icedxdy::Horizontal;
use icedxdy::widget::{column, field, radio_group};

radio_group(|g| {
    column![
        field(g.item("default"))
            .orientation(Horizontal)
            .label("Default")
            .description("Standard spacing for most use cases."),
        field(g.item("comfortable"))
            .orientation(Horizontal)
            .label("Comfortable")
            .description("More space between elements."),
        field(g.item("compact"))
            .orientation(Horizontal)
            .label("Compact")
            .description("Minimal spacing for dense layouts."),
    ]
    .spacing(8)
    .into()
})
.default_value("comfortable")
```
