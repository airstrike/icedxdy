```rust
use icedxdy::widget::{input, text};

column![
    text::small("Username").medium(),
    input("Enter your username", &self.value)
        .on_input(Message::Changed)
        .width(Fill),
    text::muted("Choose a unique username for your account."),
]
.spacing(8)
.width(320)
```
