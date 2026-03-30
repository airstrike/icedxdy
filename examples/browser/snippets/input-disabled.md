```rust
use icedxdy::widget::{input, text};

column![
    text::small("Email").semibold(),
    input::input("Email", "").disabled(),
    text::muted("This field is currently disabled."),
]
```
