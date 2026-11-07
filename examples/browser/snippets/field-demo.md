```rust
use icedxdy::widget::{field, input};

field(
    input("1234 5678 9012 3456").value(&self.value)
        .on_input(Message::Changed)
        .width(Fill),
)
.label("Card Number")
.description("Enter your 16-digit card number")
```
