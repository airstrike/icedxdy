```rust
use icedxdy::widget::{column, progress, row, space, text};

column![
    row![
        text::small("Upload progress"),
        space::horizontal(),
        text::small("66%"),
    ],
    progress(66.0).width(320),
]
.spacing(8)
.width(320)
```
