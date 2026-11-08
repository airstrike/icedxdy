```rust
use icedxdy::widget::{container, slider};

container(slider(0.0..=100.0).default_value(50.0).step(1.0).disabled(true))
    .width(Fill)
    .max_width(320)
```
