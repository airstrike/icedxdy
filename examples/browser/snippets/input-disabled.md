```rust
use icedxdy::widget::{field, input};

field(input("Email").width(Fill).disabled(true))
    .label("Email")
    .description("This field is currently disabled.")
    .width(320)
```
