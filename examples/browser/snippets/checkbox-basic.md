```rust
use icedxdy::widget::{checkbox, row, text};

row![
    checkbox(self.is_checked).on_toggle(Message::Toggled),
    text("Accept terms and conditions"),
]
.spacing(8)
```
