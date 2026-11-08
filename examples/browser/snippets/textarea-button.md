```rust
use icedxdy::widget::{button, column, textarea};

column![
    textarea("Type your message here."),
    button("Send message").width(Fill),
]
.spacing(8)
.width(Fill)
```
