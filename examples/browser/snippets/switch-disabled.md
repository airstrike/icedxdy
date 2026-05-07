```rust
use icedxdy::widget::{row, switch, text};

// No `.on_toggle(...)` ⇒ disabled (matches iced's input convention).
row![
    switch(false),
    text::muted("Disabled").medium(),
]
.spacing(8)
```
