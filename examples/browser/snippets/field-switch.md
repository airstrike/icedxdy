```rust
use icedxdy::widget::{field, switch};

field(switch(self.is_on).on_toggle(Message::Toggled))
    .label("Multi-factor authentication")
    .description("Require a second verification step at sign-in.")
```
