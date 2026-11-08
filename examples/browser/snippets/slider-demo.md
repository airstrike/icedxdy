```rust
use icedxdy::widget::{container, slider};

container(slider(0.0..=100.0).default_value(75.0).step(1.0))
    .width(Fill)
    .max_width(320)
```
