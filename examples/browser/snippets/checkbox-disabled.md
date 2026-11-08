```rust
use icedxdy::widget::{checkbox, field, text};

field(checkbox().disabled(true))
    .orientation(field::Horizontal)
    .label(text::muted("Enable notifications"))
    .width(224)
```
