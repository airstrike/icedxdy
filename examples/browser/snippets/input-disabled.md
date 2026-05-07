```rust
use icedxdy::widget::{column, field, input};

// Omitting `.on_input(...)` disables the input — the iced convention.
column![
    field::field(input("Email", "").width(Fill))
        .label("Email")
        .description("This field is currently disabled."),
]
.width(320)
```
