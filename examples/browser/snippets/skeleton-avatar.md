```rust
use icedxdy::widget::{column, row, skeleton};

row![
    skeleton().width(40).height(40).radius(u32::MAX as f32),
    column![
        skeleton().width(150).height(16),
        skeleton().width(100).height(16),
    ]
    .spacing(8),
]
.spacing(16)
.align_y(Center)
```
