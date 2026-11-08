```rust
use icedxdy::widget::{row, spinner};

row![
    spinner().size(12.0),
    spinner().size(16.0),
    spinner().size(24.0),
    spinner().size(32.0),
]
.spacing(24)
.align_y(Center)
```
