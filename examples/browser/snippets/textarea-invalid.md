```rust
use icedxdy::widget::{field, textarea};

field(textarea("Type your message here.").style(textarea::Invalid))
    .label("Message")
    .description("Please enter a valid message.")
```
