```rust
use icedxdy::widget::{column, row, skeleton};

row![
    skeleton().width(48).height(48).radius(u32::MAX as f32),
    column![
        skeleton().width(250).height(16),
        skeleton().width(200).height(16),
    ]
    .spacing(8),
]
.spacing(16)
.align_y(Center)
```
