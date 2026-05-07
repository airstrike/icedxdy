```rust
use icedxdy::widget::{input, text};

// Omitting `.on_input(...)` disables the input — the iced convention.
column![
    text::label("Email"),
    input("Email", "").width(Fill),
    text::muted("This field is currently disabled."),
]
.spacing(8)
.width(320)
```
