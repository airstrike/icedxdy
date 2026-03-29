```rust
use icedxdy::widget::{input, text};

column![
    text::small("Email"),
    input::input("Email", &self.email)
        .on_input(Message::EmailChanged),
]
.spacing(8)
```
