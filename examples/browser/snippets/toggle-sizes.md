```rust
use icedxdy::widget::{row, toggle};

row![
    toggle("Small").size(Size::Sm).style(toggle::Outline),
    toggle("Default").style(toggle::Outline),
    toggle("Large").size(Size::Lg).style(toggle::Outline),
]
.spacing(8)
```
