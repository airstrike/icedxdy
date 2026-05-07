```rust
use icedxdy::widget::{checkbox, row, text};

row![
    checkbox(false),
    text::muted("Enable notifications"),
]
.spacing(8)
.width(224)
```
