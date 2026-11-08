```rust
use icedxdy::widget::{column, skeleton};

column![
    column![skeleton().width(80).height(16), skeleton().width(Fill).height(32)].spacing(12),
    column![skeleton().width(96).height(16), skeleton().width(Fill).height(32)].spacing(12),
    skeleton().width(96).height(32),
]
.spacing(28)
.max_width(320.0)
```
