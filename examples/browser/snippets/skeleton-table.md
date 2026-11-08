```rust
use icedxdy::widget::{Column, row, skeleton};

Column::with_children((0..5).map(|_| {
    row![
        skeleton().width(Fill).height(16),
        skeleton().width(96).height(16),
        skeleton().width(80).height(16),
    ]
    .spacing(16)
    .into()
}))
.spacing(8)
.max_width(384.0)
```
