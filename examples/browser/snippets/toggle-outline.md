```rust
use icedxdy::widget::{row, toggle};

row![
    toggle![italic, "Italic"].style(toggle::Outline),
    toggle![bold, "Bold"].style(toggle::Outline),
]
.spacing(8)
```
