```rust
use icedxdy::Horizontal;
use icedxdy::widget::{column, field, radio_group};

radio_group(|g| {
    column![
        field(g.item("default")).orientation(Horizontal).label("Default"),
        field(g.item("comfortable")).orientation(Horizontal).label("Comfortable"),
        field(g.item("compact")).orientation(Horizontal).label("Compact"),
    ]
    .spacing(8)
    .into()
})
.default_value("comfortable")
```
