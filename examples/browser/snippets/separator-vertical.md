```rust
use icedxdy::Vertical;
use icedxdy::widget::{row, separator, text};

row![
    text::small("Blog"),
    separator().orientation(Vertical),
    text::small("Docs"),
    separator().orientation(Vertical),
    text::small("Source"),
]
.spacing(16)
.height(20)
.align_y(Center)
```
