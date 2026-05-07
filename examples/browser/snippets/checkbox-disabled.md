```rust
use icedxdy::widget::{checkbox, column, field, text};

column![
    field::field(checkbox(false))
        .orientation(field::Orientation::Horizontal)
        .label(text::muted("Enable notifications")),
]
.width(224)
```
