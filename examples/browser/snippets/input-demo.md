```rust
use icedxdy::widget::{input, text};

column![
    text::small("API Key").semibold(),
    input("sk-...", &self.value)
        .on_input(Message::Changed),
    text::muted("Your API key is encrypted and stored securely."),
]
```
