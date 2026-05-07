```rust
use icedxdy::widget::{column, input, text};

column![
    text::label("Your email"),
    input("m@example.com", &self.value)
        .on_input(Message::Changed)
        .width(Fill),
]
.spacing(8)
.width(320)
```
