```rust
use icedxdy::widget::{checkbox, field, text};

field(checkbox(false))
    .orientation(field::Horizontal)
    .label(text::muted("Enable notifications"))
    .width(224)
```
