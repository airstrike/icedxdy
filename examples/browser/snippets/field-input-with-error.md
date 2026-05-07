```rust
use icedxdy::theme;
use icedxdy::widget::{field, input};

field(
    input("you@example.com", &self.value)
        .on_input(Message::Changed)
        .width(Fill)
        .style(theme::text_input::Invalid),
)
.label("Email")
.error("Please enter a valid email address.")
```
