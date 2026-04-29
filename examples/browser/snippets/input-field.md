```rust
use icedxdy::widget::{input, text};

column![
    text::small("Username").semibold(),
    input("Enter your username", &self.value)
        .on_input(Message::Changed),
    text::muted("Choose a unique username for your account."),
]
```
