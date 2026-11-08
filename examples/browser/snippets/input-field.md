```rust
use icedxdy::widget::{field, input};

field(
    input("Enter your username")
        .width(Fill),
)
.label("Username")
.description("Choose a unique username for your account.")
.width(320)
```
