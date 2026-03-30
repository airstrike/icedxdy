```rust
use icedxdy::widget::{badge, spinner};

badge(row![spinner(), "Deleting"])
    .variant(badge::Variant::Destructive)

badge(row!["Generating", spinner()])
    .variant(badge::Variant::Secondary)
```
