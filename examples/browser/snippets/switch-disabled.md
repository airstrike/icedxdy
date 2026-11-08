```rust
use icedxdy::widget::{row, switch, text};

row![
    switch().disabled(true),
    text::muted("Disabled").medium(),
]
.spacing(8)
```
