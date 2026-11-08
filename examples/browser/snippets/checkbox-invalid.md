```rust
use icedxdy::theme;
use icedxdy::widget::{checkbox, row, text};

row![
    checkbox()
        .style(theme::checkbox::Invalid),
    text::label("Accept terms and conditions")
        .style(theme::text::Destructive),
]
.spacing(8)
.width(224)
```
