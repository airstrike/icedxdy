```rust
use icedxdy::widget::{column, progress, row, space, text};

column![
    row![
        text::label("Upload progress"),
        space::horizontal(),
        text::label("66%"),
    ],
    progress(66.0).width(320),
]
.spacing(8)
.width(320)
```
