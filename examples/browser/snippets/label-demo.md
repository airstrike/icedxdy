```rust
use icedxdy::widget::{field, input};

field(
    input("m@example.com", &self.value)
        .on_input(Message::Changed)
        .width(Fill),
)
.label("Your email")
.width(320)
```
