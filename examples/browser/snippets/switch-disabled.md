```rust
use icedxdy::widget::{row, switch, text};

row![
    switch(false),
    text::muted("Disabled").medium(),
]
.spacing(8)
```
