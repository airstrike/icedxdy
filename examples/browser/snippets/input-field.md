```rust
use icedxdy::widget::{field, input};

field(
    input("Enter your username", &self.value)
        .on_input(Message::Changed)
        .width(Fill),
)
.label("Username")
.description("Choose a unique username for your account.")
.width(320)
```
