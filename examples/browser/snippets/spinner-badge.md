```rust
use icedxdy::widget::{badge, row, spinner};

row![
    badge![spinner().size(12.0), "Syncing"],
    badge![spinner().size(12.0), "Updating"].style(badge::Secondary),
    badge![spinner().size(12.0), "Processing"].style(badge::Outline),
]
.spacing(16)
.align_y(Center)
```
