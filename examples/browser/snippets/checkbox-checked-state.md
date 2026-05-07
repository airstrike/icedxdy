```rust
use icedxdy::widget::{checkbox, row, text};

row![
    checkbox(self.is_checked).on_toggle(Message::Toggle),
    text::label("Accept terms and conditions"),
]
.spacing(8)
.width(224)
```
