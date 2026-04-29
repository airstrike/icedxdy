```rust
use icedxdy::widget::{input, text};

// Omitting `.on_input(...)` disables the input — the iced convention.
column![
    text::small("Email").semibold(),
    input("Email", ""),
    text::muted("This field is currently disabled."),
]
```
