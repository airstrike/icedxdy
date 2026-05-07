```rust
use icedxdy::widget::{checkbox, row, text};

// No `.on_toggle(...)` ⇒ disabled (matches iced's input convention).
row![
    checkbox(false),
    text("Enable notifications"),
]
.spacing(8)
```
