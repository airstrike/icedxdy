```rust
use icedxdy::widget::{column, skeleton};

column![
    skeleton().width(Fill).height(16),
    skeleton().width(Fill).height(16),
    line(3, 4),
]
.spacing(8)
.width(Fill)
.max_width(320.0)
```
