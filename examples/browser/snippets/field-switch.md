```rust
use icedxdy::widget::{field, switch};

field(switch().default_checked(true))
    .label("Multi-factor authentication")
    .description("Require a second verification step at sign-in.")
```
