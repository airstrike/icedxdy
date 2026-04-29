```rust
use icedxdy::widget::{column, progress, row, space, text};

column![
    row![
        text::small("Upload progress").medium(),
        space::horizontal(),
        text::small("66%").medium(),
    ],
    progress(66.0).width(320),
]
.spacing(8)
.width(320)
```
